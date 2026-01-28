---
name: write-tests
description: Write tests for the current changeset
---

# Write Tests

## Context

Current changes to write tests for:

```sh
!jj show
```

## Task

Review the current changes and write tests following `docs/development/testing.md` and `docs/development/code-style.md`.
Validate your work using `mise run test` and ensure all code is properly formatted using `mise run format`.

## Style Guidelines

**No inline comments** - Tests should be self-documenting through clear variable names and structure.

**Setup + space + assertion** - Separate setup from assertions with a blank line:

```rust
#[test]
fn it_does_a_thing() {
    let foo = "foo";
    let bar = "bar";

    assert_ne!(foo, bar);
}

#[test]
fn multiple_assertions() {
    let foo = "foo";
    let bar = "bar";

    assert_ne!(foo, bar);

    let baz = "baz";

    assert_ne!(foo, baz);
}
```

When a test has multiple logical steps, separate each setup+assertion group with a blank line.
