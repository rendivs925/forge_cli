# Tasks

- [x] Initialize Cargo workspace with `crates/forge-cli` and `crates/forge-core`.
- [x] Add `clap` and `serde` dependencies.
- [x] Define the full `Command` enum matching `cli/commands.md` surface.
- [x] Define global flags (`-q`, `-v`, `--no-color`, `--format`, `--config`, `--profile`, `--workspace`, `--offline`, `--no-cache`, `--fail-fast`, `--timings`).
- [x] Implement `ExitCode` model per `cli/exit-codes.md`.
- [x] Implement `ForgeError` with exit-code category mapping.
- [x] Implement `forge --help` output.
- [x] Implement `forge version`.
- [x] Implement thin handlers for `check`, `scan`, `gate` exercising exit code contract.
- [x] Implement `forge doctor` (repo + Forge environment checks).
- [x] Implement `--format json` output path for result-bearing commands.
- [x] Add unit tests for exit code mapping.
- [x] Add integration tests for `forge --help`, `forge version`, exit codes.
- [x] Run formatting, compilation, unit tests, integration tests, linting, and OpenSpec validation.
