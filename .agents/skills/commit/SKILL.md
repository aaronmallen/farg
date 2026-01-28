---
name: commit
description: Create a commit with jj following project commit message style
---

# Commit

## Context

Current changes to commit:

```sh
!jj status
```

```sh
!jj diff --stat
```

Recent commit messages for style reference:

```sh
!jj log --limit 5 -T 'description ++ "\n---\n"'
```

## Task

Create a commit using `jj commit` following the project's commit message style.

### Message Format

**Subject line:**

- Start with imperative verb: Add, Fix, Remove, Update, Refactor, Rename, Move
- Describe WHAT the commit does, not how
- Keep under 72 characters
- No period at end
- Use backticks for code references: Add `Component` type

**Body (blank line after subject):**

- Explain the change in present tense (describing state after commit)
- Focus on WHAT and WHY, not HOW
- Wrap at 72 characters
- Use bullet points for lists
- Reference ADRs when relevant: "See ADR-NNNN: Title"

### Examples

Simple addition:

```txt
Add `Matrix3` type for 3x3 matrix operations

Provides const-constructible 3x3 matrices with arithmetic operations,
determinant calculation, and matrix inversion for color space
transformations.
```

Feature with ADR reference:

```txt
Add chromatic adaptation transforms with feature gating

Introduces ChromaticAdaptationTransform for adapting colors between
illuminants. Includes Bradford, CAT02, CAT16, Von Kries, Sharp,
Fairchild, Hunt-Pointer-Estevez, and CMC CAT97/2000 transforms.

Components are feature-gated for compile-time optimization with
Bradford as the default. XYZ Scaling serves as an always-available
fallback.

See ADR-0002: Feature-Gated Components
```

### Command

Use heredoc for multi-line messages:

```sh
jj commit -m "$(cat <<'EOF'
Subject line here

Body paragraph here explaining the change.

- Bullet points if needed
- Another point

See ADR-NNNN: Title (if applicable)
EOF
)"
```
