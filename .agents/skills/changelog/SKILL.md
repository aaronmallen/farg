---
name: changelog
description: Update CHANGELOG.md with recent changes following Keep a Changelog format
---

# Changelog

## Context

Current changelog state:

```sh
!cat CHANGELOG.md
```

Recent changes to document:

```sh
!jj log --limit 10 -T 'description ++ "\n---\n"'
```

## Task

Update `CHANGELOG.md` following [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) format.

### Format Rules

1. Add entries under `## [Unreleased]` section
2. Group changes by type:
    - `### Added` - new features
    - `### Changed` - changes to existing functionality
    - `### Deprecated` - features to be removed
    - `### Removed` - removed features
    - `### Fixed` - bug fixes
    - `### Security` - vulnerability fixes

3. Each entry should:
    - Start with a verb (Add, Change, Fix, Remove, etc.)
    - Be concise but descriptive
    - Reference ADRs when relevant: `(See ADR-NNNN)`
    - Group related items logically

### Style

- Write in imperative mood ("Add X" not "Added X")
- Focus on user-facing changes, not implementation details
- Combine related commits into single entries when appropriate
- Order entries by importance within each section

### Example Entry

```markdown
### Added

- Add `Xyz` and `Lms` color spaces as foundation for color representation
- Add chromatic adaptation transforms: Bradford, CAT02, CAT16, Von Kries, and others (See ADR-0002)
- Add `illuminant` module with CIE standard illuminants (D65 always available, others feature-gated)
```
