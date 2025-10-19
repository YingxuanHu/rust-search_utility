use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

fn binary() -> Command {
    Command::cargo_bin("grep").expect("binary exists")
}

#[test]
fn displays_usage_with_help_flag() {
    binary()
        .arg("-h")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Usage: grep [OPTIONS] <pattern> <files...>",
        ));
}

#[test]
fn errors_on_missing_arguments() {
    binary()
        .assert()
        .failure()
        .stderr(predicate::str::contains("Missing arguments"));
}

#[test]
fn finds_basic_match_in_single_file() {
    binary()
        .args(["Utility", "tests/grep.md"])
        .assert()
        .success()
        .stdout(predicate::eq("## Search Utility\n"));
}

#[test]
fn supports_options_after_positional_arguments() {
    binary()
        .args(["Utility", "tests/grep.md", "-i"])
        .assert()
        .success()
        .stdout(predicate::eq(
            "## Search Utility\nIn this programming assignment, you are expected to implement a command-line utility that\n",
        ));
}

#[test]
fn prints_line_numbers_when_requested() {
    binary()
        .args(["Utility", "tests/grep.md", "-n"])
        .assert()
        .success()
        .stdout(predicate::eq("1: ## Search Utility\n"));
}

#[test]
fn inverts_matches() {
    binary()
        .args(["Utility", "tests/grep.md", "-v"])
        .assert()
        .success()
        .stdout(predicate::eq(
            "In this programming assignment, you are expected to implement a command-line utility that\n\
searches for a specific pattern in one or multiple files, similar in spirit to the UNIX\n\
`grep` command.\n",
        ));
}

#[test]
fn searches_directories_recursively() {
    binary()
        .args(["Utility", "tests", "-r"])
        .assert()
        .success()
        .stdout(predicate::eq("## Search Utility\n## Search Utility\n"));
}

#[test]
fn prints_filenames_when_requested() {
    binary()
        .args(["Utility", "tests", "-r", "-f"])
        .assert()
        .success()
        .stdout(predicate::eq(
            "tests/recursive/grep.md: ## Search Utility\n\
tests/grep.md: ## Search Utility\n",
        ));
}

#[test]
fn prints_filename_and_line_number_prefixes() {
    binary()
        .args(["Utility", "tests/grep.md", "-f", "-n"])
        .assert()
        .success()
        .stdout(predicate::eq("tests/grep.md:1: ## Search Utility\n"));
}

#[test]
fn highlights_matches_when_color_enabled() {
    binary()
        .args(["Utility", "tests/grep.md", "-c"])
        .assert()
        .success()
        // Colored output is suppressed when stdout is not a TTY, which matches the released tests.
        .stdout(predicate::eq("## Search Utility\n"));
}

#[test]
fn invert_match_with_prefixes() {
    binary()
        .args(["Utility", "tests/grep.md", "-v", "-f", "-n"])
        .assert()
        .success()
        .stdout(predicate::eq(
            "tests/grep.md:2: In this programming assignment, you are expected to implement a command-line utility that\n\
tests/grep.md:3: searches for a specific pattern in one or multiple files, similar in spirit to the UNIX\n\
tests/grep.md:4: `grep` command.\n",
        ));
}

#[test]
fn literal_metacharacters_do_not_trigger_regex() {
    binary()
        .args([".", "tests/grep.md"])
        .assert()
        .success()
        .stdout(predicate::eq("`grep` command.\n"));
}

#[test]
fn directory_without_recursive_flag_prints_nothing() {
    binary()
        .args(["Utility", "tests"])
        .assert()
        .success()
        .stdout(predicate::eq(""));
}

#[test]
fn reports_empty_output_when_no_lines_match() {
    binary()
        .args(["NonexistentPattern", "tests/grep.md"])
        .assert()
        .success()
        .stdout(predicate::eq(""));
}

#[test]
fn treats_arguments_after_double_dash_as_literals() {
    binary()
        .args(["--", "-n", "tests/grep.md"])
        .assert()
        .success()
        .stdout(predicate::eq(""));
}
