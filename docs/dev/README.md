# Contributing to Farg

Thank you for your interest in contributing to Farg. This guide will help you get started with
development.

## Quick Start

```bash
# Install mise (recommended)
curl https://mise.run | sh

# Install toolchain and run all checks
mise install
mise run check

# Run tests
mise run test

# Format and lint
mise run format
mise run lint
```

Without mise, run scripts directly from `tasks/`:

```bash
./tasks/check
./tasks/test
./tasks/format/rust
./tasks/lint/rust
```

## Development Workflow

1. Create a branch for your changes
2. Make your changes following the [code style guide][code-style]
3. Run `mise run check` to verify everything passes
4. Submit a pull request

## Common Tasks

| Task           | Command               | Description                    |
|----------------|-----------------------|--------------------------------|
| Check          | `mise run check`      | Verify compilation             |
| Test           | `mise run test`       | Run tests with coverage        |
| Format         | `mise run format`     | Format all files               |
| Lint           | `mise run lint`       | Lint all files                 |
| Audit          | `mise run audit`      | Check for vulnerabilities      |

Filter tests: `mise run test -- --filter xyz`

## Guides

| Guide                      | Topics                                 |
|----------------------------|----------------------------------------|
| [Development Tasks][tasks] | Available commands and toolchain       |
| [Code Style][code-style]   | Formatting, linting, code organization |
| [Testing][testing]         | Test structure and conventions         |

## Documentation

- [Design Documents][design] - Architecture decisions and design rationale
- [Process Guide][process] - ADR and RFC workflows for proposing changes

[code-style]: https://github.com/aaronmallen/farg/blob/main/docs/dev/code-style.md
[design]: https://github.com/aaronmallen/farg/blob/main/docs/design/README.md
[process]: https://github.com/aaronmallen/farg/blob/main/docs/process/README.md
[tasks]: https://github.com/aaronmallen/farg/blob/main/docs/dev/tasks.md
[testing]: https://github.com/aaronmallen/farg/blob/main/docs/dev/testing.md
