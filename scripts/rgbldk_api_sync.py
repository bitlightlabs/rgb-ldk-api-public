#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import re
import shutil
import subprocess
import sys
import time
from dataclasses import asdict, dataclass, replace
from pathlib import Path
from tempfile import TemporaryDirectory
from urllib.parse import urlparse


ROOT = Path(__file__).resolve().parents[1]


@dataclass(frozen=True)
class SourceInfo:
    node_repo: str
    node_commit: str | None
    node_package_version: str
    api_version: str
    generated_at_utc: str


@dataclass(frozen=True)
class DtoSource:
    source_path: Path
    output_path: Path
    content: str


@dataclass(frozen=True)
class NodeSources:
    info: SourceInfo
    dtos: tuple[DtoSource, ...]


@dataclass(frozen=True)
class OpenApiDocument:
    content: str
    data: dict


@dataclass(frozen=True)
class Artifact:
    path: Path
    content: str


class ReadNodeSourcesPass:
    """Read provenance and DTO sources without changing the node repository."""

    def __init__(self, node_repo: Path):
        self._node_repo = node_repo

    def run(self) -> NodeSources:
        cargo_toml = self._read_toml(self._node_repo / "Cargo.toml")
        package = cargo_toml.get("package")
        if not isinstance(package, dict) or "version" not in package:
            raise RuntimeError(f"missing [package].version in {self._node_repo / 'Cargo.toml'}")
        reference = self._git_origin_url(self._node_repo) or str(self._node_repo)
        info = SourceInfo(
            node_repo=self._sanitize_repo_reference(reference),
            node_commit=self._git_head(self._node_repo),
            node_package_version=str(package["version"]),
            api_version="v1",
            generated_at_utc=time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()),
        )
        source = self._locate_source()
        if source.is_dir():
            paths = sorted(source.rglob("*.rs"))
        else:
            paths = [source]
        dtos = []
        for path in paths:
            if source.is_dir():
                output = Path("src/dto") / path.relative_to(source)
            else:
                output = Path("src/dto.rs")
            dtos.append(DtoSource(
                source_path=path.relative_to(self._node_repo),
                output_path=output,
                content=path.read_text(encoding="utf-8"),
            ))
        return NodeSources(info, tuple(dtos))

    def _locate_source(self) -> Path:
        for relative in ("crates/node-http/src/dto", "src/http/dto.rs", "src/http/dto"):
            source = self._node_repo / relative
            if source.exists():
                return source
        raise RuntimeError(f"missing node DTO source in {self._node_repo}")

    @staticmethod
    def _read_toml(path: Path) -> dict:
        try:
            import tomllib
        except ModuleNotFoundError:
            raise RuntimeError("python >= 3.11 is required (missing tomllib)") from None

        return tomllib.loads(path.read_text(encoding="utf-8"))

    @staticmethod
    def _git_head(repo: Path) -> str | None:
        try:
            out = subprocess.check_output(["git", "-C", str(repo), "rev-parse", "HEAD"], stderr=subprocess.DEVNULL)
            text = out.decode("utf-8").strip()
            return text or None
        except Exception:
            return None

    @staticmethod
    def _git_origin_url(repo: Path) -> str | None:
        try:
            out = subprocess.check_output(
                ["git", "-C", str(repo), "remote", "get-url", "origin"], stderr=subprocess.DEVNULL
            )
            text = out.decode("utf-8").strip()
            return text or None
        except Exception:
            return None

    @staticmethod
    def _sanitize_repo_reference(value: str) -> str:
        ssh_match = re.match(r"^git@[^:]+:([^/]+)/(.+?)(?:\.git)?$", value)
        if ssh_match:
            return f"{ssh_match.group(1)}/{ssh_match.group(2)}"
        if value.startswith(("https://", "http://", "ssh://")):
            parsed = urlparse(value)
            path = parsed.path.strip("/")
            if path.endswith(".git"):
                path = path[:-4]
            if path:
                return path
            return value
        if re.search(r"^[a-zA-Z][a-zA-Z0-9+.-]*://", value):
            return value

        looks_like_local_path = (
            value.startswith(("/", "~", "./", "../"))
            or "/Users/" in value
            or "/home/" in value
            or "\\" in value
            or re.match(r"^[A-Za-z]:\\\\", value) is not None
        )
        if not looks_like_local_path:
            return value

        parts = [part for part in re.split(r"[\\\\/]+", value.strip()) if part]
        name = parts[-1] if parts else ""
        if name.endswith(".git"):
            name = name[:-4]
        return name or "local-path-redacted"


class ExportOpenApiPass:
    """Run the node exporter in temporary storage and retain its exact JSON text."""

    def __init__(self, node_repo: Path):
        self._node_repo = node_repo

    def run(self) -> OpenApiDocument:
        with TemporaryDirectory(prefix="rgbldk-openapi-") as directory:
            output = Path(directory) / "openapi.json"
            subprocess.check_call(
                ["cargo", "run", "-p", "rgbldkd", "--bin", "rgbldkd", "--",
                 "openapi", "--output", str(output)],
                cwd=self._node_repo,
            )
            content = output.read_text(encoding="utf-8")
        return OpenApiDocument(content, json.loads(content))


class StripDomainAdaptersPass:
    """Remove node-only implementations while retaining the public DTO source."""

    def run(self, sources: tuple[DtoSource, ...]) -> tuple[DtoSource, ...]:
        outputs = []
        for source in sources:
            content = source.content
            if source.source_path.name == "swap.rs":
                # The swap module's trailing adapter and tests belong to the node.
                for marker in ("\nimpl From<crate::swap::SwapInfo>", "\nimpl From<rgb_ldk_node::SwapInfo>"):
                    if marker in content:
                        content = content.split(marker, 1)[0]
                        break
            else:
                content = self._strip_domain_from_impls(content)
            outputs.append(replace(source, content=content.rstrip()))
        return tuple(outputs)

    @staticmethod
    def _strip_domain_from_impls(content: str) -> str:
        """Remove `impl From<crate::…>` / `impl From<rgb_ldk_node::…>` adapters that
        only compile inside rgb-ldk-node."""
        out: list[str] = []
        i = 0
        markers = ("\nimpl From<crate::", "\nimpl From<rgb_ldk_node::")
        while True:
            hits = [(m, content.find(m, i)) for m in markers]
            hits = [(m, p) for m, p in hits if p >= 0]
            if not hits:
                out.append(content[i:])
                break
            marker, j = min(hits, key=lambda item: item[1])
            out.append(content[i:j])
            brace = content.find("{", j + 1)
            if brace < 0:
                out.append(content[j:])
                break
            depth = 0
            p = brace
            while p < len(content):
                ch = content[p]
                if ch == "{":
                    depth += 1
                elif ch == "}":
                    depth -= 1
                    if depth == 0:
                        p += 1
                        break
                p += 1
            if p < len(content) and content[p] == "\n":
                p += 1
            i = p
        return "".join(out)


class WalletEnumsPass:
    """Mirror domain-owned wallet enums without depending on domain crates."""

    NAMES = (
        "WalletUtxoOnchainStateDto",
        "WalletUtxoAvailabilityDto",
        "WalletUtxoOperationPhaseDto",
        "WalletUtxoLockPurposeDto",
    )
    SCHEMA_FIELDS = {"type", "enum", "title", "description", "deprecated", "example", "examples"}

    def __init__(self, openapi: dict):
        self._openapi = openapi
        self._declarations: dict[str, str] = {}
        self._seen: set[str] = set()

    def run(self, sources: tuple[DtoSource, ...]) -> tuple[DtoSource, ...]:
        # Tracking belongs to one run, including when a pass is reused after failure.
        self._declarations = {}
        self._seen = set()
        has_aliases = False
        for source in sources:
            for statement in re.findall(r"pub use\s+[^;]+;", source.content):
                if self._references_wallet_enum(statement):
                    has_aliases = True
        if not has_aliases:
            return sources
        self._load_declarations()
        outputs = []
        for source in sources:
            outputs.append(replace(source, content=self._rewrite(source.content)))
        self._validate_complete()
        return tuple(outputs)

    def _load_declarations(self) -> None:
        components = self._openapi.get("components")
        schemas = components.get("schemas") if isinstance(components, dict) else None
        if not isinstance(schemas, dict):
            raise RuntimeError("wallet enums: missing OpenAPI components.schemas object")
        for name in self.NAMES:
            self._declarations[name] = self._render_enum(name, schemas.get(name))

    @classmethod
    def _references_wallet_enum(cls, statement: str) -> bool:
        for name in cls.NAMES:
            if re.search(rf"\b{name}\b", statement):
                return True
        return False

    def _render_enum(self, name: str, schema: object) -> str:
        if not isinstance(schema, dict) or set(schema) - self.SCHEMA_FIELDS or schema.get("type") != "string":
            raise RuntimeError(f"{name}: expected a plain string enum schema")
        values = schema.get("enum")
        if not isinstance(values, list) or not values:
            raise RuntimeError(f"{name}: expected a nonempty enum")
        variants: dict[str, str] = {}
        for value in values:
            if not isinstance(value, str) or not re.fullmatch(r"[a-z][a-z0-9]*(?:_[a-z0-9]+)*", value):
                raise RuntimeError(f"{name}: unsupported enum value {value!r}; expected snake_case")
            words = []
            for part in value.split("_"):
                words.append(part[0].upper() + part[1:])
            variant = "".join(words)
            if variant == "Self" or variant in variants:
                raise RuntimeError(f"{name}: duplicate or invalid Rust variant {variant}")
            variants[variant] = value
        lines = [
            "#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]",
            f"pub enum {name} {{",
        ]
        for variant, value in sorted(variants.items()):
            lines.append(f'    #[serde(rename = "{value}")]')
            lines.append(f"    {variant},")
        lines.append("}")
        return "\n".join(lines)

    def _rewrite(self, content: str) -> str:
        return re.sub(r"pub use\s+[^;]+;", self._replace_reexport, content)

    def _replace_reexport(self, match: re.Match) -> str:
        statement = match.group(0)
        if not self._references_wallet_enum(statement):
            return statement
        parsed = re.fullmatch(r"pub use\s+txoscope_core::\{([^{}]+)\}\s*;", statement)
        if not parsed:
            raise RuntimeError(f"unsupported wallet enum reexport: {statement}")
        remaining = []
        generated = set()
        for raw_item in parsed[1].split(","):
            item = raw_item.strip()
            if not item:
                continue
            alias = re.fullmatch(r"[A-Za-z_][A-Za-z0-9_]*\s+as\s+([A-Za-z_][A-Za-z0-9_]*)", item)
            if alias and alias[1] in self._declarations:
                name = alias[1]
                if name in self._seen:
                    raise RuntimeError(f"duplicate wallet enum reexport: {name}")
                self._seen.add(name)
                generated.add(name)
            elif self._references_wallet_enum(item):
                raise RuntimeError(f"unsupported wallet enum reexport: {item}")
            else:
                remaining.append(item)
        output = []
        if remaining:
            output.append(f"pub use txoscope_core::{{{', '.join(remaining)}}};")
        for name in self.NAMES:
            if name in generated:
                output.append(self._declarations[name])
        return "\n\n".join(output)

    def _validate_complete(self) -> None:
        missing = set(self._declarations) - self._seen
        if missing:
            raise RuntimeError(f"missing wallet enum reexports: {', '.join(sorted(missing))}")


class RenderArtifactsPass:
    """Render the complete export in memory; no output files are written here."""

    def run(
        self, info: SourceInfo, document: OpenApiDocument, sources: tuple[DtoSource, ...]
    ) -> tuple[Artifact, ...]:
        spec = Path("generated/spec")
        crate = Path("crates/rgbldk_http_dto")
        artifacts = [
            Artifact(spec / "api-version.txt", f"{info.api_version}\n"),
            Artifact(spec / "node-package-version.txt", f"{info.node_package_version}\n"),
            Artifact(spec / "source.json", json.dumps(asdict(info), indent=2, sort_keys=True) + "\n"),
            Artifact(spec / "openapi.json", document.content),
            Artifact(spec / "endpoints.md", self._render_endpoints_md(document.data)),
            Artifact(crate / "Cargo.toml", self._render_manifest(info.node_package_version)),
            Artifact(crate / "src/lib.rs", "//! Generated. Do not edit.\n\npub mod dto;\npub use dto::*;\n"),
        ]
        for source in sources:
            origin = f"rgb-ldk-node/{source.source_path.as_posix()}"
            content = f"// Generated from {origin}. Do not edit.\n\n{source.content}\n"
            artifacts.append(Artifact(crate / source.output_path, content))
        return tuple(artifacts)

    @staticmethod
    def _render_manifest(node_version: str) -> str:
        return "\n".join([
            "[package]",
            'name = "rgbldk_http_dto"',
            f'version = "{node_version}"',
            'edition = "2021"',
            'license = "UNLICENSED"',
            "publish = false",
            "",
            "[dependencies]",
            'serde = { version = "1", features = ["derive"] }',
            'serde_json = "1"',
            'utoipa = "5.4.0"',
            "",
        ])

    def _schema_type_name(self, schema: dict | None) -> str | None:
        if not isinstance(schema, dict):
            return None
        ref = schema.get("$ref")
        if isinstance(ref, str) and ref.startswith("#/components/schemas/"):
            return ref.rsplit("/", 1)[-1]
        schema_type = schema.get("type")
        if schema_type == "array":
            inner = self._schema_type_name(schema.get("items"))
            return f"{inner}[]" if inner else "array"
        if schema_type == "string":
            fmt = schema.get("format")
            if fmt == "binary":
                return "binary"
            return "string"
        if isinstance(schema_type, str):
            return schema_type
        return None

    @staticmethod
    def _prefixed_api_path(path: str) -> str:
        if path.startswith("/api/v1"):
            return path
        return "/api/v1" + (path if path.startswith("/") else "/" + path)

    def _render_endpoints_md(self, openapi: dict) -> str:
        paths = openapi.get("paths")
        if not isinstance(paths, dict):
            raise RuntimeError("openapi.json is missing top-level paths")
        out: list[str] = []
        out.append("# Endpoint List (Prefix: `/api/v1`)")
        out.append("")
        out.append("*This file is exported by `scripts/rgbldk_api_sync.py`.*")
        out.append("")
        route_entries: list[tuple[str, str, dict]] = []
        for raw_path, path_item in paths.items():
            if not isinstance(path_item, dict):
                continue
            for method in ("get", "post", "put", "delete", "patch"):
                op = path_item.get(method)
                if isinstance(op, dict):
                    route_entries.append((method.upper(), self._prefixed_api_path(raw_path), op))

        route_entries.sort(key=lambda item: (item[1], item[0]))

        for method, full_path, op in route_entries:
            out.append(f"### {method} `{full_path}`")
            out.append("")
            summary = op.get("summary")
            if isinstance(summary, str) and summary.strip():
                out.append(f"* **Summary:** {summary.strip()}")
            description = op.get("description")
            if isinstance(description, str) and description.strip():
                out.append(f"* **Description:** {description.strip()}")
            request_body = op.get("requestBody")
            if method in ("POST", "PUT", "PATCH") and isinstance(request_body, dict):
                content = request_body.get("content")
                if isinstance(content, dict):
                    json_content = content.get("application/json")
                    octet_content = content.get("application/octet-stream")
                    selected = json_content if isinstance(json_content, dict) else octet_content
                    if isinstance(selected, dict):
                        req_type = self._schema_type_name(selected.get("schema"))
                        if req_type:
                            out.append(f"* **Request Body:** `{req_type}`")
            responses = op.get("responses")
            if isinstance(responses, dict):
                for status in sorted(responses, key=lambda code: (not str(code).isdigit(), str(code))):
                    resp = responses.get(status)
                    if not isinstance(resp, dict):
                        continue
                    content = resp.get("content")
                    resp_type = None
                    if isinstance(content, dict):
                        json_content = content.get("application/json")
                        octet_content = content.get("application/octet-stream")
                        selected = json_content if isinstance(json_content, dict) else octet_content
                        if isinstance(selected, dict):
                            resp_type = self._schema_type_name(selected.get("schema"))
                    if resp_type:
                        out.append(f"* **Response ({status}):** `{resp_type}`")
                    else:
                        desc = resp.get("description")
                        if isinstance(desc, str) and desc.strip():
                            out.append(f"* **Response ({status}):** {desc.strip()}")
            out.append("")

        return "\n".join(out).rstrip() + "\n"


class WriteArtifactsPass:
    """Materialize a validated, fully rendered export."""

    def __init__(self, output_dir: Path):
        self._output_dir = output_dir

    def run(self, artifacts: tuple[Artifact, ...]) -> None:
        for artifact in artifacts:
            output = self._output_dir / artifact.path
            output.parent.mkdir(parents=True, exist_ok=True)
            output.write_text(artifact.content, encoding="utf-8")


class InstallArtifactsPass:
    """Merge spec files, replace the DTO crate, and remove the legacy crate."""

    def __init__(self, api_repo: Path):
        self._api_repo = api_repo

    def run(self, exported: Path) -> None:
        spec = exported / "generated/spec"
        crate = exported / "crates/rgbldk_http_dto"
        if not spec.is_dir():
            raise RuntimeError(f"missing generated spec in export: {spec}")
        if not crate.is_dir():
            raise RuntimeError(f"missing dto crate in export: {crate}")
        self._copytree_merge(spec, self._api_repo / "generated/spec")
        (self._api_repo / "crates").mkdir(parents=True, exist_ok=True)
        self._copytree_overwrite(crate, self._api_repo / "crates/rgbldk_http_dto")
        legacy = self._api_repo / "generated/crates/rgbldk_http_dto"
        if legacy.exists():
            shutil.rmtree(legacy)

    @staticmethod
    def _copytree_merge(src: Path, dst: Path) -> None:
        dst.mkdir(parents=True, exist_ok=True)
        for path in src.rglob("*"):
            rel = path.relative_to(src)
            out = dst / rel
            if path.is_dir():
                out.mkdir(parents=True, exist_ok=True)
                continue
            out.parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(path, out)

    @staticmethod
    def _copytree_overwrite(src: Path, dst: Path) -> None:
        if dst.exists():
            shutil.rmtree(dst)
        shutil.copytree(src, dst)


class CommitArtifactsPass:
    """Stage exported artifacts and commit when Git reports staged changes."""

    def __init__(self, api_repo: Path, message: str):
        self._api_repo = api_repo
        self._message = message

    def run(self) -> None:
        git = ["git", "-C", str(self._api_repo)]
        subprocess.check_call(git + ["add", "generated", "crates/rgbldk_http_dto"])
        diff = subprocess.run(git + ["diff", "--cached", "--quiet"])
        if diff.returncode == 0:
            return
        if diff.returncode != 1:
            diff.check_returncode()
        subprocess.check_call(git + ["commit", "--no-gpg-sign", "-m", self._message])


class ExportPipeline:
    """Order source, schema, transformation, rendering, and write passes."""

    def __init__(self, node_repo: Path):
        self._node_repo = node_repo.resolve()

    def run(self, output_dir: Path) -> None:
        sources = ReadNodeSourcesPass(self._node_repo).run()
        document = ExportOpenApiPass(self._node_repo).run()
        dtos = StripDomainAdaptersPass().run(sources.dtos)
        dtos = WalletEnumsPass(document.data).run(dtos)
        artifacts = RenderArtifactsPass().run(sources.info, document, dtos)
        WriteArtifactsPass(output_dir.resolve()).run(artifacts)


class SyncPipeline:
    """Keep an export staged until validation succeeds, then install and commit."""

    def __init__(self, node_repo: Path, api_repo: Path):
        self._node_repo = node_repo.resolve()
        self._api_repo = api_repo.resolve()

    def run(self, git_commit: bool, git_commit_message: str) -> None:
        if not (self._api_repo / ".git").exists():
            raise RuntimeError(f"api repo does not look like a git repo: {self._api_repo}")
        temporary_root = self._api_repo / ".tmp"
        temporary_root.mkdir(parents=True, exist_ok=True)
        with TemporaryDirectory(prefix="rgbldk-api-sync-", dir=temporary_root) as directory:
            exported = Path(directory)
            ExportPipeline(self._node_repo).run(exported)
            InstallArtifactsPass(self._api_repo).run(exported)
            if git_commit:
                CommitArtifactsPass(self._api_repo, git_commit_message).run()


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(
        prog="rgbldk_api_sync.py",
        description="Sync API artifacts from rgb-ldk-node into this rgb-ldk-api checkout",
    )
    sub = parser.add_subparsers(dest="cmd", required=True)

    exp = sub.add_parser("export", help="export artifacts to an output directory")
    exp.add_argument("--node-repo", required=True, help="path to rgb-ldk-node repo root")
    exp.add_argument("--out", required=True, help="output directory")

    syn = sub.add_parser("sync", help="export and copy artifacts into this repo")
    syn.add_argument("--node-repo", required=True, help="path to rgb-ldk-node repo root")
    syn.add_argument("--api-repo", default=str(ROOT), help=f"path to rgb-ldk-api repo checkout (default: {ROOT})")
    syn.add_argument("--git-commit", action="store_true", help="commit the synced generated outputs")
    syn.add_argument("--git-commit-message", default="sync: update generated artifacts from rgb-ldk-node")

    args = parser.parse_args(argv)
    if args.cmd == "export":
        ExportPipeline(Path(args.node_repo)).run(Path(args.out))
        return 0
    if args.cmd == "sync":
        SyncPipeline(Path(args.node_repo), Path(args.api_repo)).run(
            git_commit=bool(args.git_commit),
            git_commit_message=str(args.git_commit_message),
        )
        return 0
    raise RuntimeError("unreachable")


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
