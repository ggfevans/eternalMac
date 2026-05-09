#[derive(Debug, Clone)]
pub struct Definition {
    pub label: String,
    pub program_arguments: Vec<String>,
    pub run_at_load: bool,
    pub keep_alive: bool,
}

fn escape_xml(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

pub fn render(definition: &Definition) -> String {
    let label = escape_xml(&definition.label);
    let args = definition
        .program_arguments
        .iter()
        .map(|arg| format!("<string>{}</string>", escape_xml(arg)))
        .collect::<Vec<_>>()
        .join("");

    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0"><dict>
<key>Label</key><string>{}</string>
<key>ProgramArguments</key><array>{}</array>
<key>RunAtLoad</key><{}/>
<key>KeepAlive</key><{}/>
</dict></plist>"#,
        label,
        args,
        if definition.run_at_load {
            "true"
        } else {
            "false"
        },
        if definition.keep_alive {
            "true"
        } else {
            "false"
        },
    )
}
