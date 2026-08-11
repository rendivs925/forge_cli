# Analyzer Adapter Specification

## Purpose

Define the common contract Forge SHALL expose for integrating external
analyzers without coupling the core to any specific tool.

## Requirements

### Requirement: FORGE-ENG-020 — Analyzer abstraction

Forge SHALL expose a common analyzer contract.

#### Scenario: External analyzer integration

- GIVEN an external analyzer is integrated with Forge
- THEN the integration SHALL expose its identity
- AND capabilities
- AND supported languages or project types
- AND execution requirements
- AND normalized analysis results

### Requirement: FORGE-ENG-021 — Tool discovery

Forge SHALL detect whether required external tools are available.

#### Scenario: Missing executable

- GIVEN an analyzer requires an executable
- AND the executable is unavailable
- WHEN `forge doctor` executes
- THEN Forge SHALL identify the missing executable

### Requirement: FORGE-ENG-022 — Version compatibility

Adapters SHALL be able to declare supported tool versions.

#### Scenario: Unsupported version

- GIVEN an installed tool version is outside the supported range
- WHEN Forge validates the environment
- THEN Forge SHALL report the incompatibility before analysis when possible
