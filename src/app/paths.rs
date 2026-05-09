use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Paths {
    pub config_dir: PathBuf,
    pub config_file: PathBuf,
    pub state_dir: PathBuf,
    pub state_file: PathBuf,
    pub log_dir: PathBuf,
}

impl Paths {
    pub fn new(home: PathBuf) -> Self {
        let config_dir = home.join(".config").join("eternalmac");
        let state_dir = home
            .join("Library")
            .join("Application Support")
            .join("eternalmac");
        let log_dir = home.join("Library").join("Logs").join("eternalmac");
        Self {
            config_file: config_dir.join("config.toml"),
            config_dir,
            state_file: state_dir.join("state.json"),
            state_dir,
            log_dir,
        }
    }
}
