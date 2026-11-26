// src/config/parse.rs
use serde::{Deserialize, Serialize};
use serde_json::{self, json};
use std::path::PathBuf;

use crate::config::fetch::create_config_path;

#[derive(Serialize, Deserialize)]
struct Config {
    version: String,
}

// TODO: finish defining config
fn config() {
    let config = json!({
        "version": "",
    });
}

// TODO: parse config into struct & return it to main
pub fn parse_config() {
    let config_path = create_config_path();
}
