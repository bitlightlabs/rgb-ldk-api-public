#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import re
import sys
from dataclasses import dataclass
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
DTO_RS = ROOT / "crates" / "rgbldk_http_dto" / "src" / "dto.rs"
SOURCE_JSON = ROOT / "generated" / "spec" / "source.json"

OUT_TS_TYPES = ROOT / "packages" / "rgbldk-node-api-types" / "src" / "generated.ts"
OUT_TS_CLIENT = ROOT / "packages" / "rgbldk-node-client" / "src" / "generated.ts"
OUT_RS_CLIENT = ROOT / "crates" / "rgbldk_http_client" / "src" / "generated.rs"
OUT_OPENAPI_JSON = ROOT / "generated" / "spec" / "openapi.json"


@dataclass
class Field:
    name: str
    json_name: str
    rust_ty: str
    optional: bool
    u64_string: bool


@dataclass
class Struct:
    name: str
    fields: list[Field]


@dataclass
class Variant:
    name: str
    fields: list[Field]
    tuple_ty: str | None = None


@dataclass
class Enum:
    name: str
    serde_tag: str | None
    serde_content: str | None
    variants: list[Variant]


def main(argv: list[str]) -> int:
    p = argparse.ArgumentParser(prog="rgb-ldk-api.py")
    sub = p.add_subparsers(dest="cmd", required=True)
    sub.add_parser("gen", help="generate TS DTO/types + TS client + Rust client stubs")
    sub.add_parser("sanitize", help="sanitize generated metadata (e.g. remove local paths)")

    args = p.parse_args(argv)
    if args.cmd == "gen":
        gen()
        return 0
    if args.cmd == "sanitize":
        sanitize_generated_metadata()
        return 0
    raise RuntimeError("unreachable")


def gen() -> None:
    if not DTO_RS.exists():
        raise SystemExit(f"missing dto source: {DTO_RS} (run node sync first)")
    if not OUT_OPENAPI_JSON.exists():
        raise SystemExit(f"missing openapi source: {OUT_OPENAPI_JSON} (run node sync first)")

    src = DTO_RS.read_text(encoding="utf-8").splitlines()
    structs, enums = parse_rust_dtos(src)

    OUT_TS_TYPES.write_text(render_ts_types(structs, enums), encoding="utf-8")
    OUT_TS_CLIENT.write_text(render_ts_client(), encoding="utf-8")
    OUT_RS_CLIENT.write_text(render_rs_client(), encoding="utf-8")
    sanitize_generated_metadata()


def sanitize_generated_metadata() -> None:
    sanitize_source_json(SOURCE_JSON)


def sanitize_source_json(path: Path) -> None:
    if not path.exists():
        return
    try:
        data = json.loads(path.read_text(encoding="utf-8"))
    except Exception as e:
        print(f"warning: failed to parse {path}: {e}", file=sys.stderr)
        return
    if not isinstance(data, dict):
        return

    node_repo = data.get("node_repo")
    if not isinstance(node_repo, str):
        return

    sanitized = sanitize_repo_reference(node_repo)
    if sanitized == node_repo:
        return

    data["node_repo"] = sanitized
    path.write_text(json.dumps(data, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")


def sanitize_repo_reference(value: str) -> str:
    # Keep URLs / already-sanitized identifiers untouched.
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

    parts = [p for p in re.split(r"[\\\\/]+", value.strip()) if p]
    name = parts[-1] if parts else ""
    if name.endswith(".git"):
        name = name[:-4]
    return name or "local-path-redacted"


def parse_rust_dtos(lines: list[str]) -> tuple[list[Struct], list[Enum]]:
    structs: list[Struct] = []
    enums: list[Enum] = []

    i = 0
    attrs: list[str] = []

    while i < len(lines):
        line = lines[i].strip()
        if line.startswith("#["):
            attr, i = consume_attr(lines, i)
            attrs.append(attr)
            continue

        m = re.match(r"pub struct ([A-Za-z0-9_]+)\s*\{", line)
        if m:
            name = m.group(1)
            i += 1
            fields: list[Field] = []
            field_attrs: list[str] = []
            while i < len(lines):
                l = lines[i].strip()
                if l.startswith("#["):
                    attr, i = consume_attr(lines, i)
                    field_attrs.append(attr)
                    continue
                if l.startswith("}"):
                    break
                fm = re.match(r"pub\s+([A-Za-z0-9_#]+)\s*:\s*([^,]+),", l)
                if fm:
                    raw_name = fm.group(1)
                    rust_ty = fm.group(2).strip()
                    json_name = field_json_name(raw_name, field_attrs)
                    optional = is_optional_field(rust_ty, field_attrs)
                    u64_string = is_u64_decimal_string(field_attrs)
                    fields.append(
                        Field(
                            name=rust_ident_to_ts(raw_name),
                            json_name=json_name,
                            rust_ty=rust_ty,
                            optional=optional,
                            u64_string=u64_string,
                        )
                    )
                    field_attrs = []
                i += 1
            structs.append(Struct(name=name, fields=fields))
            attrs = []
            i += 1
            continue

        m = re.match(r"pub enum ([A-Za-z0-9_]+)\s*\{", line)
        if m:
            name = m.group(1)
            serde_tag, serde_content = parse_enum_serde_tag(attrs)
            i += 1
            variants: list[Variant] = []
            variant_attrs: list[str] = []
            while i < len(lines):
                l = lines[i].strip()
                if l.startswith("#["):
                    attr, i = consume_attr(lines, i)
                    variant_attrs.append(attr)
                    continue
                if l.startswith("}"):
                    break

                # VariantName { ... }
                vm = re.match(r"([A-Za-z0-9_]+)\s*\{", l)
                if vm:
                    vname = vm.group(1)
                    i += 1
                    fields: list[Field] = []
                    field_attrs: list[str] = []
                    while i < len(lines):
                        vl = lines[i].strip()
                        if vl.startswith("#["):
                            attr, i = consume_attr(lines, i)
                            field_attrs.append(attr)
                            continue
                        if vl.startswith("}"):
                            break
                        fm = re.match(r"([A-Za-z0-9_#]+)\s*:\s*([^,]+),", vl)
                        if fm:
                            raw_name = fm.group(1)
                            rust_ty = fm.group(2).strip()
                            json_name = field_json_name(raw_name, field_attrs)
                            optional = is_optional_field(rust_ty, field_attrs)
                            u64_string = is_u64_decimal_string(field_attrs)
                            fields.append(
                                Field(
                                    name=rust_ident_to_ts(raw_name),
                                    json_name=json_name,
                                    rust_ty=rust_ty,
                                    optional=optional,
                                    u64_string=u64_string,
                                )
                            )
                            field_attrs = []
                        i += 1
                    variants.append(Variant(name=vname, fields=fields))
                    variant_attrs = []
                    i += 1
                    # optional trailing comma
                    if i < len(lines) and lines[i].strip().startswith(","):
                        i += 1
                    continue

                # Unit variant: VariantName,
                vm = re.match(r"([A-Za-z0-9_]+)\s*,", l)
                if vm:
                    variants.append(Variant(name=vm.group(1), fields=[]))
                    variant_attrs = []
                    i += 1
                    continue

                # Tuple variant: VariantName(Type),
                vm = re.match(r"([A-Za-z0-9_]+)\s*\(\s*([^\)]+)\s*\)\s*,?", l)
                if vm:
                    vname = vm.group(1)
                    inner_ty = vm.group(2).strip()
                    variants.append(Variant(name=vname, fields=[], tuple_ty=inner_ty))
                    variant_attrs = []
                    i += 1
                    continue

                i += 1

            enums.append(
                Enum(name=name, serde_tag=serde_tag, serde_content=serde_content, variants=variants)
            )
            attrs = []
            i += 1
            continue

        attrs = []
        i += 1

    # Keep output stable: sort by name, but keep enums after structs.
    structs.sort(key=lambda s: s.name)
    enums.sort(key=lambda e: e.name)
    return structs, enums


def consume_attr(lines: list[str], i: int) -> tuple[str, int]:
    # Consume a possibly multi-line Rust attribute:
    #   #[serde(
    #     default,
    #     with = "..."
    #   )]
    parts = [lines[i].strip()]
    i += 1
    while parts and not parts[-1].endswith("]") and i < len(lines):
        parts.append(lines[i].strip())
        i += 1
    return (" ".join(parts), i)


def parse_enum_serde_tag(attrs: list[str]) -> tuple[str | None, str | None]:
    for a in attrs:
        if a.startswith("#[serde(") and "tag" in a and "content" in a:
            # #[serde(tag = "type", content = "data")]
            tag_m = re.search(r'tag\s*=\s*"([^"]+)"', a)
            content_m = re.search(r'content\s*=\s*"([^"]+)"', a)
            return (tag_m.group(1) if tag_m else None, content_m.group(1) if content_m else None)
    return (None, None)


def rust_ident_to_ts(raw: str) -> str:
    if raw.startswith("r#"):
        return raw[2:]
    return raw


def field_json_name(raw: str, attrs: list[str]) -> str:
    # Default: same as identifier (strip r#)
    json_name = rust_ident_to_ts(raw)
    for a in attrs:
        if a.startswith("#[serde(") and "rename" in a:
            m = re.search(r'rename\s*=\s*"([^"]+)"', a)
            if m:
                json_name = m.group(1)
    return json_name


def is_optional_field(rust_ty: str, attrs: list[str]) -> bool:
    rust_ty = rust_ty.strip()
    if rust_ty.startswith("Option<"):
        return True
    for a in attrs:
        if a.startswith("#[serde(") and "skip_serializing_if" in a:
            return True
        if a.startswith("#[serde(") and "default" in a:
            return True
        if a == "#[serde(default)]":
            return True
    return False


def is_u64_decimal_string(attrs: list[str]) -> bool:
    for a in attrs:
        if "serde_u64_decimal_string" in a:
            return True
        if "serde_opt_u64_decimal_string" in a:
            return True
    return False


def ts_type_for(field: Field) -> str:
    ty = field.rust_ty.replace(" ", "")
    if ty.startswith("Option<") and ty.endswith(">"):
        inner = ty[len("Option<") : -1]
        # Option + serde_opt_u64_decimal_string should map to U64String
        return ts_type_for(Field(field.name, field.json_name, inner, False, field.u64_string))
    if ty.startswith("Vec<") and ty.endswith(">"):
        inner = ty[len("Vec<") : -1]
        inner_ts = ts_type_for(Field(field.name, field.json_name, inner, False, field.u64_string))
        return f"{inner_ts}[]"
    if ty in ("String",):
        return "string"
    if ty in ("bool",):
        return "boolean"
    if ty in ("u8", "u16", "u32", "usize", "i32", "i64"):
        return "number"
    if ty in ("f32", "f64"):
        return "number"
    if ty == "u64":
        return "U64String" if field.u64_string else "number"
    if ty.startswith("serde_json::"):
        return "unknown"
    # Nested DTO type name
    return ty


def render_ts_types(structs: list[Struct], enums: list[Enum]) -> str:
    out: list[str] = []
    out.append("// Generated. Do not edit.")
    out.append("// Source: crates/rgbldk_http_dto/src/dto.rs")
    out.append("// Run: `pnpm gen` at repo root.")
    out.append("")
    out.append("export type U64String = string;")
    out.append("")

    for s in structs:
        out.append(f"export interface {s.name} " + "{")
        for f in s.fields:
            key = f.name
            # Preserve JSON name if it differs: use string literal key.
            if f.json_name != f.name:
                key = f'"{f.json_name}"'
            opt = "?" if f.optional else ""
            out.append(f"  {key}{opt}: {ts_type_for(f)};")
        out.append("}")
        out.append("")

    for e in enums:
        if e.serde_tag and e.serde_content:
            parts: list[str] = []
            for v in e.variants:
                if v.tuple_ty is not None:
                    data_ts = ts_type_for(Field("value", "value", v.tuple_ty, False, False))
                    parts.append(f'  | {{ {e.serde_tag}: "{v.name}"; {e.serde_content}: {data_ts}; }}')
                    continue
                data_lines: list[str] = []
                for f in v.fields:
                    key = f.name
                    if f.json_name != f.name:
                        key = f'"{f.json_name}"'
                    opt = "?" if f.optional else ""
                    data_lines.append(f"      {key}{opt}: {ts_type_for(f)};")
                if not data_lines:
                    data = "{}"
                else:
                    data = "{\n" + "\n".join(data_lines) + "\n    }"
                parts.append(f'  | {{ {e.serde_tag}: "{v.name}"; {e.serde_content}: {data}; }}')
            out.append(f"export type {e.name} =")
            out.extend(parts)
            out.append(";")
            out.append("")
        else:
            # Untagged enums (or enums without tag/content):
            # - If all variants are unit variants => string union
            # - If any variant has fields/tuple payload => union of shapes (no discriminant in JSON)
            has_payload = any((len(v.fields) > 0) or (v.tuple_ty is not None) for v in e.variants)
            if not has_payload:
                lit = " | ".join([f'\"{v.name}\"' for v in e.variants]) or "never"
                out.append(f"export type {e.name} = {lit};")
                out.append("")
                continue

            parts: list[str] = []
            for v in e.variants:
                if v.tuple_ty is not None:
                    parts.append(f"  | {ts_type_for(Field('value', 'value', v.tuple_ty, False, False))}")
                    continue
                if not v.fields:
                    # Mixed unit+struct variants are rare; fall back to `unknown` for this variant.
                    parts.append("  | unknown")
                    continue
                data_lines: list[str] = []
                for f in v.fields:
                    key = f.name
                    if f.json_name != f.name:
                        key = f'"{f.json_name}"'
                    opt = "?" if f.optional else ""
                    data_lines.append(f"    {key}{opt}: {ts_type_for(f)};")
                parts.append("  | {\n" + "\n".join(data_lines) + "\n  }")
            out.append(f"export type {e.name} =")
            out.extend(parts)
            out.append(";")
            out.append("")

    return "\n".join(out).rstrip() + "\n"


def render_ts_client() -> str:
    return (
        "\n".join(
            [
                "// Generated. Do not edit.",
                "// Run: `pnpm gen` at repo root.",
                "",
                "export type TokenSource =",
                "  | { kind: \"none\" }",
                "  | { kind: \"fixed\"; token: string }",
                "  | { kind: \"provider\"; getToken: () => string | Promise<string> | null | undefined };",
                "",
                "export type ClientOptions = {",
                "  baseUrl: string;",
                "  token?: TokenSource;",
                "  fetchImpl?: typeof fetch;",
                "};",
                "",
                "export class RgbLdkNodeClient {",
                "  private readonly baseUrl: string;",
                "  private readonly token: TokenSource;",
                "  private readonly fetchImpl: typeof fetch;",
                "",
                "  constructor(opts: ClientOptions) {",
                "    this.baseUrl = opts.baseUrl.replace(/\\/+$/, \"\");",
                "    this.token = opts.token ?? { kind: \"none\" };",
                "    this.fetchImpl = opts.fetchImpl ?? fetch;",
                "  }",
                "",
                "  private async authHeader(): Promise<Record<string, string>> {",
                "    if (this.token.kind === \"none\") return {};",
                "    if (this.token.kind === \"fixed\") return { Authorization: `Bearer ${this.token.token}` };",
                "    const token = await this.token.getToken();",
                "    if (!token) return {};",
                "    const trimmed = token.trim();",
                "    if (!trimmed) return {};",
                "    return { Authorization: `Bearer ${trimmed}` };",
                "  }",
                "",
                "  async getJson<T>(path: string): Promise<T> {",
                "    const url = `${this.baseUrl}${path.startsWith(\"/\") ? \"\" : \"/\"}${path}`;",
                "    const headers = await this.authHeader();",
                "    const res = await this.fetchImpl(url, { method: \"GET\", headers });",
                "    const text = await res.text();",
                "    if (!res.ok) throw new Error(`HTTP ${res.status}: ${text}`);",
                "    return JSON.parse(text) as T;",
                "  }",
                "",
                "  async postJson<B, T>(path: string, body: B): Promise<T> {",
                "    const url = `${this.baseUrl}${path.startsWith(\"/\") ? \"\" : \"/\"}${path}`;",
                "    const headers = {",
                "      \"content-type\": \"application/json\",",
                "      ...(await this.authHeader()),",
                "    };",
                "    const res = await this.fetchImpl(url, { method: \"POST\", headers, body: JSON.stringify(body) });",
                "    const text = await res.text();",
                "    if (!res.ok) throw new Error(`HTTP ${res.status}: ${text}`);",
                "    return JSON.parse(text) as T;",
                "  }",
                "}",
                "",
            ]
        )
        + "\n"
    )


def render_rs_client() -> str:
    return (
        "\n".join(
            [
                "// Generated. Do not edit.",
                "// Run: `python3 scripts/rgb-ldk-api.py gen`",
                "",
                "use std::path::PathBuf;",
                "",
                "use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};",
                "use reqwest::Url;",
                "use thiserror::Error;",
                "",
                "#[derive(Debug, Clone)]",
                "pub enum Auth {",
                "\tNone,",
                "\tFixedToken(String),",
                "\tTokenFile(PathBuf),",
                "}",
                "",
                "#[derive(Debug, Error)]",
                "pub enum ClientError {",
                "\t#[error(\"invalid base url: {0}\")]",
                "\tInvalidBaseUrl(String),",
                "\t#[error(\"invalid bearer token\")]",
                "\tInvalidBearerToken,",
                "\t#[error(\"read token file failed: {0}\")]",
                "\tReadTokenFile(std::io::Error),",
                "\t#[error(\"http error: {0}\")]",
                "\tHttp(#[from] reqwest::Error),",
                "\t#[error(\"json error: {0}\")]",
                "\tJson(#[from] serde_json::Error),",
                "\t#[error(\"non-success status: {status} body={body}\")]",
                "\tNonSuccess { status: u16, body: String },",
                "}",
                "",
                "#[derive(Clone)]",
                "pub struct Client {",
                "\tbase_url: Url,",
                "\thttp: reqwest::Client,",
                "\tauth: Auth,",
                "}",
                "",
                "impl Client {",
                "\tpub fn new(base_url: &str, auth: Auth) -> Result<Self, ClientError> {",
                "\t\tlet base_url = Url::parse(base_url)",
                "\t\t\t.map_err(|_| ClientError::InvalidBaseUrl(base_url.to_string()))?;",
                "\t\tOk(Self { base_url, http: reqwest::Client::new(), auth })",
                "\t}",
                "",
                "\tfn read_token_file(path: &std::path::Path) -> Result<String, std::io::Error> {",
                "\t\tOk(std::fs::read_to_string(path)?.trim().to_string())",
                "\t}",
                "",
                "\tasync fn headers(&self) -> Result<HeaderMap, ClientError> {",
                "\t\tlet mut headers = HeaderMap::new();",
                "\t\tlet token_opt = match &self.auth {",
                "\t\t\tAuth::None => None,",
                "\t\t\tAuth::FixedToken(t) => Some(t.clone()),",
                "\t\t\tAuth::TokenFile(p) => {",
                "\t\t\t\tSome(Self::read_token_file(p).map_err(ClientError::ReadTokenFile)?)",
                "\t\t\t},",
                "\t\t};",
                "\t\tif let Some(token) = token_opt {",
                "\t\t\tlet v = HeaderValue::from_str(&format!(\"Bearer {token}\"))",
                "\t\t\t\t.map_err(|_| ClientError::InvalidBearerToken)?;",
                "\t\t\theaders.insert(AUTHORIZATION, v);",
                "\t\t}",
                "\t\tOk(headers)",
                "\t}",
                "",
                "\tpub async fn get_json<T: serde::de::DeserializeOwned>(&self, path: &str) -> Result<T, ClientError> {",
                "\t\tlet url = self.base_url.join(path)",
                "\t\t\t.map_err(|_| ClientError::InvalidBaseUrl(path.to_string()))?;",
                "\t\tlet headers = self.headers().await?;",
                "\t\tlet resp = self.http.get(url).headers(headers).send().await?;",
                "\t\tlet status = resp.status();",
                "\t\tlet body = resp.text().await?;",
                "\t\tif !status.is_success() {",
                "\t\t\treturn Err(ClientError::NonSuccess { status: status.as_u16(), body });",
                "\t\t}",
                "\t\tOk(serde_json::from_str(&body)?)",
                "\t}",
                "",
                "\tpub async fn post_json<B: serde::Serialize, T: serde::de::DeserializeOwned>(",
                "\t\t&self,",
                "\t\tpath: &str,",
                "\t\tbody: &B,",
                "\t) -> Result<T, ClientError> {",
                "\t\tlet url = self.base_url.join(path)",
                "\t\t\t.map_err(|_| ClientError::InvalidBaseUrl(path.to_string()))?;",
                "\t\tlet headers = self.headers().await?;",
                "\t\tlet resp = self.http.post(url).headers(headers).json(body).send().await?;",
                "\t\tlet status = resp.status();",
                "\t\tlet body = resp.text().await?;",
                "\t\tif !status.is_success() {",
                "\t\t\treturn Err(ClientError::NonSuccess { status: status.as_u16(), body });",
                "\t\t}",
                "\t\tOk(serde_json::from_str(&body)?)",
                "\t}",
                "}",
                "",
            ]
        )
        + "\n"
    )


def render_openapi_json(structs: list[Struct], enums: list[Enum], endpoints_md: str) -> str:
    endpoints = parse_endpoints_md(endpoints_md)
    schemas: dict[str, dict] = {}

    for s in structs:
        schemas[s.name] = schema_for_struct(s)
    for e in enums:
        schemas[e.name] = schema_for_enum(e)

    paths: dict[str, dict] = {}
    for ep in endpoints:
        oas_path, params = to_openapi_path(ep.path)
        method = ep.method.lower()
        path_item = paths.setdefault(oas_path, {})

        op: dict = {
            "operationId": f"{ep.method}_{oas_path}".replace("/", "_").replace("{", "").replace("}", ""),
            "responses": {},
        }
        if params:
            op["parameters"] = [
                {
                    "name": p,
                    "in": "path",
                    "required": True,
                    "schema": {"type": "string"},
                }
                for p in params
            ]

        if ep.request_type:
            op["requestBody"] = {
                "required": True,
                "content": {"application/json": {"schema": schema_ref_or_inline(ep.request_type)}},
            }

        for status, resp_type in ep.responses.items():
            op["responses"][str(status)] = {
                "description": "OK" if int(status) < 400 else "Error",
                "content": {"application/json": {"schema": schema_ref_or_inline(resp_type)}},
            }

        # Ensure at least one response exists.
        if not op["responses"]:
            op["responses"]["200"] = {"description": "OK"}

        path_item[method] = op

    doc = {
        "openapi": "3.1.0",
        "info": {
            "title": "rgbldk node api",
            "version": "v1",
        },
        "servers": [{"url": "http://127.0.0.1:8500"}],
        "paths": paths,
        "components": {"schemas": schemas},
    }
    return json.dumps(doc, indent=2, sort_keys=True) + "\n"


@dataclass
class Endpoint:
    method: str
    path: str
    request_type: str | None
    responses: dict[int, str]


def parse_endpoints_md(md: str) -> list[Endpoint]:
    if not md.strip():
        return []

    lines = md.splitlines()
    endpoints: list[Endpoint] = []
    i = 0
    cur: Endpoint | None = None
    while i < len(lines):
        line = lines[i].strip()
        m = re.match(r"^###\s+(GET|POST|PUT|DELETE|PATCH)\s+`([^`]+)`", line)
        if m:
            if cur:
                endpoints.append(cur)
            cur = Endpoint(method=m.group(1), path=m.group(2), request_type=None, responses={})
            i += 1
            continue

        if cur:
            # Request body: `* **Request Body:** TypeName`
            rm = re.search(r"\*\*\s*Request(?: Body)?\s*:\s*\*\*\s*`?([A-Za-z0-9_\\[\\]]+)`?", line)
            if rm:
                t = rm.group(1).strip()
                if t.lower().startswith("empty"):
                    cur.request_type = None
                else:
                    cur.request_type = normalize_type_name(t)

            # Request Body: `* **Request Body:** TypeName`
            rm2 = re.search(r"\*\*\s*Request Body\s*:\s*\*\*\s*([A-Za-z0-9_\\[\\]]+)", line)
            if rm2:
                cur.request_type = normalize_type_name(rm2.group(1).strip())

            # Response: `* **Response (200):** TypeName`
            resp = re.search(r"\*\*\s*Response\s*\\(([^\\)]+)\\)\s*:\s*\*\*\s*`?([A-Za-z0-9_\\[\\]]+)`?", line)
            if resp:
                codes = resp.group(1)
                t = normalize_type_name(resp.group(2).strip())
                for code in codes.split("/"):
                    code = code.strip()
                    if code.isdigit():
                        cur.responses[int(code)] = t

        i += 1

    if cur:
        endpoints.append(cur)

    return endpoints


def normalize_type_name(t: str) -> str:
    # `Type[]` -> `Type[]`
    t = t.strip()
    # Strip surrounding backticks if any remain.
    t = t.strip("`")
    return t


def to_openapi_path(path: str) -> tuple[str, list[str]]:
    # Convert /api/v1/payment/:payment_id -> /api/v1/payment/{payment_id}
    params: list[str] = []

    def repl(m: re.Match) -> str:
        name = m.group(1)
        params.append(name)
        return "{" + name + "}"

    out = re.sub(r":([A-Za-z0-9_]+)", repl, path)
    return out, params


def schema_ref_or_inline(type_name: str) -> dict:
    # Supports `TypeName[]`.
    if type_name.endswith("[]"):
        base = type_name[:-2]
        return {"type": "array", "items": schema_ref_or_inline(base)}
    # Primitive fallbacks
    if type_name in ("string", "String"):
        return {"type": "string"}
    return {"$ref": f"#/components/schemas/{type_name}"}


def schema_for_struct(s: Struct) -> dict:
    props: dict[str, dict] = {}
    required: list[str] = []
    for f in s.fields:
        props[f.json_name] = schema_for_field(f)
        if not f.optional:
            required.append(f.json_name)
    sch: dict = {"type": "object", "properties": props}
    if required:
        sch["required"] = sorted(required)
    return sch


def schema_for_enum(e: Enum) -> dict:
    if e.serde_tag and e.serde_content:
        one_of: list[dict] = []
        for v in e.variants:
            if v.tuple_ty is not None:
                variant_schema = {
                    "type": "object",
                    "properties": {
                        e.serde_tag: {"const": v.name},
                        e.serde_content: schema_ref_or_inline(v.tuple_ty),
                    },
                    "required": [e.serde_tag, e.serde_content],
                }
                one_of.append(variant_schema)
                continue
            data_props: dict[str, dict] = {}
            data_required: list[str] = []
            for f in v.fields:
                data_props[f.json_name] = schema_for_field(f)
                if not f.optional:
                    data_required.append(f.json_name)
            data_schema: dict = {"type": "object", "properties": data_props}
            if data_required:
                data_schema["required"] = sorted(data_required)
            variant_schema = {
                "type": "object",
                "properties": {
                    e.serde_tag: {"const": v.name},
                    e.serde_content: data_schema,
                },
                "required": [e.serde_tag, e.serde_content],
            }
            one_of.append(variant_schema)
        return {"oneOf": one_of}

    # Untagged enums:
    # - If all variants are unit variants => string enum
    # - Else => oneOf union of the possible payload shapes
    has_payload = any((len(v.fields) > 0) or (v.tuple_ty is not None) for v in e.variants)
    if not has_payload:
        return {"type": "string", "enum": [v.name for v in e.variants]}

    one_of: list[dict] = []
    for v in e.variants:
        if v.tuple_ty is not None:
            one_of.append(schema_ref_or_inline(v.tuple_ty))
            continue
        if v.fields:
            data_props: dict[str, dict] = {}
            data_required: list[str] = []
            for f in v.fields:
                data_props[f.json_name] = schema_for_field(f)
                if not f.optional:
                    data_required.append(f.json_name)
            data_schema: dict = {"type": "object", "properties": data_props}
            if data_required:
                data_schema["required"] = sorted(data_required)
            one_of.append(data_schema)
            continue
        one_of.append({"type": "string", "const": v.name})

    return {"oneOf": one_of}


def schema_for_field(f: Field) -> dict:
    ty = f.rust_ty.replace(" ", "")
    if ty.startswith("Option<") and ty.endswith(">"):
        inner = ty[len("Option<") : -1]
        return schema_for_field(Field(f.name, f.json_name, inner, False, f.u64_string))
    if ty.startswith("Vec<") and ty.endswith(">"):
        inner = ty[len("Vec<") : -1]
        return {"type": "array", "items": schema_for_field(Field(f.name, f.json_name, inner, False, f.u64_string))}
    if ty == "String":
        return {"type": "string"}
    if ty == "bool":
        return {"type": "boolean"}
    if ty in ("u8", "u16", "u32", "usize", "i32", "i64"):
        return {"type": "integer"}
    if ty in ("f32", "f64"):
        return {"type": "number"}
    if ty == "u64":
        if f.u64_string:
            return {"type": "string", "pattern": "^[0-9]+$"}
        return {"type": "integer", "format": "uint64"}
    if ty.startswith("serde_json::"):
        return {}
    return {"$ref": f"#/components/schemas/{ty}"}


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
