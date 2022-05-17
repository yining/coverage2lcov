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

.PHONY: clean
clean:
	cargo clean
	rm -f $(HOME)/bin/coverage2lcov

.PHONY: test
test:
	cargo test

.PHONY: build
build: test
	cargo build && cargo doc

.PHONY: install
install: build
	cp ./target/debug/coverage2lcov \
		$(HOME)/bin/coverage2lcov

.PHONY: coverage-gen
coverage-gen: coverage-cargo-llvm

.PHONY: coverage-grcov
coverage-grcov:
	export RUSTFLAGS="-C instrument-coverage"
	export LLVM_PROFILE_FILE='profraw-files/coverage-%p-%m.profraw'
	cargo test
	grcov . --binary-path ./target/debug -s . -t lcov --branch \
		--ignore-not-existing --ignore "/*" > $(COVERAGE_LCOV_DATA_FILE)
	# grcov . --binary-path ./target/debug -s . -t html --branch --ignore-not-existing  -o $(COVERAGE_HTML_DIR)
	genhtml -output-directory $(COVERAGE_HTML_DIR) \
		--show-details --highlight --ignore-errors source --legend \
		$(COVERAGE_LCOV_DATA_FILE)

.PHONY: coverage-cargo-llvm
coverage-cargo-llvm:
	# NOTE: by default `cargo llvm-cov` runs `cargo test`
	cargo llvm-cov --lcov --output-path $(COVERAGE_LCOV_DATA_FILE)
	cargo llvm-cov --html --output-dir $(COVERAGE_HTML_DIR)
