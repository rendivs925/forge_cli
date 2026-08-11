# AI Agent Integration Specification

## Purpose

Define the Forge interface for programmatic and AI-agent consumption so that
coding agents can use Forge as a verification feedback loop.

## Requirements

### Requirement: FORGE-CLI-060 — Machine-readable analysis

Forge SHALL provide analysis output suitable for programmatic consumption.

#### Scenario: Programmatic output

- WHEN an agent requests machine-readable output
- THEN Forge SHALL emit structured output that does not require parsing
  human-readable terminal text

### Requirement: FORGE-CLI-061 — Actionable findings

Findings exposed to agents SHALL include stable identifiers, locations,
severity, descriptions, and remediation information where available.

#### Scenario: Structured finding

- GIVEN an analysis produces a finding
- WHEN Forge emits agent output
- THEN Forge SHALL include the finding's identifier, location, severity,
  description, and remediation guidance where available

### Requirement: FORGE-CLI-062 — Explicit verification

Agents SHALL be able to execute Forge after modifications to determine whether
the implementation satisfies configured quality policies.

#### Scenario: Post-change verification

- WHEN an agent executes Forge after modifying code
- THEN Forge SHALL report whether the change satisfies the configured quality
  policies

### Requirement: FORGE-CLI-063 — No hidden mutation

Analysis commands SHALL NOT modify source code unless the user explicitly
invokes a mutation command such as `forge fix`.

#### Scenario: Analysis is read-only

- GIVEN an analysis command executes
- THEN Forge SHALL NOT modify source files

### Requirement: FORGE-CLI-064 — Deterministic verification

The same repository state SHALL produce equivalent machine-readable analysis
results under equivalent configuration and tool versions.

#### Scenario: Repeat execution

- GIVEN the repository state, configuration, and tool versions are unchanged
- WHEN Forge runs the same command twice
- THEN Forge SHALL produce equivalent machine-readable results
