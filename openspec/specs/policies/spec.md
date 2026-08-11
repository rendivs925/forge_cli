# Policy Specification

## Purpose

Define how Forge SHALL represent and evaluate quality policies, including
organization and repository policies, so that policy decisions are
deterministic and explainable.

## Requirements

### Requirement: FORGE-POL-001 — Composable policies

Policies SHALL be composable from multiple conditions.

#### Scenario: Multiple conditions

- GIVEN a policy combines severity and category conditions
- WHEN the policy evaluates
- THEN Forge SHALL apply all conditions to the decision

### Requirement: FORGE-POL-002 — Organization policies

Organizations SHALL be able to define policies shared across repositories.

#### Scenario: Shared organization policy

- GIVEN an organization defines a policy
- THEN repositories governed by the organization SHALL inherit the policy

### Requirement: FORGE-POL-003 — Repository policies

Repositories SHALL be able to define stricter policies than their inherited
organization policies where permitted.

#### Scenario: Stricter repository policy

- GIVEN an organization policy allows a threshold
- AND the repository policy is stricter
- THEN the repository policy SHALL apply

### Requirement: FORGE-POL-004 — Policy precedence

Policy inheritance and overrides SHALL be deterministic.

#### Scenario: Deterministic resolution

- GIVEN multiple policy layers define the same condition
- WHEN Forge resolves the policy
- THEN Forge SHALL apply the documented precedence

### Requirement: FORGE-POL-005 — Policy explanation

Forge SHALL provide an explanation for every quality gate decision.

#### Scenario: Gate decision explanation

- WHEN a quality gate produces a decision
- THEN Forge SHALL identify the policy that drove the decision
