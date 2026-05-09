use assert_cmd::Command;
use predicates::prelude::PredicateBooleanExt;
use predicates::str::contains;

#[test]
fn explicit_help_mentions_devserver_goal() {
    Command::cargo_bin("eternalMac")
        .unwrap()
        .arg("--help")
        .assert()
        .success()
        .stdout(
            contains("Turn a Mac Mini into a personal devserver")
                .and(contains("Usage: eternalMac")),
        );
}

#[test]
fn bare_invocation_shows_help_output() {
    Command::cargo_bin("eternalMac")
        .unwrap()
        .assert()
        .success()
        .stdout(
            contains("Turn a Mac Mini into a personal devserver")
                .and(contains("Usage: eternalMac")),
        );
}
