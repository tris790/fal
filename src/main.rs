mod components;
mod fal_action;
mod fal_app;
mod fal_message;

use fal_app::*;

fn main() {
    let mut fal_app = FalApp::new();
    fal_app.run();
}
