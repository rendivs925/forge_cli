# Quality Gate Specification

## Purpose

Define the enforcement mechanism Forge SHALL use to decide whether a state of
the codebase may be merged, distinguishing newly introduced debt from
pre-existing baseline debt.

## Requirements

### Requirement: FORGE-GATE-001 — Policy-based gates

Quality gates SHALL evaluate normalized analysis results against configurable
policies.

#### Scenario: Gate evaluation

- GIVEN analysis results and a configured policy
- WHEN the quality gate executes
- THEN Forge SHALL evaluate the results against the policy

### Requirement: FORGE-GATE-002 — New finding enforcement

Policies SHALL be able to evaluate newly introduced findings independently
from pre-existing findings.

#### Scenario: Existing technical debt

- GIVEN a repository contains existing major findings
- AND those findings are part of the baseline
- WHEN new code introduces no additional major findings
- THEN the quality gate MAY pass

### Requirement: FORGE-GATE-003 — Severity thresholds

Policies SHALL support severity-based thresholds.

#### Scenario: Zero blockers

- GIVEN the configured blocker threshold is zero
- AND analysis contains one new blocker
- WHEN the quality gate executes
- THEN the gate SHALL fail

### Requirement: FORGE-GATE-004 — Category policies

Policies SHALL support category-specific thresholds.

#### Scenario: Category threshold

- GIVEN a category has a configured threshold
- AND the category count exceeds the threshold
- WHEN the quality gate executes
- THEN the gate SHALL fail

Examples of categories include:

- security
- reliability
- maintainability
- architecture
- performance
- supply chain

### Requirement: FORGE-GATE-005 — Deterministic decision

Given identical analysis results and policy configuration, the quality gate
SHALL produce the same result.

#### Scenario: Repeat decision

- GIVEN identical results and configuration
- WHEN the gate executes twice
- THEN Forge SHALL produce the same decision

### Requirement: FORGE-GATE-006 — Explainable failure

When a quality gate fails, Forge SHALL identify the policy that failed and the
findings responsible for the failure.

#### Scenario: Failed gate

- GIVEN the quality gate fails
- THEN Forge SHALL identify the failing policy
- AND SHALL identify the responsible findings
