# Finding Specification

## Purpose

Define the unified finding model that Forge SHALL use to represent results from
any analyzer, enabling cross-tool analysis, correlation, and quality gates.

## Requirements

### Requirement: FORGE-FIND-001 — Unified finding representation

Forge SHALL normalize findings from different analyzers into a common model.

#### Scenario: Clippy finding

- GIVEN Clippy reports a warning
- WHEN Forge processes the result
- THEN Forge SHALL represent it as a Forge finding

#### Scenario: Semgrep finding

- GIVEN Semgrep reports a security finding
- WHEN Forge processes the result
- THEN Forge SHALL represent it using the same finding contract

### Requirement: FORGE-FIND-002 — Finding identity

Each finding SHALL have a stable identity suitable for tracking across
analysis runs.

#### Scenario: Finding persists between runs

- GIVEN a finding remains at the same logical location
- WHEN a subsequent analysis is performed
- THEN Forge SHOULD preserve its identity when the underlying fingerprint
  remains stable

### Requirement: FORGE-FIND-003 — Severity

Findings SHALL have normalized severity independent of the source analyzer.

#### Scenario: Normalized severity

- GIVEN two analyzers report findings of different native severities
- WHEN Forge processes the results
- THEN both findings SHALL have comparable normalized severity

### Requirement: FORGE-FIND-004 — Location

Findings SHALL identify their source location when the originating analyzer
provides location information.

#### Scenario: Located finding

- GIVEN an analyzer provides a file and line
- THEN Forge SHALL record the finding's source location

### Requirement: FORGE-FIND-005 — Remediation

Findings SHALL make remediation guidance and automatic-fix metadata available
when the originating analyzer provides it.

#### Scenario: Remediation guidance

- GIVEN a finding has remediation guidance
- THEN Forge SHALL make the guidance available to consumers
