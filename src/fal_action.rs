use std::process::Command;

pub fn no_action(_: &str) {}

pub fn open_in_browser(url: &str) {
    // find browser
    // open url in browser
}

pub fn compute_expression(expression: &str) {
    // parse expression
    // evaluate
}

pub fn navigate_file_system(path: &str) {
    // if file open
    // else open file explorer
}

pub fn execute_shell_command(command: &str) {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", command])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()
            .expect("failed to execute process")
    };
}

pub fn launch_program(path: &str) {
    // if program name run
    // else run program at path
    Command::new(path)
        .output()
        .expect("failed to execute program");
}

pub fn custom(data: &str) {
    // todo
}
