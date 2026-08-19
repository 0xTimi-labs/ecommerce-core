.PHONY: check fmt-check fmt lint test contract-check web-check

check: fmt-check lint test contract-check web-check

fmt-check:
	cargo fmt --all -- --check

fmt:
	cargo fmt --all

lint:
	cargo clippy --workspace --all-targets --all-features -- -D warnings

test:
	cargo test --workspace

contract-check:
	@command -v buf >/dev/null 2>&1 && buf lint || echo "buf not installed locally, skipping"

web-check:
	cd apps/web && bun run check
