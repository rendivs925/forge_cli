# Forge Architecture Specification

## Purpose

Forge SHALL provide a general-purpose software quality control plane that
orchestrates existing development, security, testing, dependency, and static
analysis tools while providing a unified model for findings, rules, policies,
quality gates, configuration, reporting, and project-specific extensions.

Forge SHALL prefer integration over reimplementation.

Forge SHALL remain independent from any individual programming language,
analysis tool, organization, repository, or project architecture.

## Requirements

### Requirement: FORGE-ARCH-001 — Tool integration over tool reimplementation

Forge SHALL integrate existing analysis tools through adapters rather than
reimplementing capabilities already provided by mature external tools.

#### Scenario: Existing analyzer provides a capability

- GIVEN an external analyzer provides a required analysis capability
- WHEN Forge integrates that capability
- THEN Forge SHALL prefer an adapter over implementing an equivalent analyzer
- AND the analyzer output SHALL be normalized into the Forge finding model

#### Scenario: Analyzer is unavailable

- GIVEN an enabled analyzer is not available
- WHEN an analysis is executed
- THEN Forge SHALL report the missing dependency explicitly
- AND SHALL distinguish analyzer execution failure from quality gate failure

### Requirement: FORGE-ARCH-002 — Core independence

Forge core SHALL NOT contain project-specific, organization-specific, or
tool-specific business logic.

#### Scenario: Adding a project-specific rule

- GIVEN a repository requires a custom architectural rule
- WHEN the rule is added
- THEN the repository SHALL be able to define the rule without modifying Forge core

#### Scenario: Adding an analyzer

- GIVEN a new external analyzer must be integrated
- WHEN an adapter is implemented
- THEN unrelated Forge components SHALL NOT require modification

### Requirement: FORGE-ARCH-003 — Stable domain contracts

Forge SHALL define stable contracts for analyzers, findings, rules, policies,
quality gates, configuration, and reporters.

#### Scenario: Analyzer replacement

- GIVEN two analyzers provide equivalent capabilities
- WHEN one analyzer is replaced
- THEN the quality engine SHALL continue operating against the normalized
  finding model

### Requirement: FORGE-ARCH-004 — Behavior-first architecture

Specifications SHALL define externally observable behavior and contracts rather
than prescribing unnecessary implementation details.

#### Scenario: Implementation changes

- GIVEN the internal implementation of an analyzer changes
- WHEN observable behavior remains compliant
- THEN the implementation SHALL remain valid against the specification
