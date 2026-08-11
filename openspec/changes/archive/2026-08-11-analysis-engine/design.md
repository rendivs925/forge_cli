# Design: Analysis Engine

## Context

See proposal.md — Why. Current state:

- `forge-check`, `scan`, `gate` handlers in `crates/forge-cli` are stubs that
  only resolve configuration and print a canned "no findings" result.
- `crates/forge-config` resolves a flat `ForgeConfig` (schema, profile,
  offline, no_cache, fail_fast) with layer provenance.
- `crates/forge-core` provides `ExitCode` and `ForgeError`; exit code 3 already
  exists for tool execution errors.
- The main spec tree fully specifies the analysis, findings, rules, and
  quality-gates capabilities; this change implements them.

## Goals / Non-Goals

**Goals:**
- A `forge-analysis` crate owning the finding model, analyzer contract, rule
  model, engine, and gate evaluator — no CLI logic inside.
- Process-based analyzer execution with bounded concurrency, per-analyzer
  timeouts, version capture, and execution outcomes kept separate from
  findings.
- Deterministic results regardless of execution order.
- Persisted latest analysis consumed by `forge gate` (and later `forge report`).
- `check`/`scan`/`gate` wired end-to-end with correct exit codes.

**Non-Goals:**
- Baseline matching (FORGE-GATE-002) — a later change introduces baselines;
  until then gates evaluate all findings.
- Real third-party adapters (semgrep, clippy, gitleaks) — the adapter contract
  and a command-based adapter are built; concrete tool integrations land in
  later changes.
- `rules`/`tools`/`policy` interactive commands — data models and resolution
  exist; those commands arrive in a later change.
- Caching of analyzer results (FORGE-CACHE-*) — separate change.

## Decisions

### D1. New crate `crates/forge-analysis`

Owns: `finding`, `rule`, `analyzer`, `tool`, `engine`, `gate`, `analysis`
(aggregate), and the persisted-analysis store. Depends on `forge-core` (exit
codes) and `serde`/`serde_json`. The CLI crate depends on it; forge-config only
supplies raw resolved values through a plain-data struct, avoiding a
forge-config → forge-analysis dependency.

### D2. Finding model

`Finding` is a plain serializable struct:

- `id` — stable fingerprint derived from (analyzer id, rule id, file,
  start line, start column, message). Stable across runs for the same logical
  location (FORGE-FIND-002, FORGE-BASE-003).
- `rule_id`, `severity` (`Severity` enum: Blocker, Critical, Major, Minor,
  Info — comparable ordering), `category` (`Category` enum: security,
  reliability, maintainability, architecture, performance, supply-chain).
- `location` (file path relative to workspace root, start/end line and
  column, optional), `message`, `remediation` (optional text), `analyzer_id`.
- `fingerprint` used for identity, derived at construction.

Severity and category are normalized enums so cross-analyzer findings are
comparable (FORGE-FIND-003).

### D3. Analyzer contract

`Analyzer` trait:

- `id()`, `name()`, `description()`
- `supported_project_types()` / `capabilities()`
- `execution_requirements() -> ToolRequirement` (executable name and optional
  supported version range, FORGE-ENG-020, FORGE-ENG-022)
- `run(&self, ctx: &RunContext) -> Result<RunOutput, RunError>`

`RunContext` carries workspace root, config, and per-tool settings.
`RunOutput` holds findings plus `tool_version` and `duration`.

A built-in `CommandAnalyzer` implements the trait generically: it runs a
configured external executable with args, captures stdout/stderr, enforces a
timeout, and interprets output as Forge JSON finding lines (documented
schema) or exit-code-only mode (FORGE-ENG-040). Concrete tool adapters in
later changes reuse this executor.

### D4. Engine with bounded concurrency, isolation, determinism

`AnalysisEngine::run(&self, analyzers, ctx) -> Analysis`:

- Executes analyzers in batches of at most `concurrency` (from config,
  default 4), each batch in parallel threads joined before the next batch
  (FORGE-ENG-002). Deterministic: results merged in analyzer registration
  order, findings sorted by (rule_id, file, line, column) (FORGE-ENG-004).
- Per-analyzer isolation: each `run` returns its own outcome; a panic or
  failure is caught per thread and recorded as an execution outcome, never
  corrupting other analyzers' findings (FORGE-ENG-003).
- Timeout: process polling via `Child::try_wait` with deadline; on timeout the
  child is killed and the outcome is `Failed(Timeout)` (FORGE-ENG-042).
- Version capture: adapter runs the tool with `--version` (configurable)
  before analysis when possible (FORGE-ENG-043).

### D5. Execution outcomes separate from findings

`ExecutionOutcome` enum: `Succeeded { version, duration }`,
`Failed { reason: FailedReason, ... }`, where `FailedReason` is
`MissingTool`, `ExitStatus(ExitCode)`, `Timeout`, `Internal`.
`Analysis` aggregates findings + outcomes (FORGE-ENG-041, FORGE-TOOL-003).

Exit-code rule at the CLI boundary: if any enabled analyzer produced a
`Failed` outcome, the command exits 3 (FORGE-CLI-043); the gate decision is
still computed and reported but never masks the execution failure. If all
analyzers succeeded and the gate failed, exit 1.

### D6. Rules and rule resolution

`Rule { id, name, category, default_severity, description, enabled,
applicability }`. A `RuleRegistry` merges built-in rules with configured rule
packs (FORGE-RULE-005) and applies enablement + severity overrides from
config before analysis (FORGE-RULE-003, -004). Rules are independent units;
the engine never couples rule evaluation across analyzers (FORGE-RULE-006).

### D7. Gate evaluation

`GateEvaluator::evaluate(&Analysis, &Policy) -> GateDecision`:

- `Policy` = severity thresholds (`max_blockers`, `max_critical`, ...) +
  category thresholds (`max_security`, ...) (FORGE-GATE-003, -004).
- `GateDecision` = `Pass` | `Fail { failing_conditions, responsible_findings }`,
  deterministic (FORGE-GATE-005) and explainable: it names the policy and the
  findings that broke each threshold (FORGE-GATE-006).
- Until baselines exist, all findings count as new (documented limitation;
  FORGE-GATE-002 is satisfied by the later baseline change).

### D8. Persisted analysis store

`AnalysisStore` writes the latest `Analysis` as JSON under
`<workspace>/.forge/analysis/latest.json` after scan/check, and reads it for
`forge gate` / `forge report` (FORGE-CLI-035). Missing file → the command
reports "no analysis result available" and exits 2. `scan` always re-analyzes;
`gate` never analyzes.

### D9. Configuration surface

Extend `ForgeConfig` (schema 1, backward compatible) with:

- `profile` already exists; profiles become resolvable: `profiles: map
  name -> { tools: [tool ids], concurrency }`. A built-in default profile
  enables no tools (deterministic empty analysis on fresh repos).
- `tools: map name -> { executable, args, timeout_secs, version_command,
  enabled }`.
- `policies: map name -> { max_blockers, max_critical, max_major, categories:
  { security, ... } }` and `gate_policy` to select the active one.
- `rules: map rule-id -> { enabled, severity }` overrides.

forge-config keeps the same layered resolver/provenance machinery; new keys
are `Option`-typed at the top level so resolution and provenance extend
naturally. Unknown nested keys are rejected at parse time.

## Risks / Trade-offs

- [Process polling for timeouts is not millisecond-precise] → acceptable for
  analyzer executions; polling interval is 50ms.
- [Thread-based batching adds complexity vs a thread pool crate] → avoids a new
  dependency; batching satisfies the bounded-concurrency requirement.
- [Gate counts all findings as new until baselines exist] → explicitly
  documented; the baseline change reuses `GateEvaluator` unchanged.
- [JSON-lines tool contract requires adapters to emit Forge schema] → matches
  FORGE-ENG-040's process boundary; per-tool normalization is adapter work in
  later changes.
