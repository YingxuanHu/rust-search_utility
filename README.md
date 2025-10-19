# Rust Search Utility (`grep`)

This repository contains a Rust implementation of a `grep`-style command-line search tool. It supports literal string matching across one or more files with the following options:

- `-i` &nbsp;Case-insensitive search
- `-n` &nbsp;Print line numbers for each match
- `-v` &nbsp;Invert match (show non-matching lines)
- `-r` &nbsp;Recursive directory search
- `-f` &nbsp;Prefix matches with the source filename
- `-c` &nbsp;Highlight matches in red using ANSI colour codes
- `-h`, `--help` &nbsp;Display usage information
- `--` &nbsp;Treat all subsequent arguments as positional (useful for patterns starting with `-`)

The binary accepts options before or after the search pattern for a flexible invocation style.

## Project Layout

- `grep/` &nbsp;Cargo project containing the implementation, fixtures, and automated tests
- `a2-public-tests/` &nbsp;Released test harness, left untouched for reference
- `grep/tests/` &nbsp;Markdown fixtures used by both coursework instructions and automated tests
- `grep/itests/cli.rs` &nbsp;Integration tests covering the required behaviours and edge cases

## Build & Run

```bash
cd grep
cargo build
cargo run -- Utility tests/grep.md
```

Examples:

- `cargo run -- Utility tests/grep.md -n` &rarr; include line numbers
- `cargo run -- Utility tests -r -f` &rarr; recursive search with filenames
- `cargo run -- Utility tests/grep.md -c` &rarr; colour-highlighted matches

## Automated Tests

Integration coverage (edge cases, flexible argument order, recursive search, colour output):

```bash
cd grep
cargo test
```

## Packaging for Submission

From the parent directory of `grep/`, replace `123456789` with your student number:

```bash
tar zcvf 123456789.tar.gz grep
```

The archive will include `Cargo.toml`, `Cargo.lock`, the source code under `src/`, the fixtures in `tests/`, and the integration tests in `itests/`. After extraction, `cargo run` and `cargo test` should both succeed without additional setup.
