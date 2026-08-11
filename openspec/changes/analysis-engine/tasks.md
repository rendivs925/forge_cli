## 1. forge-analysis crate scaffolding

- [x] 1.1 Create `crates/forge-analysis` crate (Cargo.toml depending on forge-core, serde, serde_json; add to workspace members)
- [x] 1.2 Define `Severity` and `Category` enums with ordering and serde support

## 2. Finding and rule models

- [x] 2.1 Implement `Finding` struct with stable fingerprint derivation (FORGE-FIND-001..005)
- [x] 2.2 Implement `Rule` struct with id, metadata, enablement, severity override (FORGE-RULE-001..004)
- [x] 2.3 Implement `RuleRegistry` merging built-in rules with configured packs and overrides (FORGE-RULE-005)

## 3. Analyzer contract and execution

- [x] 3.1 Implement `Analyzer` trait with identity, capabilities, and execution requirements (FORGE-ENG-020, FORGE-TOOL-001)
- [x] 3.2 Implement `ExecutionOutcome` and `FailedReason` (FORGE-ENG-041, FORGE-TOOL-003)
- [x] 3.3 Implement `CommandAnalyzer` running external processes with args capture (FORGE-ENG-040)
- [x] 3.4 Implement timeout enforcement via process polling (FORGE-ENG-042)
- [x] 3.5 Implement version capture via configurable version command (FORGE-ENG-043, FORGE-TOOL-004)
- [x] 3.6 Implement tool discovery: executable resolution and version-range check (FORGE-ENG-021, FORGE-ENG-022, FORGE-TOOL-002)

## 4. Analysis engine

- [x] 4.1 Implement `Analysis` aggregate combining findings and execution outcomes
- [x] 4.2 Implement `AnalysisEngine::run` with bounded-concurrency batches (FORGE-ENG-002)
- [x] 4.3 Implement per-analyzer isolation so failures never corrupt other results (FORGE-ENG-003)
- [x] 4.4 Implement deterministic merge and ordering of results (FORGE-ENG-004)

## 5. Gate evaluation

- [x] 5.1 Implement `Policy` with severity and category thresholds (FORGE-GATE-003, FORGE-GATE-004)
- [x] 5.2 Implement `GateEvaluator` producing deterministic `GateDecision` (FORGE-GATE-005)
- [x] 5.3 Make failures explainable: failing conditions plus responsible findings (FORGE-GATE-006)
- [x] 5.4 Implement `PolicyResolver` selecting the active policy from config (FORGE-POL-001, FORGE-POL-004, FORGE-POL-005)

## 6. Persisted analysis store

- [x] 6.1 Implement `AnalysisStore` writing and reading latest analysis JSON under `.forge/analysis/` (FORGE-CLI-035)

## 7. Configuration surface

- [x] 7.1 Extend `ForgeConfig` and `RawConfig` with tools, rules, profiles, policies, gate_policy keys (FORGE-CONF-010..013)
- [x] 7.2 Extend resolver layer application and validation for new keys with provenance
- [x] 7.3 Add unit tests for new config resolution and validation

## 8. CLI wiring

- [x] 8.1 Rewrite `forge check` to run the default profile and persist analysis (FORGE-CLI-021)
- [x] 8.2 Rewrite `forge scan` to run the comprehensive profile, persist, and gate with `--gate` (FORGE-CLI-022, FORGE-CLI-023)
- [x] 8.3 Rewrite `forge gate` to evaluate the persisted analysis without analyzing (FORGE-CLI-023, FORGE-CLI-035)
- [x] 8.4 Wire exit codes: 0 pass, 1 gate failure, 2 no analysis/config, 3 tool execution failure (FORGE-CLI-040..045)

## 9. Tests and quality gates

- [x] 9.1 Unit tests for findings, rules, engine, gate, and store in forge-analysis
- [x] 9.2 Integration tests: scan/gate exit codes, persisted analysis, analyzer failure isolation
- [x] 9.3 Run fmt, build, clippy, tests, and `openspec validate` until all pass
