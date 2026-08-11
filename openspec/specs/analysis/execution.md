# Analyzer Execution Specification

## Purpose

Define how Forge executes analyzers as external processes and how it represents
execution outcomes independently from findings.

## Requirements

### Requirement: FORGE-ENG-040 — Process-based execution

Forge SHALL execute external analyzers as processes and SHALL NOT embed third
party analyzer code into the Forge core.

#### Scenario: External invocation

- GIVEN an analyzer is an external executable
- WHEN Forge runs the analyzer
- THEN Forge SHALL invoke the executable with the configured arguments
- AND SHALL capture its output

### Requirement: FORGE-ENG-041 — Execution outcome representation

Analyzer execution outcomes SHALL be represented separately from analysis
findings.

#### Scenario: Execution failure

- GIVEN an analyzer fails to execute
- WHEN Forge records the outcome
- THEN Forge SHALL record an execution failure distinct from any findings

### Requirement: FORGE-ENG-042 — Timeout handling

Forge SHALL enforce a timeout on analyzer execution.

#### Scenario: Timeout exceeded

- GIVEN an analyzer exceeds its configured timeout
- WHEN Forge records the outcome
- THEN Forge SHALL report the analyzer as failed with a timeout reason

### Requirement: FORGE-ENG-043 — Version recording

Forge SHALL record the analyzer version used for each execution.

#### Scenario: Version metadata

- GIVEN an analysis executes
- THEN Forge SHALL record the version of each analyzer that produced results
