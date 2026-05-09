use assert_cmd::Command;
use predicates::str::contains;

#[test]
fn root_help_mentions_devserver_goal() {
    Command::cargo_bin("eternalMac")
        .unwrap()
        .arg("--help")
        .assert()
        .success()
        .stdout(contains("Turn a Mac Mini into a personal devserver"));
}
