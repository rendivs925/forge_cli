# Tasks

- [ ] Initialize Cargo workspace with `crates/forge-cli` and `crates/forge-core`.
- [ ] Add `clap` and `serde` dependencies.
- [ ] Define the full `Command` enum matching `cli/commands.md` surface.
- [ ] Define global flags (`-q`, `-v`, `--no-color`, `--format`, `--config`, `--profile`, `--workspace`, `--offline`, `--no-cache`, `--fail-fast`, `--timings`).
- [ ] Implement `ExitCode` model per `cli/exit-codes.md`.
- [ ] Implement `ForgeError` with exit-code category mapping.
- [ ] Implement `forge --help` output.
- [ ] Implement `forge version`.
- [ ] Implement thin handlers for `check`, `scan`, `gate` exercising exit code contract.
- [ ] Implement `forge doctor` (repo + Forge environment checks).
- [ ] Implement `--format json` output path for result-bearing commands.
- [ ] Add unit tests for exit code mapping.
- [ ] Add integration tests for `forge --help`, `forge version`, exit codes.
- [ ] Run formatting, compilation, unit tests, integration tests, linting, and OpenSpec validation.
