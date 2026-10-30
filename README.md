# rgb-ldk-api

This repository copies the request and response definitions from `rgb-ldk-node`
and generates Rust and TypeScript clients from them.

TypeScript callers use `RgbLdkNodeClient.getJson`, `postJson`, and `deleteJson`
with the generated request and response types, including for swaps.

- [Swap frontend integration guide](docs/swap-frontend-guide.md): quote confirmation,
  execution identifiers, routes, fees, recovery, errors, and migration.
- [TypeScript swap examples](examples/swap.ts) and
  [complete response examples used by tests](tests/fixtures/README.md).

## Update the generated files

Run these steps manually; the packages are not published to a package registry.

1. Copy definitions and the OpenAPI document from the node repository:

   ```bash
   python3 scripts/rgbldk_api_sync.py sync --node-repo ~/work/rgb-ldk-node --git-commit
   ```

2. Generate types and clients from the copied Rust definitions:

   ```bash
   python3 scripts/rgb-ldk-api.py gen
   ```

3. Open `docs/openapi.html` to view the OpenAPI document. It reads
   `generated/spec/openapi.json`.

The synchronization step overwrites `generated/`, including the OpenAPI document.
It defaults `--api-repo` to the current repository. With `--git-commit`, it stages
both `generated/` and `crates/rgbldk_http_dto/` for the commit. The generation step
overwrites `packages/*/src/generated.ts` and `crates/*/src/generated.rs`.

Synchronization copies swap plan types from the same upstream commit and adds
their complete OpenAPI field definitions. `generated/spec/source.json` records
the source revision and these additions. Generation uses the copied OpenAPI
document and removes local path details from `source.json`; this cleanup can
also be run separately with `python3 scripts/rgb-ldk-api.py sanitize`.

Unsigned 64-bit values serialized as decimal strings are represented by
`U64String` in TypeScript to preserve their precision.

## Run checks

```bash
pnpm test:codegen
pnpm test
pnpm typecheck
pnpm typecheck:examples
cargo test --workspace
```
