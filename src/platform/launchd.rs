use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use anyhow::Result;

#[derive(Debug, Clone)]
pub struct Definition {
    pub label: String,
    pub program_arguments: Vec<String>,
    pub environment_variables: BTreeMap<String, String>,
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
    let environment_variables = if definition.environment_variables.is_empty() {
        String::new()
    } else {
        let entries = definition
            .environment_variables
            .iter()
            .map(|(key, value)| {
                format!(
                    "<key>{}</key><string>{}</string>",
                    escape_xml(key),
                    escape_xml(value)
                )
            })
            .collect::<Vec<_>>()
            .join("");
        format!("<key>EnvironmentVariables</key><dict>{entries}</dict>")
    };

    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0"><dict>
<key>Label</key><string>{}</string>
<key>ProgramArguments</key><array>{}</array>
{}
<key>RunAtLoad</key><{}/>
<key>KeepAlive</key><{}/>
</dict></plist>"#,
        label,
        args,
        environment_variables,
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

pub fn write_plist(path: &Path, definition: &Definition) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, render(definition))?;
    Ok(())
}
