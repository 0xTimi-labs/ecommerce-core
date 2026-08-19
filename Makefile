.PHONY: check fmt-check fmt lint test contract-lint web-check

check: fmt-check lint test contract-lint web-check

fmt-check:
	cargo fmt --all -- --check

fmt:
	cargo fmt --all

lint:
	# 1. 严格检查生产代码（lib/bin）：绝对禁止 unwrap, panic, dbg, println 等
	cargo clippy --workspace --lib --bins --all-features -- -D clippy::unwrap_used -D clippy::expect_used -D clippy::panic -D clippy::panic_in_result_fn -D clippy::todo -D clippy::unimplemented -D clippy::dbg_macro -D clippy::print_stdout -D clippy::print_stderr -D warnings
	# 2. 检查全量目标（tests/examples 等）：执行基线质量规范，放行测试断言
	cargo clippy --workspace --all-targets --all-features -- -D warnings

test:
	cargo test --workspace

contract-lint:
	@command -v buf >/dev/null 2>&1 && buf lint contracts || echo "buf not installed locally, skipping (CI enforces)"

web-check:
	cd apps/web && bun run check
