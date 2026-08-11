# Declarative Rule Specification

## Purpose

Define how repositories SHALL define custom rules declaratively without
modifying Forge core.

## Requirements

### Requirement: FORGE-RULE-020 — Repository-defined rules

Repositories SHALL be able to define custom rules without modifying Forge core.

#### Scenario: Project architecture rule

- GIVEN a repository forbids direct infrastructure imports from the application
  layer
- WHEN the repository defines the rule in its Forge configuration
- THEN Forge SHALL evaluate that rule during applicable analyses

### Requirement: FORGE-RULE-021 — Rule validation

Forge SHALL validate declarative rules before execution.

#### Scenario: Invalid rule

- GIVEN a rule contains invalid syntax
- WHEN Forge loads the rule
- THEN Forge SHALL report the rule as invalid
- AND SHALL identify the relevant configuration location

### Requirement: FORGE-RULE-022 — Rule test fixtures

Custom rules SHOULD support deterministic test fixtures.

#### Scenario: Rule regression test

- GIVEN a rule has valid and invalid fixtures
- WHEN the rule test command executes
- THEN Forge SHALL verify expected findings against the fixtures
