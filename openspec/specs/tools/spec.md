# Tool Integration Specification

## Purpose

Define the Forge interface to external tools so that tool availability and
version compatibility are discoverable and analyzers can be replaced without
changing the normalized finding contract.

## Requirements

### Requirement: FORGE-TOOL-001 — Tool discovery

Forge SHALL discover configured external tools.

#### Scenario: Tool resolution

- WHEN Forge resolves an analyzer's tool
- THEN Forge SHALL determine whether the tool is available

### Requirement: FORGE-TOOL-002 — Tool health

`forge doctor` SHALL report whether configured tools are available and
compatible.

#### Scenario: Doctor health report

- WHEN `forge doctor` executes
- THEN Forge SHALL report the availability and compatibility of each configured
  tool

### Requirement: FORGE-TOOL-003 — Tool execution isolation

External analyzer failures SHALL be represented independently from findings.

#### Scenario: Tool failure

- GIVEN an external tool fails
- THEN Forge SHALL represent the failure as an execution outcome
- AND SHALL NOT represent it as a finding

### Requirement: FORGE-TOOL-004 — Tool version awareness

Forge SHALL record analyzer versions used during analysis.

#### Scenario: Version recording

- GIVEN an analysis executes
- THEN Forge SHALL record the version of each analyzer that ran

### Requirement: FORGE-TOOL-005 — Tool replacement

Replacing an analyzer SHALL NOT require changing the normalized finding
contract.

#### Scenario: Analyzer swap

- GIVEN one analyzer is replaced by another
- THEN Forge SHALL continue operating against the normalized finding model
