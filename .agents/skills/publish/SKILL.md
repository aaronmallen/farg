---
name: publish
description: Prepare and publish a new crate release
arguments:
  - name: version
    description: The version number to publish (e.g., 0.5.0)
    required: true
---

# Publish

## Context

Current version in Cargo.toml:

```sh
!grep '^version' Cargo.toml
```

Current changelog state:

```sh
!cat CHANGELOG.md
```

Current README:

```sh
!cat README.md
```

## Task

Prepare and publish version **$version** of the crate.

### Steps

1. **Bump version** — Update `version` in `Cargo.toml` to the new version. Run `mise run build` to
    update `Cargo.lock`.
2. **Update CHANGELOG.md** — Convert the `## [Unreleased]` section into a versioned release:
    - Insert a new `## [v$version] - YYYY-MM-DD` header (today's date) immediately after `## [Unreleased]`
    - Update the `[unreleased]` comparison link at the bottom to compare the new version tag against `main`
    - Add a new `[v$version]` comparison link comparing the previous version to the new one
3. **Update README.md** — Review the unreleased changelog entries and update the README to reflect any new or
    changed functionality:
    - Update version numbers in `Cargo.toml` dependency snippets (e.g., `farg = "X.Y"`)
    - Add sections or examples for significant new features
    - Update the color spaces table, feature flags table, or other reference material if they changed
    - Remove documentation for removed features
    - Keep the existing structure and writing style
4. **Format** - Run the `/format` skill to ensure the code is formatted correctly.
5. **Run tests** — Run `mise run test` to confirm everything passes.
6. **Commit** — Use the `/commit` skill to create a commit with the message:

    ```txt
    Prepare v$version release
    ```

    Include a body describing what changed (version bump, changelog, README updates).
7. **Publish** — Run `mise run publish` to publish the crate to crates.io.
