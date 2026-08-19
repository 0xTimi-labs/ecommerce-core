.PHONY: check fmt-check fmt lint test contract-lint web-check

check: fmt-check lint test contract-lint web-check

fmt-check:
	cargo fmt --all -- --check

fmt:
	cargo fmt --all

lint:
	cargo clippy --workspace --all-targets -- -D warnings

test:
	cargo test --workspace -- --skip ignore

contract-lint:
	buf lint contracts

web-check:
	cd apps/web && bun run check
