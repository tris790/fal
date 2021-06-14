mod fal_action;
mod fal_app;
mod fal_list_element;
mod fal_message;

mod components;

use fal_app::*;

fn main() {
    let mut fal_app = FalApp::new();
    fal_app.run();
}
