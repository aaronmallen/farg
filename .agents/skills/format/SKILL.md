---
name: format
description: Format code according to specified style guide
---

# Format

## Context

Current changes to ensure are properly formatted:

```sh
!jj show
```

## Task

1. Run `mise run format` to format code and fix any **errors** the formatter was unable to address.
2. For each `.rs` file in the diff, spin up a **separate agent** (using the Task tool with `subagent_type: Explore`)
    to audit that file against `docs/development/code-style.md`. Launch all agents in parallel.
    - Each agent should read the full file and check module-level ordering (constants first, then type groups
      alphabetically with impl blocks immediately following their type, then free functions) and report any
      violations with line numbers.
3. Fix any violations the agents report.
4. Run `mise run format` again after fixes to ensure formatting is still clean.
5. Run `cargo test --features full` to confirm nothing is broken.
