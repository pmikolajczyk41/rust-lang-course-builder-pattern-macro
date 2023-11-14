.PHONY: build-macro build-all test lint clean help

build-macro: ## Build macro crate
	cargo build --release --manifest-path builder-pattern-macro/Cargo.toml

build-all: build-macro ## Build both macro crate and test crate
	cargo build --release --manifest-path macro-tests/Cargo.toml

test: ## Run macro tests
	TRYBUILD=overwrite cargo test --manifest-path macro-tests/Cargo.toml --release || exit 1

lint: ## Check linting rules
	cargo fmt --all --check --manifest-path macro-tests/Cargo.toml
	cargo fmt --all --check --manifest-path builder-pattern-macro/Cargo.toml

clean: ## Clean all the workspace build files
	cargo clean --manifest-path macro-tests/Cargo.toml
	cargo clean --manifest-path builder-pattern-macro/Cargo.toml

help: ## Displays this help
	@awk 'BEGIN {FS = ":.*##"; printf "Usage:\n  make \033[1;36m<target>\033[0m\n\nTargets:\n"} /^[a-zA-Z0-9_-]+:.*?##/ { printf "  \033[1;36m%-25s\033[0m %s\n", $$1, $$2 }' $(MAKEFILE_LIST)
