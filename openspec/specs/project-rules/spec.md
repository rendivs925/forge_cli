# Project Rule Specification

## Purpose

Define how Forge SHALL support repository-local and organization rules without
contaminating Forge core with project-specific behavior.

## Requirements

### Requirement: FORGE-PROJ-001 — Repository-local rules

Forge SHALL allow repositories to define rules that apply only to that
repository.

#### Scenario: Repository rule

- GIVEN a repository defines a local rule
- THEN Forge SHALL apply the rule to that repository's analyses

### Requirement: FORGE-PROJ-002 — Organization rules

Forge SHALL support rules shared across multiple repositories.

#### Scenario: Organization rule

- GIVEN an organization defines a shared rule
- THEN repositories governed by the organization SHALL inherit the rule

### Requirement: FORGE-PROJ-003 — Rule precedence

When multiple configuration layers define the same rule, Forge SHALL resolve
the effective configuration deterministically.

#### Scenario: Project overrides organization

- GIVEN an organization defines a rule as `major`
- AND a repository overrides it as `blocker`
- WHEN configuration is resolved
- THEN the repository configuration SHALL take precedence according to the
  configured precedence model

### Requirement: FORGE-PROJ-004 — No core modification

Adding or changing repository-local rules SHALL NOT require modifying Forge
source code.

#### Scenario: Rule without source change

- GIVEN a repository adds a local rule
- THEN Forge SHALL evaluate the rule without a Forge source change
