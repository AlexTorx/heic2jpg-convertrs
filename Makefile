.PHONY: all build test clean coverage

# Default target
all: build

# Build the project
build:
	cargo build --release

# Run tests
test:
	cargo test

# Run tests with verbose output
test-verbose:
	cargo test -- --nocapture

# Clean build artifacts
clean:
	cargo clean
	rm -f tarpaulin-report.html

# Generate test coverage report
coverage:
	cargo tarpaulin --out html

# Install development dependencies
install-dev:
	cargo install cargo-tarpaulin

# Run clippy for linting
lint:
	cargo clippy

# Format code
format:
	cargo fmt

# Check formatting without making changes
format-check:
	cargo fmt -- --check

# Help command
help:
	@echo "Available commands:"
	@echo "  make build        - Build the project in release mode"
	@echo "  make test         - Run tests"
	@echo "  make test-verbose - Run tests with verbose output"
	@echo "  make clean        - Clean build artifacts"
	@echo "  make coverage     - Generate test coverage report"
	@echo "  make install-dev  - Install development dependencies"
	@echo "  make lint         - Run clippy for linting"
	@echo "  make format       - Format code"
	@echo "  make format-check - Check code formatting" 