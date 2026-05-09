use eternalmac::tooling::dependencies::{build_dependency_plan, Inspector};

#[test]
fn dependency_plan_includes_only_missing_tools() {
    let inspector = Inspector::from_installed([
        ("et", false),
        ("tmux", true),
        ("mutagen", false),
        ("tailscale-app", false),
    ]);

    let plan = build_dependency_plan(&inspector);
    assert_eq!(plan.formulae, vec!["et", "mutagen"]);
    assert_eq!(plan.casks, vec!["tailscale-app"]);
}
