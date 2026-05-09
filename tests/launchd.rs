use eternalmac::platform::launchd::{render, Definition};

#[test]
fn plist_render_includes_label_and_program() {
    let xml = render(&Definition {
        label: "com.eternalmac.server".into(),
        program_arguments: vec![
            "/opt/homebrew/bin/eternalMac".into(),
            "daemon".into(),
            "server".into(),
        ],
        run_at_load: true,
        keep_alive: true,
    });

    assert!(xml.contains("com.eternalmac.server"));
    assert!(xml.contains("<string>daemon</string>"));
}
