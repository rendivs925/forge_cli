# Analysis Engine Specification

## Purpose

Define how Forge orchestrates analyzers and combines their results into a
unified analysis.

## Requirements

### Requirement: FORGE-ENG-001 — Analyzer orchestration

The analysis engine SHALL orchestrate configured analyzers and collect their
results into a unified analysis.

#### Scenario: Multiple analyzers

- GIVEN multiple analyzers are enabled
- WHEN an analysis executes
- THEN independent analyzers SHALL be eligible for concurrent execution
- AND their results SHALL be combined into one analysis result

### Requirement: FORGE-ENG-002 — Resource bounded execution

The analysis engine SHALL bound concurrent external processes and resource
consumption.

#### Scenario: Parallel analysis

- GIVEN six analyzers are configured
- AND the configured concurrency limit is four
- WHEN analysis executes
- THEN no more than four analyzer executions SHALL run concurrently

### Requirement: FORGE-ENG-003 — Analyzer isolation

A failure in one analyzer SHALL NOT corrupt results produced by unrelated
analyzers.

#### Scenario: Analyzer failure

- GIVEN Semgrep fails
- WHEN Clippy and Gitleaks complete successfully
- THEN their results SHALL remain available
- AND Forge SHALL report Semgrep's failure separately

### Requirement: FORGE-ENG-004 — Deterministic execution

Given the same repository state, configuration, analyzer versions, and rule
versions, Forge SHALL produce equivalent analysis results.

#### Scenario: Repeat analysis

- GIVEN the repository state, configuration, analyzer versions, and rule
  versions are unchanged
- WHEN analysis executes twice
- THEN Forge SHALL produce equivalent results
