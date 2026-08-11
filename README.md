# Forge

A general-purpose **software quality control plane** written in Rust. Forge orchestrates
existing development, security, testing, dependency, and static analysis tools through
adapters, providing a unified model for findings, rules, policies, quality gates,
configuration, and reporting.

Forge prefers **integration over reimplementation**: it normalizes outputs from external
analyzers into a common finding model rather than rebuilding what mature tools already do.

## Status

Early development. The CLI foundation and configuration layer are in place:

| Command | Status |
|---------|--------|
| `forge init` | stub pass |
| `forge check` | resolves config, stub pass |
| `forge scan` | resolves config, stub pass |
| `forge gate` | resolves config, stub pass |
| `forge doctor` | environment diagnosis |
| `forge version` | version info |
| `forge config show` | effective configuration |
| `forge config explain <key>` | provenance by layer |
| `forge rules`, `forge tools`, `forge profile`, `forge policy`, `forge baseline`, `forge explain`, `forge fix`, `forge report`, `forge diff`, `forge cache` | not yet implemented |

Output formats: terminal and `--format json`. Exit codes follow the contract: `0` success, `1` quality gate failure, `2` usage/configuration error, `3` tool execution error, `4` internal error, `5` interrupted.

## Building

```bash
cargo build --release
```

## Running

```bash
# Show effective configuration
forge config show

# Inspect a configuration key
forge config explain profile

# Use a project config file
forge config show --config ./forge.toml --workspace .

# Fast local check
forge check

# Full analysis
forge scan

# Evaluate quality gate
forge gate
```

## Architecture

Forge is a multi-crate workspace:

- **`crates/forge-core`** — exit code model, error types, application context
- **`crates/forge-cli`** — the `forge` binary; CLI definition, routing, command handlers
- **`crates/forge-config`** — layered TOML configuration with schema versioning, provenance, and validation

Behavioral requirements are governed by **OpenSpec** specs in `openspec/specs/`. Changes are developed as discrete proposals under `openspec/changes/` and archived into the permanent spec tree after implementation.

## Design Principles

- Forge core is **tool-agnostic** and **organization-agnostic**
- Analyzers integrate via adapters; output is normalized into a common finding model
- Configuration is layered: built-in defaults → global user config → project config → CLI flags
- Specifications define externally observable behavior, not implementation details
