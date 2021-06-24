#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod components;
mod fal_action;
mod fal_app;
// mod fal_command;
mod fal_command2;
mod fal_message;
mod platform_api;
mod program_lister;

use fal_app::*;

fn main() {
    let mut fal_app = FalApp::new();
    fal_app.run();
}
