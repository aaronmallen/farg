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

Without mise, run scripts directly from `bin/`:

```bash
./bin/check
./bin/test
./bin/format/rust
./bin/lint/rust
```

## Development Workflow

1. Create a branch for your changes
2. Make your changes following the [code style guide](development/code-style.md)
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

## Documentation

- [Development Guide](development/README.md) - Tasks, tooling, and conventions
- [Design Documents](design/README.md) - Architecture decisions and design rationale
- [Process Guide](process/README.md) - ADR and RFC workflows for proposing changes
