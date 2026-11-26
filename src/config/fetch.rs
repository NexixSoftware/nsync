use core::panic;
use std::{
    env,
    fs::{self},
    path::{Path, PathBuf},
};

#[cfg(target_os = "linux")]
const CONFIG_PATH: &str = ".config/nsync";
#[cfg(target_os = "windows")]
const CONFIG_PATH: &str = "Program Files\\nsync";

pub fn get_config_path() -> PathBuf {
    let format = &format!(
        "{}{}",
        env::home_dir().unwrap().display(),
        CONFIG_PATH.to_string()
    );
    let full_config_path = Path::new(&format);

    let exists = fs::exists(full_config_path).unwrap_or(false);

    if !exists {
        fs::create_dir(full_config_path).unwrap();
    }

    let owned_cfg = full_config_path.to_owned();
    owned_cfg
}
