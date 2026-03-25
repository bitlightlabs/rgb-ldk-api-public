# rgb-ldk-api (generated mirror repo)

This repository is a permission-friendly mirror of `rgb-ldk-node` API artifacts plus generated clients.

Current workflow (manual, no CI/registries):

1) Sync generated artifacts from `rgb-ldk-node` into this repo:

```bash
python3 scripts/rgbldk_api_sync.py sync --node-repo ~/work/rgb-ldk-node --git-commit
```

2) Generate DTO types + clients from the synced Rust DTO source:

```bash
python3 scripts/rgb-ldk-api.py gen
```

3) View the synced OpenAPI doc (Redoc):

- open `docs/openapi.html` (it reads `generated/spec/openapi.json`)

Notes:

- `generated/` is overwritten by the sync step, including `generated/spec/openapi.json`.
- `python3 scripts/rgbldk_api_sync.py sync` defaults `--api-repo` to the current repo, and stages both `generated/` and `crates/rgbldk_http_dto/` when `--git-commit` is used.
- `packages/*/src/generated.ts` and `crates/*/src/generated.rs` are overwritten by the gen step.
- `python3 scripts/rgb-ldk-api.py gen` no longer rebuilds OpenAPI; it consumes the synced spec and also sanitizes `generated/spec/source.json` (you can run `python3 scripts/rgb-ldk-api.py sanitize` standalone).
- u64 values that are serialized as decimal strings in JSON are represented as `U64String` in TS.
