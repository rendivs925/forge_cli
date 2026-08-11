# Forge Engineering Contract

## Specification Driven Development

No non-trivial feature implementation SHALL begin without an OpenSpec change.

The implementation SHALL satisfy the active OpenSpec requirements.

## Architecture

Forge SHALL integrate existing tools rather than reimplementing mature
analysis capabilities.

Forge core SHALL remain independent from project-specific rules.

## Extensibility

New analyzers, rules, reporters, and policies SHALL be implemented through
stable extension points.

## Quality

Every change SHALL pass:

- formatting
- compilation
- unit tests
- integration tests
- linting
- relevant analyzer checks
- OpenSpec verification
- quality gates

## Scope

Do not introduce abstractions without a concrete extension requirement.

Do not add project-specific behavior to Forge core.

Do not duplicate analyzer functionality already provided by an integrated tool.

## Rust Style

Imports SHALL be declared at the top of the file, never inline.

Every crate file SHALL follow these rules:

- No `unwrap()`, `expect()`, or `panic!()`; use fallible combinators and
  propagate errors with `?` against typed errors.
- No `.clone()` without justification.
- No mutable static state.
- No secrets in code or logs.
- No debug prints; use structured logging.
- No SQL in controllers; SQL lives only in repositories.

Functions SHALL be small and single-responsibility (roughly 25 LOC); guard
clauses and early returns are preferred over deep nesting.

Layering SHALL be strict: repository -> service -> controller. Cross-domain
access SHALL go through service interfaces, never direct repository access.

Domain behavior SHALL be encapsulated on its type; free functions are only
for stateless utilities. Shared types SHALL be defined once and imported.

Use type-driven design: make illegal states unrepresentable, prefer enums
over strings, and use builders when fields are dependent.

Errors SHALL be typed, logged once at the boundary, and never expose internal
details to callers. Validation SHALL happen once at the boundary.

Tests SHALL exercise behavior, not implementation details.
