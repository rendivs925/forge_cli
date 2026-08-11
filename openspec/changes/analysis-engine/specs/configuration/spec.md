# Forge Configuration — Analysis Deltas

## ADDED Requirements

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
