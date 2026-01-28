# Code Style

This document describes the formatting and organization conventions used in Farg. Most formatting is
handled automatically by tools—you generally just need to run `mise run format` before committing.

The code organization rules (module ordering, impl block ordering) are the main things to keep in mind
when writing new code.

## Running Formatters and Linters

```bash
mise run format       # Format all files
mise run format:rust  # Format Rust files only
mise run lint         # Lint all files
mise run lint:rust    # Lint Rust files only

# Without mise:
./bin/format/rust
./bin/lint/rust
```

## Formatting Rules

Code formatting is enforced using `rustfmt` with nightly features. Configuration is in `.config/rustfmt.toml`.

| Setting                  | Value            | Description                             |
|--------------------------|------------------|-----------------------------------------|
| `max_width`              | 120              | Maximum line width                      |
| `tab_spaces`             | 2                | Spaces per indentation level            |
| `group_imports`          | StdExternalCrate | Group std, external, then crate imports |
| `imports_granularity`    | Crate            | Merge imports from the same crate       |
| `reorder_imports`        | true             | Sort imports alphabetically             |
| `reorder_impl_items`     | true             | Sort impl items alphabetically          |
| `struct_lit_single_line` | false            | Multi-line struct literals              |

Dependencies in `Cargo.toml` are sorted using `cargo-sort`.

## Linting

Code is linted using `clippy`. All default lints must pass without warnings.

## Code Organization

### Module-Level Ordering

Order items within a module by:

1. **Constants**: All `const` declarations first
2. **Type definitions**: Structs, enums, type aliases
3. **Implementations**: `impl` blocks for the types

### Impl Block Ordering

Order functions and methods within `impl` blocks by:

1. **Class vs Instance**: Associated functions (no `self`) first, then methods (with `self`)
2. **Visibility**: Public items first, then private items
3. **Alphabetical**: Within each group, sort alphabetically

```rust
impl MyStruct {
    // Associated functions - public
    pub fn new() -> Self { }

    // Associated functions - private
    fn from_internal() -> Self { }

    // Methods - public
    pub fn calculate(&self) -> f64 { }
    pub fn process(&mut self) { }

    // Methods - private
    fn helper(&self) -> bool { }
    fn validate(&self) -> bool { }
}
```

In test modules, fall back to purely alphabetical ordering when the associated/method/public/private structure
doesn't apply. See [testing](testing.md) for test-specific conventions.
