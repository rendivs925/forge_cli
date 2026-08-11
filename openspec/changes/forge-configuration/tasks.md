# Tasks

## 1. Configuration crate

- [ ] 1.1 Add `crates/forge-config` to the workspace with `serde`, `toml`, and `forge-core` dependencies.
- [ ] 1.2 Define `ForgeConfig` with a required `schema` key and `#[serde(default)]` sections.
- [ ] 1.3 Implement `ConfigError` with source location and typed validation errors.

## 2. Loading and validation

- [ ] 2.1 Implement loading of a `forge.toml` from a path, mapping TOML/deserialization errors to `ConfigError`.
- [ ] 2.2 Implement schema version verification against the supported version (1).
- [ ] 2.3 Implement validation of the merged configuration with offending-key reporting.

## 3. Layered resolution

- [ ] 3.1 Implement `ConfigResolver` with precedence: defaults, global user config, project config, CLI flags.
- [ ] 3.2 Record per-key provenance (contributing layers) during resolution.
- [ ] 3.3 Wire the `--config` path and global flags into resolution.

## 4. CLI integration

- [ ] 4.1 Implement `forge config show` (terminal and JSON output).
- [ ] 4.2 Implement `forge config explain <key>` reporting contributing layers.
- [ ] 4.3 Route `check`, `scan`, and `gate` through the config resolver so they validate config before executing.

## 5. Tests and quality gates

- [ ] 5.1 Add unit tests for loading, schema version, defaults, precedence, and provenance.
- [ ] 5.2 Add integration tests for `forge config show` and `forge config explain`.
- [ ] 5.3 Run formatting, compilation, unit tests, integration tests, linting, and OpenSpec validation.
