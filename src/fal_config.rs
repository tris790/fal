use crate::platform_api::Program;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct FalConfig {
    pub user_defined_programs: Vec<Program>,
}

impl Default for FalConfig {
    fn default() -> Self {
        FalConfig {
            user_defined_programs: vec![
                Program {
                    name: String::from("VS Code"),
                    launch_cmd: String::from("code"),
                },
                Program {
                    name: String::from("Calculator"),
                    launch_cmd: String::from("calc"),
                },
                Program {
                    name: String::from("VLC"),
                    launch_cmd: String::from("vlc"),
                },
                Program {
                    name: String::from("Terminal"),
                    launch_cmd: String::from("wt"),
                },
                Program {
                    name: String::from("Discord"),
                    launch_cmd: String::from("discord"),
                },
                Program {
                    name: String::from("File Explorer"),
                    launch_cmd: String::from("explorer"),
                },
            ],
        }
    }
}
