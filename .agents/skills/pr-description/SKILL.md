---
name: pr-description
description: Generate a pull request description from branch commits
---

# PR Description

## Context

Commits in this branch:

```sh
!jj log -r 'ancestors(@, 20) ~ ancestors(trunk(), 20)'
```

Changed files:

```sh
!jj diff -r 'trunk()' --stat
```

## Task

Generate a pull request description summarizing the branch changes.

### Format

```markdown
## Summary

Brief overview of what this PR accomplishes (1-2 sentences).

## Changes

- Bullet point for each logical change
- Group related commits together
- Reference ADRs: (See ADR-NNNN)

## Testing

- [ ] Tests added/updated
- [ ] All tests passing (`mise test`)
- [ ] Manual testing performed (if applicable)

## Documentation

- [ ] CHANGELOG.md updated
- [ ] ADR written (if architectural decision)
- [ ] Doc comments added for public API
```

### Guidelines

- **Summary**: Focus on the "why" - what problem does this solve?
- **Changes**: Organize by feature/area, not by commit
- **Testing**: Be specific about what was tested
- **Documentation**: Check off what's included

### Style

- Write in present tense ("Adds support for..." not "Added...")
- Be concise but complete
- Link to related issues/discussions if any
- Mention breaking changes prominently
