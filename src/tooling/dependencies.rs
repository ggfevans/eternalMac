use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct Inspector {
    installed: BTreeMap<String, bool>,
}

impl Inspector {
    pub fn from_installed<const N: usize>(items: [(&str, bool); N]) -> Self {
        let mut installed = BTreeMap::new();
        for (name, value) in items {
            installed.insert(name.to_string(), value);
        }
        Self { installed }
    }

    pub fn has(&self, name: &str) -> bool {
        self.installed.get(name).copied().unwrap_or(false)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DependencyPlan {
    pub formulae: Vec<String>,
    pub casks: Vec<String>,
}

pub fn build_dependency_plan(inspector: &Inspector) -> DependencyPlan {
    let formulae = ["et", "tmux", "mutagen"]
        .into_iter()
        .filter(|name| !inspector.has(name))
        .map(String::from)
        .collect();

    let casks = ["tailscale-app"]
        .into_iter()
        .filter(|name| !inspector.has(name))
        .map(String::from)
        .collect();

    DependencyPlan { formulae, casks }
}
