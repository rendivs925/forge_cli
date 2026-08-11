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

### Requirement: FORGE-CONF-006 — Configuration file location

Forge SHALL read project configuration from a `forge.toml` file located at the
workspace root, and SHALL load it in TOML format.

#### Scenario: Loading project configuration

- GIVEN a `forge.toml` exists at the workspace root
- WHEN Forge resolves configuration
- THEN Forge SHALL load the file as project configuration

#### Scenario: Missing project configuration

- GIVEN no `forge.toml` exists at the workspace root
- WHEN Forge resolves configuration
- THEN Forge SHALL fall back to the remaining configuration layers

### Requirement: FORGE-CONF-007 — Layer precedence

Forge SHALL apply configuration layers in a fixed, documented order from lowest
to highest precedence: built-in defaults, global user configuration, project
configuration, then CLI flags.

#### Scenario: Defaults and overrides

- GIVEN configuration is present in multiple layers
- WHEN Forge resolves a configuration value
- THEN Forge SHALL use the value from the highest-precedence layer that
  defines it

#### Scenario: CLI flag override

- GIVEN a project configuration defines a value
- AND the user supplies a corresponding CLI flag
- WHEN Forge resolves the value
- THEN Forge SHALL prefer the CLI flag value

### Requirement: FORGE-CONF-008 — Default configuration

Forge SHALL provide built-in default configuration so analysis can proceed
without any user-provided configuration files.

#### Scenario: No configuration files present

- GIVEN neither global nor project configuration exists
- WHEN Forge resolves configuration
- THEN Forge SHALL use the built-in defaults
- AND SHALL NOT fail because configuration is absent

### Requirement: FORGE-CONF-010 — Analysis profile configuration

Forge SHALL support named analysis profiles in configuration, and SHALL select
the profile named by the effective `profile` value for check and scan
execution.

#### Scenario: Default profile

- GIVEN a configuration with no explicit profile
- WHEN `forge check` executes
- THEN Forge SHALL use the built-in default profile

#### Scenario: Named profile

- GIVEN a configuration with `profile = "comprehensive"`
- AND a profile named `comprehensive` is defined
- WHEN `forge scan` executes
- THEN Forge SHALL run the analyzers enabled by that profile

### Requirement: FORGE-CONF-011 — Rule configuration

Forge SHALL support enabling and disabling rules and overriding rule severity
through configuration, applying overrides to the rule's findings.

#### Scenario: Disabled rule

- GIVEN a rule is disabled in configuration
- WHEN analysis executes
- THEN Forge SHALL NOT report findings for the disabled rule

#### Scenario: Severity override

- GIVEN a rule's severity is overridden in configuration
- WHEN analysis executes
- THEN findings for that rule SHALL carry the overridden severity

### Requirement: FORGE-CONF-012 — Tool configuration

Forge SHALL support configuring the executable path and supported version range
for each integrated external tool.

#### Scenario: Custom executable path

- GIVEN a tool's executable path is configured
- WHEN Forge resolves the tool
- THEN Forge SHALL use the configured executable
- AND SHALL record its version

### Requirement: FORGE-CONF-013 — Policy configuration

Forge SHALL support configuring quality policies with severity and category
thresholds, and SHALL evaluate the policy named by the effective configuration.

#### Scenario: Policy thresholds

- GIVEN a policy defines severity and category thresholds
- WHEN the quality gate evaluates against that policy
- THEN the configured thresholds SHALL drive the decision
