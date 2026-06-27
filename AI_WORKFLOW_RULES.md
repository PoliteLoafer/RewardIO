# AI Workflow Rules

Purpose: increase implementation quality and reduce token usage by giving agents stable project context and strict execution rules.

## 1) First-read order (token-efficient)

Before coding, read in this order:

1. `README.md`
2. `PROJECT_STRUCTURE.md`
3. target crate/module files only

Do not scan the whole repository if the task is scoped to one module.

## 2) Scope discipline

- Implement only what the task explicitly requires.
- Prefer minimal diffs over broad refactors.
- Do not change unrelated files "for cleanup".
- Keep architecture boundaries:
  - business rules in `crates/core`
  - infrastructure in `crates/infra`
  - HTTP/wiring in `crates/api`

## 3) Quality gates for every change

- Preserve existing behavior unless task says otherwise.
- Follow existing naming/style/import order in touched files.
- Avoid sensitive logging (passwords, full auth payloads, secrets).
- For database-related changes, ensure migration symmetry:
  - `*.up.sql` and matching `*.down.sql`
  - both are executable and reversible

## 4) Validation rules

- Rust backend changes:
  - every new feature must include tests that validate real behavior and real use cases
  - tests must assert outcomes (status/body/state/side effects), not just execute code for coverage percentage
  - run targeted crate tests first: `cargo test -p rewardio-api` (or affected crate)
  - run wider tests only when risk/scope requires it
- Migration changes:
  - run `sqlx migrate run`
  - run `sqlx migrate revert`
  - check state with `sqlx migrate info`
- Documentation-only changes: tests are optional.

## 5) Output format for AI agents

When providing a result, include:

- Summary of what was changed
- Exact files touched
- Verification steps and results
- Any remaining risks or follow-ups

Keep answers concise and concrete.

## 6) Guardrails (must not)

- Do not invent non-existent project modules or commands.
- Do not claim tests passed if they were not executed.
- Do not bypass failing tests by weakening assertions or skipping suites.
- Do not expose secrets from `.env` or runtime configuration in logs/responses.

## 7) Quick task template for AI

1. Confirm scope and target files.
2. Read only required context.
3. Implement minimal patch.
4. Run appropriate verification.
5. Report changed files + evidence.
