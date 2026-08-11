# Forge Configuration Specification (Delta)

## ADDED Requirements

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
