.PHONY: check fmt-check fmt lint test contract-check web-check script-test deny-check codegen

check: codegen fmt-check lint test contract-check web-check script-test

codegen:
	@command -v buf >/dev/null 2>&1 && buf generate || (echo "错误: 未检测到 buf CLI，请先安装 buf (brew install bufbuild/buf/buf)" && exit 1)

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

script-test:
	@command -v bun >/dev/null 2>&1 && bun test scripts/*.test.ts || echo "bun not installed locally, skipping script tests"

deny-check:
	@command -v cargo-deny >/dev/null 2>&1 && cargo deny check || echo "cargo-deny not installed locally, skipping"
