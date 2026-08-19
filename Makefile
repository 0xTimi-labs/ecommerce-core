.PHONY: check fmt-check fmt lint test contract-check web-check deny-check codegen

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

deny-check:
	@command -v cargo-deny >/dev/null 2>&1 && cargo deny check || echo "cargo-deny not installed locally, skipping"

codegen:
	@command -v buf >/dev/null 2>&1 && buf generate || echo "buf not installed locally, skipping"
