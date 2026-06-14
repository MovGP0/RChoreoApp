# Issue Tracking

This project uses **bd** (Beads) for issue tracking. Beads state lives in `.beads/`; this repository versions the top-level JSONL exchange files and ignores local runtime/database artifacts.

Run `bd onboard` before first use in a session, then run `bd ready` before selecting work.

## Quick Reference

```bash
bd onboard
bd ready
bd show <id>
bd update <id> --status in_progress
bd update <id> --append-notes "..."
bd close <id>
```

Use `bd hooks list` to check Git hook status. If hooks are missing or outdated, refresh local hooks with:

```bash
bd hooks install --force
```

The current hook model delegates to `bd hooks run <hook>`. Do not use old hook scripts that call `bd sync`; that command is not available in current Beads versions.

## Creating Work Items

Create a bead for every user instruction that produces project changes.

```bash
bd create --title "scope: concrete action" --description "Goal:
Definition of Done:
Notes:" --priority 2 --type task
bd update <id> --status in_progress
```

Titles use:

```yaml
<scope>: <concrete action>
```

Descriptions use:

```yaml
Goal:
Definition of Done:
Notes:
```

Priority values:

| Priority | Meaning |
| --- | --- |
| 0 | Critical path / blocking |
| 1 | High |
| 2 | Normal |
| 3 | Low |
| 4 | Backlog |

## JSONL Exchange Files

Use `bd export` and `bd import` for JSONL exchange. `bd sync` is obsolete for this workflow.

```bash
bd export -o .beads/issues.jsonl
bd import .beads/issues.jsonl
```

Before a commit, make sure `.beads/issues.jsonl` and other intended JSONL files are current and staged with the rest of the change. The Beads pre-commit hook may export and stage `.beads/issues.jsonl` automatically when configured, but agents should still inspect `git status` before handing off.

Only version the intended `.beads/*.jsonl` exchange files and `.beads/.gitignore`. Local runtime files such as `embeddeddolt/`, `backup/`, `.auto-import-issues.jsonl`, sockets, locks, SQLite files, and generated server state must stay unversioned.

## Dependencies

Use dependencies when one bead cannot proceed until another is complete.

```bash
bd dep add <child> <parent>
bd dep add RChoreoApp-101 RChoreoApp-100
```

The child is blocked by the parent. Do not start blocked beads.

## Session Workflow

1. Run `bd ready`.
2. Select the highest-priority ready bead that matches the user request.
3. Run `bd show <id>`.
4. Mark it in progress.
5. Do the work and keep notes on important findings.
6. Run the relevant build, lint, and test checks for changed Rust crates.
7. Export Beads JSONL when needed.
8. Inspect `git status`.
9. Let the user manually verify behavior before closing the bead.

Do not close a bead until the user confirms the problem is solved.

## Landing the Plane

When ending a work session:

1. File follow-up beads for remaining work.
2. Run quality gates when code changed.
3. Update bead notes and status.
4. Refresh JSONL exchange files with `bd export -o .beads/issues.jsonl` when needed.
5. Verify hooks with `bd hooks list`; refresh with `bd hooks install --force` if outdated.
6. Inspect `git status` and report staged, unstaged, and ignored Beads files clearly.
7. Commit, pull/rebase, and push only when the user has explicitly asked for those Git operations.

If a Git hook fails with a message mentioning `bd sync`, the hook is stale. Run:

```bash
bd hooks list
bd hooks install --force
bd hooks run pre-commit
```

Then retry the Git operation.

## Failure Modes to Avoid

- Working without a bead.
- Working on blocked beads.
- Creating related tasks without dependencies.
- Letting `.beads/issues.jsonl` drift from the Beads database when JSONL is being versioned.
- Changing code without a bead in progress.
- Closing a bead before the user confirms the result.
- Staging or committing Git changes without explicit user permission.
