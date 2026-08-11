## Why

Forge's core value is orchestrating analyzers and enforcing quality gates, but
today `forge check`, `forge scan`, and `forge gate` are stubs that only resolve
configuration. Without an analysis engine, finding model, rule registry, and
quality gate evaluation, none of the downstream capabilities (reporting,
baseline, diff, explain, fix) can be built.

## What Changes

- Introduce `crates/forge-analysis`: a new crate defining the normalized finding
  model, the analyzer adapter trait, the rule model, the analysis engine, and
  the quality gate evaluator.
- Implement process-based analyzer execution with bounded concurrency,
  timeouts, execution-outcome recording (distinct from findings), and analyzer
  version capture.
- Implement a built-in analyzer registry with an adapter for a simple external
  tool, demonstrating the extension contract end-to-end.
- Wire `forge check`, `forge scan`, and `forge gate` to the analysis engine:
  - `check` runs a fast profile; `scan` runs the comprehensive profile and can
    gate with `--gate`; `gate` evaluates findings against configured policies.
- Add quality gate evaluation with severity thresholds and category policies,
  producing deterministic, explainable decisions that identify the failing
  policy and responsible findings.
- Expose rules, tools, and policy surfaces used by the engine (data models and
  resolution only; interactive commands land in a later change).

## Capabilities

The main spec tree already specifies the analysis, findings, rules, and
quality-gates capabilities (FORGE-ENG-*, FORGE-FIND-*, FORGE-RULE-*,
FORGE-GATE-*). This change implements those requirements; it introduces spec
deltas only where genuinely new observable behavior is added.

### New Capabilities

- (none — the analysis, findings, rules, and quality-gates capabilities already
  exist in `openspec/specs/`)

### Modified Capabilities

- `cli`: analysis results are persisted between commands so `forge gate` and
  `forge report` consume the latest analysis without re-running analyzers.
- `configuration`: the configuration surface gains analysis-related keys:
  profiles, rule enablement and severity overrides, tool executables, and
  policy thresholds.

## Impact

- New crate `crates/forge-analysis` added to the workspace.
- New configuration keys for profiles, rules enablement/severity overrides,
  tool executables, and policy thresholds; `forge-config` resolver extended to
  carry them into the analysis crate.
- `forge-cli` command handlers for check/scan/gate rewritten to invoke the
  engine, and a persisted analysis result store shared with gate/report.
- Analyzer failures do not fail analysis; they are recorded as execution
  outcomes. Gate failures use exit code 1; tool execution errors use exit
  code 3.
- No third-party analyzer code is embedded; all execution is process-based.
