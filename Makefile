# Makefile for magicbox-infra.
# Run `make` (or `make help`) to list the available actions.

.DEFAULT_GOAL := help

.PHONY: help lint test build build-debug

help: ## Show this help
	@echo "Available targets:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) \
		| awk 'BEGIN {FS = ":.*?## "} {printf "  %-12s %s\n", $$1, $$2}'

lint: ## Run clippy lints
	cargo clippy -- -D warnings

test: ## Run the test suite
	cargo test

build: lint test ## Lint, test, then build the release binary
	cargo build --release

build-debug: lint test ## Build the debug binary (cargo build)
	cargo build
