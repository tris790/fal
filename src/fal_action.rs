use std::process::Command;

use crate::platform_api::Program;

pub trait FalAction {
    fn execute(&mut self, input: &str);
}

pub struct ExecuteAction {
    pub launch_cmd: String,
}
pub struct LaunchAction {
    pub launch_cmd: String,
}
pub struct NoAction {}
pub struct OpenInBrowserAction {}
pub struct ComputeExpressionAction {}
pub struct NagivateFileSystemAction {}
pub struct CustomAction {}

impl FalAction for ExecuteAction {
    fn execute(&mut self, input: &str) {
        if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(&["/C", self.launch_cmd.as_str()])
                .output()
                .expect("failed to execute process")
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(self.launch_cmd.as_str())
                .output()
                .expect("failed to execute process")
        };
    }
}
impl FalAction for LaunchAction {
    fn execute(&mut self, file_path: &str) {
        Command::new(file_path)
            .output()
            .expect("failed to execute program");
    }
}
impl FalAction for NoAction {
    fn execute(&mut self, input: &str) {
        todo!()
    }
}
impl FalAction for OpenInBrowserAction {
    fn execute(&mut self, input: &str) {
        todo!()
    }
}
impl FalAction for ComputeExpressionAction {
    fn execute(&mut self, input: &str) {
        todo!()
    }
}
impl FalAction for NagivateFileSystemAction {
    fn execute(&mut self, input: &str) {
        todo!()
    }
}
impl FalAction for CustomAction {
    fn execute(&mut self, input: &str) {
        todo!()
    }
}
