#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod components;
mod fal_action;
mod fal_app;
mod fal_command;
mod fal_config;
mod fal_message;
mod platform_api;

use std::{fs::File, path::Path};

use fal_app::*;
use fal_config::FalConfig;

fn create_config(config_path: &Path) -> FalConfig {
    let config = FalConfig::default();
    let file = File::create(config_path).unwrap();
    serde_json::to_writer(file, &config).unwrap();
    config
}

fn main() {
    let config_path = Path::new("fal_config.json");
    let config: FalConfig = match File::open(config_path) {
        Ok(file) => {
            if let Ok(config) = serde_json::from_reader(file) {
                config
            } else {
                create_config(&config_path)
            }
        }
        Err(_) => create_config(&config_path),
    };

    let mut fal_app = FalApp::new(config);
    fal_app.run();
}
