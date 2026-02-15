use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct BuildConfig {
	pub package: String,
	pub release: bool,
    pub target: Option<String>,
	pub features: Vec<String>,
}

const CONFIG_DIR: &str = ".smart-build";
const CONFIG_FILE: &str = "config.json";

pub fn load_config() -> Option<BuildConfig> {
	let path = Path::new(CONFIG_DIR).join(CONFIG_FILE);
	if !path.exists() {
		return None;
	}

	let data = fs::read_to_string(path).ok()?;
	serde_json::from_str(&data).ok()
}

pub fn save_config(config: &BuildConfig) {
	let dir = Path::new(CONFIG_DIR);
	if !dir.exists() {
		fs::create_dir(dir).ok();
	}

	let path = dir.join(CONFIG_FILE);
	if let Ok(json) = serde_json::to_string_pretty(config) {
		fs::write(path, json).ok();
	}
}
