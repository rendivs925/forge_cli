## Why

Forge has permanent behavioral specifications but no executable surface. There
is no `forge` binary, no command routing, no exit code contract, and no
environment diagnosis. CI and agents cannot consume Forge until the CLI
foundation exists.

## What Changes

- Introduce the `forge` binary with the full command surface defined in
  `specs/cli/commands.md`.
- Implement `forge --help`, `forge version`, and `forge doctor`.
- Implement deterministic command routing from the `cli/spec.md` and
  `cli/exit-codes.md` requirements.
- Implement the exit code contract (0-5) end to end.
- Add thin command handlers for `check`, `scan`, and `gate` that exercise the
  exit code contract without full analysis engines.

## Capabilities

### New Capabilities
- `cli/spec.md`: Forge SHALL expose a stable CLI with machine-readable output
  and deterministic exit codes.
- `cli/commands.md`: Forge SHALL expose the command surface for check, scan,
  gate, rules, tools, profile, policy, baseline, explain, fix, report, diff,
  config, doctor, cache, and version.
- `cli/exit-codes.md`: Forge SHALL expose deterministic exit codes.
- `cli/agent.md`: Forge SHALL provide machine-readable output suitable for
  agent consumption and SHALL NOT mutate source during analysis.
- `architecture/spec.md`: Forge SHALL prefer integration over reimplementation.

## Impact

- Affected specs: cli, architecture.
- Affected layers: `forge-cli` crate (CLI definition, routing, command
  handlers), `forge-core` crate (exit code and error model).
- Required approvals: none beyond the normal review.
