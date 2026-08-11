# OpenSpec Workspace

OpenSpec is the source of truth for Forge's current behavioral requirements and
proposed changes to that behavior.

## Navigation

1. Read `specs/<domain>/spec.md` to understand current behavior.
2. Read active `changes/<change>/` artifacts before proposing a concurrent change.
3. Read `changes/archive/` to understand historical decisions and delivery evidence.

## Ownership Boundaries

| Concern | Authority |
|---|---|
| Behavior specifications and change deltas | `openspec/` |
| Architecture decisions and ADRs | `docs/architecture/` |
| AI guidance and engineering contract | `AGENTS.md` |
| Automated enforcement | planned inside the `forge` binary itself |

## Change Lifecycle

`proposal -> design and delta -> implementation -> verification -> closure -> archive`

Only a completed, verified change may sync behavioral deltas into `specs/`.
Cancelled, superseded, and partial changes preserve history in the archive but
do not alter current behavior.

## Naming

- Domains and capabilities: lowercase kebab-case.
- Change directories: one intent in lowercase kebab-case.
- Requirement IDs: stable `FORGE-<DOMAIN>-<NUMBER>` identifiers.
- Do not encode implementation tactics in a spec path.
