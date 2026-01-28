# Development Guide

This guide covers the tools and workflows you'll use when working on Farg. If you haven't already,
start with the [Contributing Guide](../CONTRIBUTING.md) to set up your environment.

## Quick Reference

```bash
mise run check        # Verify compilation
mise run test         # Run tests with coverage
mise run format       # Format all files
mise run lint         # Lint all files
mise run audit        # Check for vulnerabilities
```

Filter tests: `mise run test -- --filter xyz`

## Guides

| Guide                                            | Topics                                 |
|--------------------------------------------------|----------------------------------------|
| [Development Tasks](tasks.md)                    | Available commands and toolchain       |
| [Code Style](code-style.md)                      | Formatting, linting, code organization |
| [Testing](testing.md)                            | Test structure and conventions         |
