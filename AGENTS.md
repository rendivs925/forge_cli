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
