SHELL := bash
MAKEFLAGS += --warn-undefined-variables
MAKEFLAGS += --no-builtin-rules
.ONESHELL:
.SHELLFLAGS := -eu -o pipefail -c
.DEFAULT_GOAL := build
.DELETE_ON_ERROR:
.SUFFIXES:

COVERAGE_LCOV_DATA_FILE=lcov.info
COVERAGE_HTML_DIR=htmlcov
CARGO=cargo

.PHONY: clean
clean:
	$(CARGO) clean
	rm -f  ./$(COVERAGE_LCOV_DATA_FILE)
	rm -rf ./$(COVERAGE_HTML_DIR)

.PHONY: test
test:
	$(CARGO) test --no-fail-fast --all-features

.PHONY: check
check:
	$(CARGO) clippy --tests -- -Dclippy::all -Dclippy::pedantic -Dclippy::cargo
	$(CARGO) check --all-features

.PHONY: build
build: test
	$(CARGO) build && cargo doc

.PHONY: install
install: test
	$(CARGO) install --path $(PWD)

.PHONY: coverage-gen
coverage-gen: coverage-cargo-llvm

.PHONY: coverage-grcov
coverage-grcov:
	export RUSTFLAGS="-C instrument-coverage"
	export LLVM_PROFILE_FILE='profraw-files/coverage-%p-%m.profraw'
	$(CARGO) test --no-fail-fast
	grcov . --binary-path ./target/debug -s . -t lcov --branch \
		--ignore-not-existing --ignore "/*" > $(COVERAGE_LCOV_DATA_FILE)
	# grcov . --binary-path ./target/debug -s . -t html --branch --ignore-not-existing  -o $(COVERAGE_HTML_DIR)
	genhtml -output-directory $(COVERAGE_HTML_DIR) \
		--show-details --highlight --ignore-errors source --legend \
		$(COVERAGE_LCOV_DATA_FILE)

.PHONY: coverage-cargo-llvm
coverage-cargo-llvm:
	# NOTE: by default `cargo llvm-cov` runs `cargo test`
	$(CARGO) llvm-cov --lcov --output-path $(COVERAGE_LCOV_DATA_FILE)
	$(CARGO) llvm-cov --html --output-dir $(COVERAGE_HTML_DIR)
