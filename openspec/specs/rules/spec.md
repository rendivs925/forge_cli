# Rules Specification

## Purpose

Define the unified rule model that Forge SHALL use to expose analyzer rules and
custom rules through one discoverable interface.

## Requirements

### Requirement: FORGE-RULE-001 — Rule identity

Every Forge rule SHALL have a stable unique identifier.

#### Scenario: Rule reference

- GIVEN a rule has ID `security.sql-injection`
- WHEN a user references the rule
- THEN Forge SHALL resolve that rule deterministically

### Requirement: FORGE-RULE-002 — Rule metadata

Rules SHALL expose metadata including category, severity, description, and
applicability.

#### Scenario: Rule metadata

- GIVEN a rule exists
- THEN Forge SHALL expose its category, severity, description, and
  applicability

### Requirement: FORGE-RULE-003 — Rule enablement

Users SHALL be able to enable or disable rules through configuration.

#### Scenario: Disabled rule

- GIVEN a rule is disabled in configuration
- WHEN analysis executes
- THEN Forge SHALL NOT report findings for the disabled rule

### Requirement: FORGE-RULE-004 — Rule severity override

Users SHALL be able to override rule severity where policy permits.

#### Scenario: Severity override

- GIVEN a user overrides a rule's severity
- THEN Forge SHALL apply the overridden severity to the rule's findings

### Requirement: FORGE-RULE-005 — Rule packs

Rules SHALL be groupable into reusable rule packs.

#### Scenario: Security profile

- GIVEN a security rule pack exists
- WHEN a profile extends the security rule pack
- THEN all rules included by the pack SHALL become available according to
  their configuration

### Requirement: FORGE-RULE-006 — Rule isolation

A rule SHALL NOT require unrelated rules to execute successfully.

#### Scenario: Independent rules

- GIVEN one rule fails to evaluate
- THEN unrelated rules SHALL continue to produce results
