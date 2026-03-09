use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

pub const LANGUAGES: &[&str] = &["en-US", "zh-CN"];

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub nickname: String,
    pub language: String,
}

impl Settings {
    pub fn new() -> Settings {
        Settings::default()
    }

    pub fn load_from_disk() -> Settings {
        let path = get_config_path();
        if let Ok(contents) = fs::read_to_string(path) {
            toml::from_str(&contents).unwrap_or_else(|_| Settings::default())
        } else {
            Settings::default()
        }
    }
    pub fn save_to_disk(&mut self) {
        let path = get_config_path();
        let toml = toml::to_string_pretty(&self).unwrap();
        fs::write(path, toml).unwrap();
    }
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            nickname: "Player".to_string(),
            language: "en-US".to_string(),
        }
    }
}

fn get_config_path() -> PathBuf {
    if let Some(path) = ProjectDirs::from("com", "towergames", "warp") {
        let dir = path.config_dir();
        if !dir.exists() {
            fs::create_dir_all(dir).unwrap();
        }
        return dir.join("config.toml");
    }
    PathBuf::from("config.toml")
}
