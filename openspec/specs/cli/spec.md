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
