# Forge CLI Commands

## Purpose

Define the stable command surface Forge SHALL expose and the responsibilities
that belong to each command.

## Command surface

The Forge public CLI SHALL expose the following commands:

```text
forge
│
├── init          Initialize Forge in a repository
├── check         Fast local quality check
├── scan          Full analysis
├── gate          Evaluate quality gate
│
├── rules         Manage rules and rule packs
├── tools         Manage integrated analyzers
├── profile       Manage analysis profiles
├── policy        Manage quality policies
│
├── baseline      Manage existing technical debt
├── explain       Explain a finding or rule
├── fix           Apply supported automatic fixes
│
├── report        Generate reports
├── diff          Analyze changes
│
├── config        Inspect effective configuration
├── doctor        Diagnose Forge/tool environment
├── cache         Manage analysis cache
│
└── version
```

## Requirements

### Requirement: FORGE-CLI-020 — Command categories

Forge SHALL group commands into the primary workflows of fast local checks,
comprehensive analysis, and quality policy evaluation.

#### Scenario: Workflow grouping

- WHEN a user inspects `forge --help`
- THEN Forge SHALL make the check, scan, and gate workflows prominent

### Requirement: FORGE-CLI-021 — Check workflow

`forge check` SHALL execute the configured fast or incremental analysis profile
and SHALL be suitable for frequent developer use.

#### Scenario: Fast check

- GIVEN a repository with changed files
- WHEN `forge check` executes
- THEN Forge SHALL analyze the changed files against the configured profile
- AND SHALL return a quality result

### Requirement: FORGE-CLI-022 — Scan workflow

`forge scan` SHALL execute the configured comprehensive analysis profile across
the full repository.

#### Scenario: Full scan

- WHEN `forge scan` executes
- THEN Forge SHALL run the configured analyzers against the repository
- AND SHALL combine their results into one analysis

### Requirement: FORGE-CLI-023 — Gate workflow

`forge gate` SHALL evaluate quality policies against analysis results and SHALL
NOT itself perform analysis.

#### Scenario: Policy evaluation

- WHEN `forge gate` executes against analysis results
- THEN Forge SHALL evaluate the configured quality policies
- AND SHALL return a policy decision

#### Scenario: Gated scan

- GIVEN the user specifies a gate flag
- WHEN `forge scan --gate` executes
- THEN Forge SHALL evaluate the quality gate against the scan results

### Requirement: FORGE-CLI-024 — Change analysis

`forge diff` SHALL analyze findings relative to a base revision so that only
new or changed findings affect the quality result.

#### Scenario: New findings on changed lines

- GIVEN a change introduces a new finding
- WHEN `forge diff` executes
- THEN Forge SHALL report the new finding
- AND SHALL include it in the quality decision

### Requirement: FORGE-CLI-025 — Rule introspection

Forge SHALL allow users to list and explain rules without executing analysis.

#### Scenario: Listing rules

- WHEN a user executes `forge rules list`
- THEN Forge SHALL display the available rules grouped by category

#### Scenario: Explaining a rule

- WHEN a user executes `forge rules explain <rule>`
- THEN Forge SHALL display the rule's metadata, rationale, and configuration

### Requirement: FORGE-CLI-026 — Finding explanation

Forge SHALL explain a specific finding with its location, source, and
recommended remediation.

#### Scenario: Explaining a finding

- WHEN a user executes `forge explain <finding-id>`
- THEN Forge SHALL display the finding's rule, severity, location, and
  remediation guidance

### Requirement: FORGE-CLI-027 — Supported fixes only

`forge fix` SHALL apply only automatic fixes that are explicitly supported and
safe, and SHALL preview changes before applying them.

#### Scenario: Dry run

- WHEN a user executes `forge fix --dry-run`
- THEN Forge SHALL display the available fixes without modifying source code

#### Scenario: Applying fixes

- GIVEN a fix is supported for a finding
- WHEN a user confirms the fix
- THEN Forge SHALL apply the fix

### Requirement: FORGE-CLI-028 — Tool management

Forge SHALL allow users to list integrated tools and diagnose their environment.

#### Scenario: Listing tools

- WHEN a user executes `forge tools list`
- THEN Forge SHALL display the integrated tools with version and status

#### Scenario: Tool diagnosis

- WHEN a user executes `forge tools doctor`
- THEN Forge SHALL report tool availability and compatibility

### Requirement: FORGE-CLI-029 — Configuration inspection

Forge SHALL expose the effective configuration and its provenance.

#### Scenario: Show configuration

- WHEN a user executes `forge config show`
- THEN Forge SHALL display the effective configuration

#### Scenario: Explain configuration

- WHEN a user executes `forge config explain <key>`
- THEN Forge SHALL identify the configuration layers contributing the effective
  value

### Requirement: FORGE-CLI-030 — Environment diagnosis

`forge doctor` SHALL report the health of the Forge installation, repository,
tools, configuration, cache, and network requirements.

#### Scenario: Healthy environment

- GIVEN a healthy environment
- WHEN `forge doctor` executes
- THEN Forge SHALL report all checks as passing

#### Scenario: Missing tool

- GIVEN a configured tool is missing
- WHEN `forge doctor` executes
- THEN Forge SHALL identify the missing executable

### Requirement: FORGE-CLI-031 — Baseline management

Forge SHALL allow users to create, show, update, and clear the analysis
baseline.

#### Scenario: Creating a baseline

- WHEN a user executes `forge baseline create`
- THEN Forge SHALL record the current findings as the baseline
- AND SHALL report the baseline fingerprint

### Requirement: FORGE-CLI-032 — Reporting

Forge SHALL generate reports from an existing analysis in multiple formats.

#### Scenario: Report formats

- WHEN a user requests a report
- THEN Forge SHALL support at least terminal, JSON, SARIF, and JUnit formats

### Requirement: FORGE-CLI-033 — Cache management

Forge SHALL allow users to inspect, clear, and prune the analysis cache.

#### Scenario: Cache status

- WHEN a user executes `forge cache status`
- THEN Forge SHALL display cache location, size, entries, and hit rate

### Requirement: FORGE-CLI-034 — Initialization

`forge init` SHALL detect the repository type and generate a starter Forge
configuration.

#### Scenario: Initializing a repository

- GIVEN a supported repository
- WHEN `forge init` executes
- THEN Forge SHALL create the initial configuration and extension directories
- AND SHALL NOT modify existing hooks without explicit consent
