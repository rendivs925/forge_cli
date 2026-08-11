# Rule Testing Specification

## Purpose

Define how Forge SHALL verify declarative and built-in rules against
deterministic fixtures so that rule behavior is regression-tested.

## Requirements

### Requirement: FORGE-RULE-040 — Deterministic rule verification

Forge SHALL verify rule behavior against deterministic fixtures.

#### Scenario: Expected findings match

- GIVEN a rule has fixtures marked valid and invalid
- WHEN the rule test executes
- THEN Forge SHALL report whether the rule produced the expected findings

### Requirement: FORGE-RULE-041 — Rule test reporting

Rule test results SHALL identify each fixture and whether it passed or failed.

#### Scenario: Fixture failure

- GIVEN a fixture that should produce a finding does not
- WHEN the rule test executes
- THEN Forge SHALL report the fixture as failed
- AND SHALL identify the rule and fixture
