use eternalmac::sync::service::build_pair;

#[test]
fn sync_pair_defaults_to_last_write_wins() {
    let pair = build_pair("project", "~/src/project", "~/remote/project");
    assert_eq!(pair.name, "project");
    assert_eq!(pair.mode, "last-write-wins");
}
