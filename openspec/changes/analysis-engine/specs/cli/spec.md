# Forge CLI — Analysis Deltas

## ADDED Requirements

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
