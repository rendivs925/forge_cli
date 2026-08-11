# Forge Configuration Specification

## Purpose

Define Forge's layered configuration model so that users can inspect and reason
about effective configuration and so that configuration remains extensible.

## Requirements

### Requirement: FORGE-CONF-001 — Hierarchical configuration

Forge SHALL support layered configuration.

The supported precedence SHALL be deterministic.

#### Scenario: Layered resolution

- GIVEN configuration exists in multiple layers
- WHEN Forge resolves configuration
- THEN Forge SHALL apply the layers in the documented precedence order

### Requirement: FORGE-CONF-002 — Effective configuration

Forge SHALL expose the effective configuration.

#### Scenario: Configuration inspection

- WHEN a user executes `forge config show`
- THEN Forge SHALL display the effective configuration

### Requirement: FORGE-CONF-003 — Configuration provenance

Forge SHALL be able to explain where an effective configuration value came
from.

#### Scenario: Configuration explanation

- WHEN a user executes `forge config explain <key>`
- THEN Forge SHALL identify the contributing configuration layers

### Requirement: FORGE-CONF-004 — Schema validation

Forge SHALL validate configuration before analysis.

#### Scenario: Invalid configuration

- GIVEN the configuration does not conform to the schema
- WHEN Forge starts an analysis
- THEN Forge SHALL report the invalid configuration
- AND SHALL identify the relevant location when possible

### Requirement: FORGE-CONF-005 — Configuration versioning

Forge configuration SHALL include a schema version that allows future schema
evolution.

#### Scenario: Schema version

- GIVEN a configuration declares a schema version
- WHEN Forge loads the configuration
- THEN Forge SHALL verify the version is supported
