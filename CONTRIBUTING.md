# Contributing

Thanks for contributing! This project uses pre-commit hooks and license header checks.

---

## Quick Setup

```bash
# Install pre-commit
pipx install pre-commit
# or: pip install pre-commit

# Enable hooks
pre-commit install --hook-type pre-commit --hook-type pre-push
```

## Required Checks

Pre-commit hooks run automatically on commit/push:

- `cargo fmt --check`
- `cargo check`
- `cargo clippy -- -D warnings`
- `cargo test`

You can run them manually:

```bash
pre-commit run --all-files
```

## Code Style

- Format with `cargo fmt`.
- Avoid introducing unused dependencies.
- Keep APIs documented (rustdoc is required).

## Reporting Issues

- Provide steps to reproduce.
- Include Rust version and OS.
- Attach logs or error messages when possible.

## Pull Requests

- Keep changes focused and small.
- Update docs and examples for public API changes.
- Ensure tests and pre-commit checks pass.
