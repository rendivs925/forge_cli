# Design

## Context

Forge currently has no configuration layer: `check`, `scan`, and `gate` report
hardcoded passes and `forge config show` / `config explain` are unimplemented.
See proposal.md for the motivation and scope.

## Goals / Non-Goals

**Goals:**
- A `forge-config` crate that loads `forge.toml` (TOML) from the workspace
  root, validates its schema version, and exposes the effective configuration.
- Deterministic layer precedence: defaults < global < project < CLI flags.
- `forge config show` and `forge config explain <key>` implemented against the
  resolved configuration with provenance.
- Configuration resolved and validated before `check`, `scan`, and `gate`
  execute.

**Non-Goals:**
- Executing analyzers or producing findings (later change).
- Multiple user-config file discovery beyond the standard global path.
- Dynamic hot-reload of configuration.

## Decisions

### New crate `crates/forge-config`
Configuration is a shared domain that `forge-cli` and future engine crates
will both consume, so it lives in its own crate under the workspace rather
than inside `forge-cli`. It depends only on `serde` and `toml`.

### Configuration file contract
`forge.toml` at the workspace root declares a `schema` key (an integer). The
`ForgeConfig` struct uses `#[serde(default)]` so omitted sections fall back to
built-in defaults; the `schema` key is required and validated against the
supported version (`1`). Deserialization errors and unsupported schema
versions produce a `ConfigError` carrying the source location when available.

### Layered resolution
A `ConfigResolver` merges layers in fixed precedence order:

1. Built-in defaults (`ForgeConfig::default()`).
2. Global user configuration (`~/.config/forge/forge.toml` if present).
3. Project configuration (`<workspace>/forge.toml` or the `--config` path).
4. CLI flags (profile selection, workspace root, offline, cache, etc.).

The merger overlays each higher-precedence layer onto the current value field
by field. The resolution provenance is recorded per top-level key as the list
of contributing layers, which `config explain <key>` reports.

### Validation before analysis
`ConfigResolver::resolve()` validates the merged result and fails fast with a
`ConfigError::Invalid` naming the offending key and its source file before any
command proceeds. `check`, `scan`, and `gate` call the resolver through a
shared helper so validation happens once at the boundary.

### CLI integration
`ConfigArgs` subcommands (`show`, `explain`) are routed to handlers that read
the resolved configuration. `show` renders effective configuration (terminal
or JSON); `explain <key>` renders the contributing layers for that key.

## Risks / Trade-offs

- Schema evolution → The `schema` version gate keeps future breaking changes
  explicit; unsupported versions are reported as configuration errors.
- TOML vs other formats → TOML matches the ecosystem default (Cargo) and is
  the format `doctor` already expects (`forge.toml`).
- `--config` path vs workspace root discovery → The explicit `--config` flag
  takes precedence; otherwise the workspace root `forge.toml` is used.

## Migration Plan

No existing configuration exists, so there is nothing to migrate. Rollback is
a revert of the change commit.

## Open Questions

None.
