pub fn install_formula_args(packages: &[String]) -> Option<Vec<String>> {
    if packages.is_empty() {
        return None;
    }

    let mut args = vec!["install".to_string()];
    args.extend(packages.iter().cloned());
    Some(args)
}

pub fn tap_args(tap: &str) -> Vec<String> {
    vec!["tap".into(), tap.into()]
}

pub fn install_cask_args(cask: &str) -> Vec<String> {
    vec!["install".into(), "--cask".into(), cask.into()]
}
