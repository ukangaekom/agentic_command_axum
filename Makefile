PROJECT := agentic_command_axum

.PHONY: help run build release check fmt fmt-check test clean

help:
	@printf "Available targets:\n"
	@printf "  make run        Run the development server\n"
	@printf "  make release    Run the optimized release build\n"
	@printf "  make build      Build the debug binary\n"
	@printf "  make check      Run cargo check\n"
	@printf "  make fmt        Format Rust source files\n"
	@printf "  make fmt-check  Check Rust formatting\n"
	@printf "  make test       Run Rust tests\n"
	@printf "  make clean      Remove build artifacts\n"

run:
	cargo run --bin $(PROJECT)

release:
	cargo run --release --bin $(PROJECT)

build:
	cargo build --bin $(PROJECT)
	

check:
	cargo check --locked

fmt:
	cargo fmt

fmt-check:
	cargo fmt -- --check

test:
	cargo test --locked

clean:
	cargo clean
