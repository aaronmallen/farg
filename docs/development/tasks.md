# Development Tasks

This project uses [mise](https://mise.jdx.dev/) to manage development tools and run common tasks. Mise
ensures everyone uses the same tool versions and provides convenient commands for formatting, linting,
testing, and more.

If you prefer not to use mise, all tasks have equivalent shell scripts in the `bin/` directory.

## Quick Reference

```bash
mise run check        # Check for compilation errors
mise run test         # Run tests with coverage
mise run format       # Format all files (Rust, TOML, Markdown)
mise run lint         # Lint all files
mise run audit        # Audit dependencies for vulnerabilities
```

## Available Tasks

### Formatting

| Task                   | Aliases                           | Description                             |
|------------------------|-----------------------------------|-----------------------------------------|
| `mise run format`      | `fmt`                             | Format all files (Rust, TOML, Markdown) |
| `mise run format:rust` | `fmt:rust`, `fmt:rs`, `format:rs` | Format Rust files only                  |

The `format:rust` task runs:

1. `cargo sort -w` - Sort Cargo.toml dependencies
2. `cargo +nightly fmt` - Format code with rustfmt
3. `cargo clippy --fix` - Auto-fix clippy warnings

### Linting

| Task                 | Aliases                     | Description          |
|----------------------|-----------------------------|----------------------|
| `mise run lint`      | `l`                         | Lint all files       |
| `mise run lint:rust` | `l:rust`, `l:rs`, `lint:rs` | Lint Rust files only |

The `lint:rust` task runs:

1. `cargo sort -w --check` - Verify Cargo.toml is sorted
2. `cargo +nightly fmt --check` - Verify code is formatted
3. `cargo clippy` - Run clippy lints

### Other Tasks

| Task             | Aliases | Description                                                            |
|------------------|---------|------------------------------------------------------------------------|
| `mise run check` | `c`     | Check for compilation errors (`cargo check --workspace --all-targets`) |
| `mise run test`  | `t`     | Run tests with coverage (use `-- --filter xyz` to filter)              |
| `mise run audit` | -       | Audit dependencies for vulnerabilities and check for outdated packages |

## Without Mise

Run scripts directly from `bin/`:

```bash
./bin/format/rust     # Instead of: mise run format:rust
./bin/lint/rust       # Instead of: mise run lint:rust
./bin/test            # Instead of: mise run test
./bin/check           # Instead of: mise run check
./bin/audit           # Instead of: mise run audit
```

When running scripts directly, ensure required tools are installed and available in your PATH.

## Toolchain

Tools managed by mise (see `.config/mise.toml`):

- **Rust stable** with clippy
- **Rust nightly** with rustfmt (for unstable formatting options)
- **cargo-audit** - Security vulnerability auditing
- **cargo-llvm-cov** - Code coverage
- **cargo-nextest** - Test runner
- **cargo-outdated** - Dependency freshness checking
- **cargo-sort** - Cargo.toml sorting
