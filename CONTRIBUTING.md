# Contributing / 贡献指南

Thanks for contributing! This project uses pre-commit hooks and license headers.

感谢贡献！本项目启用了 pre-commit 钩子和 License Header 检查。

---

## Quick Setup / 快速设置

```bash
# Install pre-commit
pipx install pre-commit
# or: pip install pre-commit

# Enable hooks
pre-commit install --hook-type pre-commit --hook-type pre-push
```

## Required Checks / 必须通过的检查

Pre-commit hooks run automatically on commit/push:

- `cargo fmt --check`
- `cargo check`
- `cargo clippy -- -D warnings`
- `cargo test`

You can run them manually:

```bash
pre-commit run --all-files
```

## Code Style / 代码风格

- Format with `cargo fmt`.
- Avoid introducing unused dependencies.
- Keep APIs documented (rustdoc is required).

## Reporting Issues / 提交问题

- Provide steps to reproduce.
- Include Rust version and OS.
- Attach logs or error messages when possible.

## Pull Requests / 提交 PR

- Keep changes focused and small.
- Update docs and examples for public API changes.
- Ensure tests and pre-commit checks pass.
