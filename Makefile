.PHONY: help setup lint build publish-lib publish-cli
.DEFAULT_GOAL := help

help:
	@printf "%s\n" \
	"Usage: make <target>" \
	"" \
	"Targets:" \
	"  setup        Install pre-commit hooks (requires pre-commit)" \
	"  lint         Run pre-commit hooks on all files" \
	"  build        Build workspace (cargo build)" \
	"  publish-lib  Publish tui-banner to crates.io" \
	"  publish-cli  Publish tui-banner-cli to crates.io"

setup:
	@command -v pre-commit >/dev/null 2>&1 || { \
		echo "pre-commit not found. Install with: pipx install pre-commit (or pip install pre-commit)"; \
		exit 1; \
	}
	pre-commit install --hook-type pre-commit --hook-type pre-push

lint:
	pre-commit run --all-files

build:
	cargo build --workspace

publish-lib:
	cargo publish -p tui-banner

publish-cli:
	cargo publish -p tui-banner-cli
