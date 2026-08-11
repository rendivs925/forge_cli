# Baseline Specification

## Purpose

Define how Forge SHALL manage existing technical debt so that mature
repositories can adopt Forge without immediately blocking on historical
findings.

## Requirements

### Requirement: FORGE-BASE-001 — Existing debt baseline

Forge SHALL support establishing a baseline of existing findings.

#### Scenario: Baseline creation

- WHEN a user creates a baseline
- THEN Forge SHALL record the current findings as the baseline

### Requirement: FORGE-BASE-002 — New debt detection

Findings introduced after baseline creation SHALL be distinguishable from
baseline findings.

#### Scenario: New finding

- GIVEN a finding is introduced after baseline creation
- THEN Forge SHALL classify it as a new finding distinct from the baseline

### Requirement: FORGE-BASE-003 — Stable fingerprints

Baseline matching SHALL use stable finding fingerprints.

#### Scenario: Stable matching

- GIVEN a finding remains at the same logical location
- THEN Forge SHALL match it to the baseline using its stable fingerprint

### Requirement: FORGE-BASE-004 — Baseline updates

Users SHALL be able to intentionally update the baseline.

#### Scenario: Debt cleanup

- GIVEN an existing finding is fixed
- WHEN the baseline is regenerated
- THEN the resolved finding SHALL no longer remain in the baseline

### Requirement: FORGE-BASE-005 — Baseline transparency

Forge SHALL report when findings are suppressed because they belong to the
baseline.

#### Scenario: Baseline suppression report

- GIVEN a finding is suppressed as baseline debt
- THEN Forge SHALL report the suppression explicitly
