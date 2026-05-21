#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import os
import re
import shutil
import subprocess
import sys
import time
from dataclasses import asdict, dataclass
from pathlib import Path
from urllib.parse import urlparse


ROOT = Path(__file__).resolve().parents[1]


@dataclass
class SourceInfo:
    node_repo: str
    node_commit: str | None
    node_package_version: str
    api_version: str
    generated_at_utc: str


def _read_toml(path: Path) -> dict:
    try:
        import tomllib
    except ModuleNotFoundError:
        raise RuntimeError("python >= 3.11 is required (missing tomllib)") from None

    return tomllib.loads(path.read_text(encoding="utf-8"))


def _git_head(repo: Path) -> str | None:
    try:
        out = subprocess.check_output(["git", "-C", str(repo), "rev-parse", "HEAD"], stderr=subprocess.DEVNULL)
        text = out.decode("utf-8").strip()
        return text or None
    except Exception:
        return None


def _git_origin_url(repo: Path) -> str | None:
    try:
        out = subprocess.check_output(
            ["git", "-C", str(repo), "remote", "get-url", "origin"], stderr=subprocess.DEVNULL
        )
        text = out.decode("utf-8").strip()
        return text or None
    except Exception:
        return None


def _rfc3339_utc_now() -> str:
    return time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime())


def _write_text(path: Path, content: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(content, encoding="utf-8")


def _copytree_overwrite(src: Path, dst: Path) -> None:
    if dst.exists():
        shutil.rmtree(dst)
    shutil.copytree(src, dst)


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


def _generated_header(source: str) -> str:
    return f"// Generated from {source}. Do not edit.\n\n"


def _write_dto_sources(node_repo: Path, crate_dir: Path) -> None:
    src_dir = crate_dir / "src"
    legacy_dto = node_repo / "src" / "http" / "dto.rs"
    module_dto = node_repo / "src" / "http" / "dto"

    if legacy_dto.exists():
        _write_text(
            src_dir / "dto.rs",
            _generated_header("rgb-ldk-node/src/http/dto.rs") + legacy_dto.read_text(encoding="utf-8") + "\n",
        )
        return

    if not module_dto.exists():
        raise RuntimeError(f"missing node DTO source: {legacy_dto} or {module_dto}")

    for path in module_dto.rglob("*.rs"):
        rel = path.relative_to(module_dto)
        out = src_dir / "dto" / rel
        _write_text(
            out,
            _generated_header(f"rgb-ldk-node/src/http/dto/{rel.as_posix()}")
            + path.read_text(encoding="utf-8")
            + "\n",
        )


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


def _schema_type_name(schema: dict | None) -> str | None:
    if not isinstance(schema, dict):
        return None
    ref = schema.get("$ref")
    if isinstance(ref, str) and ref.startswith("#/components/schemas/"):
        return ref.rsplit("/", 1)[-1]
    schema_type = schema.get("type")
    if schema_type == "array":
        inner = _schema_type_name(schema.get("items"))
        return f"{inner}[]" if inner else "array"
    if schema_type == "string":
        fmt = schema.get("format")
        if fmt == "binary":
            return "binary"
        return "string"
    if isinstance(schema_type, str):
        return schema_type
    return None


def _prefixed_api_path(path: str) -> str:
    if path.startswith("/api/v1"):
        return path
    return "/api/v1" + (path if path.startswith("/") else "/" + path)


def _render_endpoints_md(openapi: dict) -> str:
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
                route_entries.append((method.upper(), _prefixed_api_path(raw_path), op))

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
                    req_type = _schema_type_name(selected.get("schema"))
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
                        resp_type = _schema_type_name(selected.get("schema"))
                if resp_type:
                    out.append(f"* **Response ({status}):** `{resp_type}`")
                else:
                    desc = resp.get("description")
                    if isinstance(desc, str) and desc.strip():
                        out.append(f"* **Response ({status}):** {desc.strip()}")
        out.append("")

    return "\n".join(out).rstrip() + "\n"


def _export_openapi_json(node_repo: Path, output_path: Path) -> None:
    output_path.parent.mkdir(parents=True, exist_ok=True)
    subprocess.check_call(
        ["cargo", "run", "--bin", "rgbldkd", "--", "openapi", "--output", str(output_path)],
        cwd=node_repo,
    )


def export_artifacts(node_repo: Path, out_dir: Path) -> None:
    node_repo = node_repo.resolve()
    out_dir = out_dir.resolve()

    cargo_toml = _read_toml(node_repo / "Cargo.toml")
    package = cargo_toml.get("package")
    if not isinstance(package, dict) or "version" not in package:
        raise RuntimeError(f"missing [package].version in {node_repo / 'Cargo.toml'}")
    node_version = str(package["version"])
    api_version = "v1"

    node_repo_ref = _git_origin_url(node_repo) or str(node_repo)
    info = SourceInfo(
        node_repo=_sanitize_repo_reference(node_repo_ref),
        node_commit=_git_head(node_repo),
        node_package_version=node_version,
        api_version=api_version,
        generated_at_utc=_rfc3339_utc_now(),
    )

    _write_text(out_dir / "generated" / "spec" / "api-version.txt", f"{api_version}\n")
    _write_text(out_dir / "generated" / "spec" / "node-package-version.txt", f"{node_version}\n")
    _write_text(out_dir / "generated" / "spec" / "source.json", json.dumps(asdict(info), indent=2, sort_keys=True) + "\n")
    exported_openapi = out_dir / "generated" / "spec" / "openapi.json"
    _export_openapi_json(node_repo, exported_openapi)
    openapi = json.loads(exported_openapi.read_text(encoding="utf-8"))
    _write_text(out_dir / "generated" / "spec" / "endpoints.md", _render_endpoints_md(openapi))

    crate_dir = out_dir / "crates" / "rgbldk_http_dto"
    _write_text(
        crate_dir / "Cargo.toml",
        "\n".join(
            [
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
            ]
        ),
    )
    _write_text(crate_dir / "src" / "lib.rs", "//! Generated. Do not edit.\n\npub mod dto;\npub use dto::*;\n")
    _write_dto_sources(node_repo, crate_dir)


def sync_to_api_repo(node_repo: Path, api_repo: Path, git_commit: bool, git_commit_message: str) -> None:
    node_repo = node_repo.resolve()
    api_repo = api_repo.resolve()
    if not (api_repo / ".git").exists():
        raise RuntimeError(f"api repo does not look like a git repo: {api_repo}")

    tmp_root = api_repo / ".tmp"
    tmp_root.mkdir(parents=True, exist_ok=True)
    tmp_export = tmp_root / f"rgbldk-api-sync-{os.getpid()}"
    if tmp_export.exists():
        shutil.rmtree(tmp_export)
    tmp_export.mkdir(parents=True, exist_ok=True)

    try:
        export_artifacts(node_repo=node_repo, out_dir=tmp_export)

        tmp_generated_spec = tmp_export / "generated" / "spec"
        if not tmp_generated_spec.exists():
            raise RuntimeError(f"missing generated spec in export: {tmp_generated_spec}")
        (api_repo / "generated").mkdir(parents=True, exist_ok=True)
        _copytree_merge(tmp_generated_spec, api_repo / "generated" / "spec")

        tmp_dto_crate = tmp_export / "crates" / "rgbldk_http_dto"
        if not tmp_dto_crate.exists():
            raise RuntimeError(f"missing dto crate in export: {tmp_dto_crate}")
        (api_repo / "crates").mkdir(parents=True, exist_ok=True)
        _copytree_overwrite(tmp_dto_crate, api_repo / "crates" / "rgbldk_http_dto")

        legacy = api_repo / "generated" / "crates" / "rgbldk_http_dto"
        if legacy.exists():
            shutil.rmtree(legacy)

        if git_commit:
            subprocess.check_call(
                [
                    "git",
                    "-C",
                    str(api_repo),
                    "add",
                    "generated",
                    "crates/rgbldk_http_dto",
                ]
            )
            diff_quiet = subprocess.call(["git", "-C", str(api_repo), "diff", "--cached", "--quiet"])
            if diff_quiet != 0:
                subprocess.check_call(
                    ["git", "-C", str(api_repo), "commit", "--no-gpg-sign", "-m", git_commit_message]
                )
    finally:
        if tmp_export.exists():
            shutil.rmtree(tmp_export)


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
        export_artifacts(node_repo=Path(args.node_repo), out_dir=Path(args.out))
        return 0
    if args.cmd == "sync":
        sync_to_api_repo(
            node_repo=Path(args.node_repo),
            api_repo=Path(args.api_repo),
            git_commit=bool(args.git_commit),
            git_commit_message=str(args.git_commit_message),
        )
        return 0
    raise RuntimeError("unreachable")


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
