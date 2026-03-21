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


ROOT = Path(__file__).resolve().parents[1]


@dataclass
class SourceInfo:
    node_repo: str
    node_commit: str | None
    node_package_version: str
    api_version: str
    generated_at_utc: str


@dataclass
class RouteDef:
    method: str
    path: str
    handler: str
    source_text: str


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


def _sanitize_repo_reference(value: str) -> str:
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


def _find_async_fn_block(text: str, name: str) -> tuple[str, str] | None:
    needle = f"async fn {name}"
    start = text.find(needle)
    if start < 0:
        return None
    brace = text.find("{", start)
    if brace < 0:
        return None
    sig = text[start:brace]
    index = brace
    depth = 0
    while index < len(text):
        char = text[index]
        if char == "{":
            depth += 1
        elif char == "}":
            depth -= 1
            if depth == 0:
                return sig, text[brace : index + 1]
        index += 1
    return None


def _normalize_type_name(type_name: str) -> str:
    value = type_name.strip().replace(" ", "")
    return value.split("::")[-1]


def _infer_request_type(sig: str) -> str | None:
    match = re.search(r"Json\([^)]*\)\s*:\s*Json<\s*([^>]+)\s*>", sig)
    if not match:
        return None
    value = _normalize_type_name(match.group(1))
    if value in ("serde_json::Value", "Value"):
        return None
    return value


def _infer_response_type(sig: str, body: str) -> str | None:
    return_match = re.search(r"->\s*([^{]+)$", sig.strip())
    if return_match:
        return_type = return_match.group(1).strip()
        json_match = re.search(r"Json<\s*([^>]+)\s*>", return_type)
        if json_match:
            inner = json_match.group(1).strip()
            if "serde_json::Value" not in inner and inner != "serde_json::Value":
                vec_match = re.match(r"Vec<\s*([^>]+)\s*>", inner)
                if vec_match:
                    return _normalize_type_name(vec_match.group(1)) + "[]"
                return _normalize_type_name(inner)

    body_match = re.search(r"json_with_status\(\s*StatusCode::OK\s*,\s*([A-Za-z0-9_]+)\b", body)
    if body_match:
        return _normalize_type_name(body_match.group(1))

    return None


def _split_endpoints_md_sections(md: str) -> dict[tuple[str, str], str]:
    lines = md.splitlines()
    sections: dict[tuple[str, str], str] = {}
    index = 0

    while index < len(lines):
        line = lines[index].strip()
        match = re.match(r"^###\s+(GET|POST|PUT|DELETE|PATCH)\s+`([^`]+)`", line)
        if not match:
            index += 1
            continue
        method = match.group(1).upper()
        path = match.group(2).strip()
        start = index
        index += 1
        while index < len(lines) and not re.match(
            r"^###\s+(GET|POST|PUT|DELETE|PATCH)\s+`([^`]+)`", lines[index].strip()
        ):
            index += 1
        sections[(method, path)] = "\n".join(lines[start:index]).rstrip() + "\n"
    return sections


def _route_files(node_repo: Path) -> list[Path]:
    files: list[Path] = []
    handlers_dir = node_repo / "src" / "http" / "handlers"
    if handlers_dir.exists():
        files.extend(sorted(handlers_dir.glob("*.rs")))
    mod_rs = node_repo / "src" / "http" / "mod.rs"
    if mod_rs.exists():
        files.append(mod_rs)
    return files


def _extract_http_routes(node_repo: Path) -> list[RouteDef]:
    pattern = re.compile(
        r'\.route\(\s*"([^"]+)"\s*,\s*(get|post|put|delete|patch)\(\s*([A-Za-z0-9_]+)\s*\)\s*\)',
        re.IGNORECASE,
    )
    routes: list[RouteDef] = []
    seen: set[tuple[str, str]] = set()

    for file_path in _route_files(node_repo):
        text = file_path.read_text(encoding="utf-8")
        for match in pattern.finditer(text):
            route = RouteDef(
                method=match.group(2).upper(),
                path=match.group(1),
                handler=match.group(3),
                source_text=text,
            )
            key = (route.method, route.path)
            if key in seen:
                continue
            seen.add(key)
            routes.append(route)

    return routes


def _render_endpoints_md(node_repo: Path) -> str:
    routes = _extract_http_routes(node_repo)

    existing = ""
    existing_path = node_repo / "docs" / "http-api" / "endpoints.md"
    if existing_path.exists():
        existing = existing_path.read_text(encoding="utf-8")
    sections = _split_endpoints_md_sections(existing) if existing else {}

    out: list[str] = []
    out.append("# Endpoint List (Prefix: `/api/v1`)")
    out.append("")
    out.append("*This file is exported by `scripts/rgbldk_api_sync.py`.*")
    out.append("")

    for route in routes:
        full_path = "/api/v1" + (route.path if route.path.startswith("/") else "/" + route.path)
        key = (route.method, full_path)
        if key in sections:
            out.append(sections[key].rstrip())
            out.append("")
            continue

        fn_block = _find_async_fn_block(route.source_text, route.handler)
        req_type = None
        resp_type = None
        if fn_block:
            sig, body = fn_block
            req_type = _infer_request_type(sig)
            resp_type = _infer_response_type(sig, body)

        out.append(f"### {route.method} `{full_path}`")
        out.append("")
        if route.method in ("POST", "PUT", "PATCH"):
            if req_type:
                out.append(f"* **Request Body:** `{req_type}`")
            else:
                out.append("* **Request:** Empty body.")
        if resp_type:
            out.append(f"* **Response (200):** `{resp_type}`")
        out.append("")

    return "\n".join(out).rstrip() + "\n"


def export_artifacts(node_repo: Path, out_dir: Path) -> None:
    node_repo = node_repo.resolve()
    out_dir = out_dir.resolve()

    cargo_toml = _read_toml(node_repo / "Cargo.toml")
    package = cargo_toml.get("package")
    if not isinstance(package, dict) or "version" not in package:
        raise RuntimeError(f"missing [package].version in {node_repo / 'Cargo.toml'}")
    node_version = str(package["version"])
    api_version = "v1"

    info = SourceInfo(
        node_repo=_sanitize_repo_reference(str(node_repo)),
        node_commit=_git_head(node_repo),
        node_package_version=node_version,
        api_version=api_version,
        generated_at_utc=_rfc3339_utc_now(),
    )

    _write_text(out_dir / "generated" / "spec" / "api-version.txt", f"{api_version}\n")
    _write_text(out_dir / "generated" / "spec" / "node-package-version.txt", f"{node_version}\n")
    _write_text(out_dir / "generated" / "spec" / "source.json", json.dumps(asdict(info), indent=2, sort_keys=True) + "\n")
    _write_text(out_dir / "generated" / "spec" / "endpoints.md", _render_endpoints_md(node_repo))

    dto_src = (node_repo / "src" / "http" / "dto.rs").read_text(encoding="utf-8")
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
                "",
            ]
        ),
    )
    _write_text(crate_dir / "src" / "lib.rs", "//! Generated. Do not edit.\n\npub mod dto;\npub use dto::*;\n")
    _write_text(crate_dir / "src" / "dto.rs", "// Generated from rgb-ldk-node/src/http/dto.rs. Do not edit.\n\n" + dto_src + "\n")


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
