pub fn install_formula_args(packages: &[String]) -> Vec<String> {
    let mut args = vec!["install".to_string()];
    args.extend(packages.iter().cloned());
    args
}

pub fn install_cask_args(cask: &str) -> Vec<String> {
    vec!["install".into(), "--cask".into(), cask.into()]
}
