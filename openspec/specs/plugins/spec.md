# Plugin Specification

## Purpose

Define the extension mechanisms Forge SHALL support so that new analyzers,
rules, and policies can be added without recompiling or forking Forge core.

## Requirements

### Requirement: FORGE-PLUG-001 — Stable extension mechanisms

Forge SHALL support extension through external tool adapters, declarative
rules, and versioned configuration.

#### Scenario: Adapter extension

- GIVEN a new external tool must be integrated
- THEN the extension SHALL be implementable as an adapter without modifying
  Forge core

#### Scenario: Declarative rule extension

- GIVEN a repository needs a custom rule
- THEN the rule SHALL be definable declaratively without modifying Forge core

### Requirement: FORGE-PLUG-002 — Language independence

Programmatic custom rules SHALL be implementable independently of Forge's
implementation language.

#### Scenario: Foreign language rule

- GIVEN a rule plugin is implemented in a language other than Rust
- WHEN Forge invokes the plugin
- THEN Forge SHALL communicate with the plugin through a stable structured
  protocol

### Requirement: FORGE-PLUG-003 — Process isolation

Forge SHALL NOT load third-party code into the Forge process.

#### Scenario: Plugin process boundary

- GIVEN a programmatic rule plugin executes
- THEN Forge SHALL communicate with the plugin as a separate process
- AND SHALL NOT load the plugin's code into the Forge process
