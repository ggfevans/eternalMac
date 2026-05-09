#[derive(Debug, Clone)]
pub struct Definition {
    pub label: String,
    pub program_arguments: Vec<String>,
    pub run_at_load: bool,
    pub keep_alive: bool,
}

pub fn render(definition: &Definition) -> String {
    let args = definition
        .program_arguments
        .iter()
        .map(|arg| format!("<string>{arg}</string>"))
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
        definition.label,
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
