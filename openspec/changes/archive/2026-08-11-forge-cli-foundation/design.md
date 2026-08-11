# Design

## Context

The repository is a fresh Rust project with a single binary crate. The CLI
foundation change establishes the executable surface, command routing, and
exit code contract. See proposal.md and the cli spec deltas for behavioral
requirements.

## Goals / Non-Goals

**Goals:**
- A `forge` binary whose `--help` output and `version` command are stable.
- Deterministic routing of every command in the CLI surface.
- The exit code contract (0-5) implemented end to end.
- A `doctor` command that reports environment, repository, and tool health
  without requiring analyzers.

**Non-Goals:**
- Implementing analysis engines, finding models, or quality gates.
- Integrating real external analyzers.
- Implementing configuration resolution or baseline.
- Full agent output format (deferred to `forge-agent-interface`).

## Decisions

### CLI framework
Use `clap` derive API for the CLI surface. The `Parser`, `Subcommand`,
`Args`, and `ValueEnum` derives model subcommands and reusable argument groups
as typed Rust types. CLI code remains thin; command logic lives in per-command
modules.

### Command routing
A single async `run(cli)` function matches on the parsed subcommand and
delegates to `commands::<name>::run(context, args)`. No analyzer logic lives
in the CLI definition.

### Exit code model
An `ExitCode` type models the contract from `cli/exit-codes.md`:

```text
0  success
1  quality gate failed
2  usage/configuration error
3  tool execution error
4  internal Forge error
5  interrupted
```

Command handlers return `Result<ExitCode, ForgeError>`; the error type maps to
a specific exit code category so CI can distinguish quality failure (1) from
execution failure (3).

### Workspace layout
Introduce the multi-crate workspace from the architecture plan:

```text
crates/forge-cli      binary: CLI definition, routing, command handlers
crates/forge-core     exit code model, error types, application context
```

Additional crates are added by later changes (config, engine, adapters, etc.);
this change only creates what the CLI foundation needs.

### Doctor without analyzers
`forge doctor` reports the repository state (git repo, project type) and
Forge environment (version, config presence, cache writability). Analyzer
health reporting is deferred until the analyzer framework exists.

### Machine-readable output
Every command that can report results accepts `--format json` and emits
valid JSON rather than requiring terminal scraping. The `ExitCode` and report
types serialize with `serde`.

## Concurrency

Not applicable to this change; no concurrent analyzer execution exists yet.

## Error model

`ForgeError` is an enum with variants mapping to exit code categories:

- Usage/config error -> 2
- Tool execution error -> 3
- Internal error -> 4

Interrupted (5) is produced by signal handling in the binary entrypoint.

## Non-goals

Forge will not reimplement a CLI framework.
Forge will not implement analysis in this change.
Forge will not parse analyzer output in this change.
Forge will not modify user hooks or source files in this change.
