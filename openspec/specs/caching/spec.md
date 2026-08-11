# Cache Specification

## Purpose

Define how Forge SHALL avoid repeated work while preserving correctness across
repository and configuration boundaries.

## Requirements

### Requirement: FORGE-CACHE-001 — Incremental analysis

Forge SHALL avoid repeating analysis for inputs that have not changed whenever
possible.

#### Scenario: Unchanged input

- GIVEN an input has not changed since a previous analysis
- WHEN analysis executes
- THEN Forge SHOULD reuse the cached result for that input

### Requirement: FORGE-CACHE-002 — Cache correctness

Cached results SHALL be invalidated when relevant inputs change.

Relevant inputs MAY include:

- source files
- configuration
- rule versions
- analyzer versions
- dependency state

#### Scenario: Source change

- GIVEN a source file changes
- WHEN analysis executes
- THEN Forge SHALL invalidate the cached result for that file

### Requirement: FORGE-CACHE-003 — Cache isolation

Cache entries SHALL be isolated sufficiently to prevent results from one
repository or configuration from being incorrectly reused by another.

#### Scenario: Repository isolation

- GIVEN two repositories share a cache
- THEN results from one repository SHALL NOT be reused for the other

### Requirement: FORGE-CACHE-004 — Cache management

Users SHALL be able to inspect, clear, and prune the cache.

#### Scenario: Cache inspection

- WHEN a user inspects the cache
- THEN Forge SHALL display the cache location, size, entries, and hit rate
