[![CI](../../actions/workflows/ci.yml/badge.svg)](../../actions/workflows/ci.yml)

# README

## What

A simple program to generate `lcov` file from data file generated by [coveragepy](https://github.com/nedbat/coveragepy ), when an older version of which does not have `coverage lcov` command.

The data in the generated `lcov` file is very limited: only `DA` record of uncovered lines in source files.

## Usage

```bash
coverage2lcov .coverage > lcov.info
```

## Install

```bash
cargo install --git https://github.com/yining/coverage2lcov
```

