# Forge CLI Specification

## Purpose

Forge SHALL provide a stable command-line interface for executing analysis,
evaluating quality policies, inspecting findings, managing configuration, and
operating the Forge environment.
## Requirements
### Requirement: FORGE-CLI-001 — Command structure

Forge SHALL expose commands for the primary software quality lifecycle.

#### Scenario: Inspecting available commands

- WHEN a user executes `forge --help`
- THEN Forge SHALL display the supported command categories

#### Scenario: Fast local validation

- WHEN a user executes `forge check`
- THEN Forge SHALL execute the configured fast or incremental analysis profile
- AND SHALL return a quality result

#### Scenario: Full repository analysis

- WHEN a user executes `forge scan`
- THEN Forge SHALL execute the configured comprehensive analysis profile

#### Scenario: Quality policy evaluation

- WHEN a user executes `forge gate`
- THEN Forge SHALL evaluate the configured quality policies against analysis results

### Requirement: FORGE-CLI-002 — Machine-readable output

Forge SHALL support machine-readable output for automation.

#### Scenario: JSON output

- WHEN a command supports JSON output
- AND the user specifies `--format json`
- THEN Forge SHALL emit valid machine-readable JSON
- AND SHALL NOT require consumers to parse human-readable terminal output

#### Scenario: SARIF output

- WHEN SARIF output is requested
- THEN Forge SHALL emit valid SARIF representing applicable findings

### Requirement: FORGE-CLI-003 — Stable exit codes

Forge SHALL expose deterministic exit codes.

#### Scenario: Quality failure

- GIVEN analysis completes successfully
- AND the configured quality gate fails
- WHEN Forge exits
- THEN Forge SHALL return the quality-gate failure exit code

#### Scenario: Analyzer execution failure

- GIVEN an analyzer cannot execute
- WHEN Forge exits
- THEN Forge SHALL return an execution failure exit code
- AND SHALL NOT report the result as a normal quality-gate failure

### Requirement: FORGE-CLI-004 — CLI errors

Forge SHALL report actionable errors.

#### Scenario: Invalid configuration

- GIVEN the repository configuration is invalid
- WHEN Forge starts
- THEN Forge SHALL identify the invalid configuration
- AND SHALL identify the relevant location when possible

### Requirement: FORGE-CLI-035 — Persisted analysis results

Forge SHALL persist the latest analysis result so that `forge gate` and
`forge report` can consume it without re-running analyzers, unless the user
explicitly requests fresh analysis.

#### Scenario: Gate after scan

- GIVEN a user runs `forge scan` successfully
- WHEN the user runs `forge gate` without re-analyzing
- THEN Forge SHALL evaluate the persisted analysis result

#### Scenario: Stale analysis

- GIVEN no analysis has been persisted in the workspace
- WHEN the user runs `forge gate`
- THEN Forge SHALL report that no analysis result is available

### Requirement: FORGE-CLI-040 — Success

A successful command SHALL exit with code 0.

#### Scenario: Passing gate

- GIVEN analysis completes successfully
- AND the configured quality gate passes
- WHEN Forge exits
- THEN Forge SHALL exit with code 0

### Requirement: FORGE-CLI-041 — Quality gate failure

A quality gate failure SHALL exit with code 1, independent of analyzer
execution health.

#### Scenario: Blocking finding

- GIVEN analysis completes successfully
- AND the configured quality gate fails
- WHEN Forge exits
- THEN Forge SHALL exit with code 1

### Requirement: FORGE-CLI-042 — Usage or configuration error

An invalid invocation or invalid configuration SHALL exit with code 2.

#### Scenario: Invalid arguments

- GIVEN a user invokes Forge with invalid arguments
- WHEN Forge exits
- THEN Forge SHALL exit with code 2

### Requirement: FORGE-CLI-043 — Tool execution error

A failure to execute an analyzer or external tool SHALL exit with code 3 and
SHALL NOT be reported as a quality gate failure.

#### Scenario: Analyzer cannot execute

- GIVEN an enabled analyzer cannot execute
- WHEN Forge exits
- THEN Forge SHALL exit with code 3

### Requirement: FORGE-CLI-044 — Internal error

An unexpected internal Forge failure SHALL exit with code 4.

#### Scenario: Unexpected failure

- GIVEN Forge encounters an unexpected internal failure
- WHEN Forge exits
- THEN Forge SHALL exit with code 4

### Requirement: FORGE-CLI-045 — Interrupted execution

An execution interrupted by the user SHALL exit with code 5.

#### Scenario: Interrupt

- GIVEN a user interrupts a running Forge command
- WHEN Forge exits
- THEN Forge SHALL exit with code 5

