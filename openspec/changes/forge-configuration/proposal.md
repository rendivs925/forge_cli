## Why

Forge's check, scan, and gate commands are hardcoded passes because no
configuration layer exists: there is no `forge.toml` loading, no schema
versioning, no layered resolution, and `forge config show` / `config explain`
return "not yet implemented". Users cannot configure Forge, and agents cannot
inspect effective configuration.

## What Changes

- Introduce a configuration crate (`forge-config`) that loads a TOML
  `forge.toml` from the workspace root with a declared schema version.
- Implement deterministic layered resolution: built-in defaults, global user
  config, project config, and CLI flags.
- Implement schema validation that reports invalid configuration with its
  location before any analysis starts.
- Implement `forge config show` (effective configuration) and
  `forge config explain <key>` (provenance) end to end.
- Wire configuration loading into the `check`, `scan`, and `gate` entrypoints
  so they resolve and validate config instead of reporting hardcoded passes.

## Capabilities

### New Capabilities
- `configuration`: layered TOML configuration with schema versioning, effective
  configuration resolution, provenance, and validation.

### Modified Capabilities
None.

## Impact

- Affected specs: configuration, cli.
- Affected layers: new `crates/forge-config` crate; `forge-cli` command routing
  and `check`/`scan`/`gate` handlers; `forge-core` unchanged.
- New dependency: `toml` (and `serde` in forge-config).
- Required approvals: none beyond the normal review.
