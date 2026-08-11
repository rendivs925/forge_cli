# Forge Exit Codes

## Purpose

Define the deterministic exit codes Forge SHALL expose so that CI systems can
distinguish quality failure from execution failure and configuration errors.

## Exit code table

```text
0  success
1  quality gate failed
2  usage/configuration error
3  tool execution error
4  internal Forge error
5  interrupted
```

## Requirements

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

#### Scenario: Invalid configuration

- GIVEN the repository configuration is invalid
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
