use std::process::Command;

pub enum FalAction {
    NONE,
    LAUNCH(String),
    CMD(String),
}

impl FalAction {
    pub fn execute(&self) {
        match self {
            Self::LAUNCH(executable) => {
                Command::new(executable)
                    .output()
                    .expect("failed to execute");
            }
            Self::CMD(cmd) => {
                if cfg!(target_os = "windows") {
                    Command::new("cmd")
                        .args(&["/C", cmd])
                        .output()
                        .expect("failed to execute process")
                } else {
                    Command::new("sh")
                        .arg("-c")
                        .arg(cmd)
                        .output()
                        .expect("failed to execute process")
                };
            }
            _ => (),
        }
    }
}
