# Reporting Specification

## Purpose

Define how Forge SHALL render analysis results across human and machine
consumers without coupling reporters to analyzers.

## Requirements

### Requirement: FORGE-REP-001 — Unified reporting model

Reporters SHALL consume the normalized Forge analysis model.

#### Scenario: Reporter input

- GIVEN a reporter executes
- THEN the reporter SHALL consume the normalized analysis model
- AND SHALL NOT parse analyzer-specific output directly

### Requirement: FORGE-REP-002 — Human-readable output

Forge SHALL provide concise terminal output suitable for developers.

#### Scenario: Terminal report

- WHEN a user requests terminal output
- THEN Forge SHALL render concise human-readable results

### Requirement: FORGE-REP-003 — Machine-readable output

Forge SHALL support machine-readable reporting formats.

#### Scenario: Structured report

- WHEN a user requests a machine-readable format
- THEN Forge SHALL emit valid structured output

### Requirement: FORGE-REP-004 — Reporter independence

Adding a reporter SHALL NOT require modifying analyzer implementations.

#### Scenario: New reporter

- GIVEN a new reporter is added
- THEN existing analyzers SHALL NOT require modification

### Requirement: FORGE-REP-005 — Report reproducibility

Reports SHALL contain sufficient metadata to identify the analysis context,
including Forge version, configuration, and applicable analyzer versions.

#### Scenario: Context metadata

- GIVEN a report is generated
- THEN the report SHALL include the Forge version, configuration identity, and
  analyzer versions used
