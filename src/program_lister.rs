pub struct Icon {}

pub struct Program {
    pub path: String,
    pub name: String,
    pub icon: Icon,
}

pub fn get_all_programs() -> Vec<Program> {
    return vec![
        Program {
            path: String::from("C:/Program Files/WindowsApps/Microsoft.WindowsTerminal_1.8.1521.0_x64__8wekyb3d8bbwe/WindowsTerminal.exe"),
            name: String::from("Windows Terminal"),
            icon: Icon {},
        },
        Program {
            path: String::from("C:/Users/NSA/AppData/Local/Programs/Microsoft VS Code/Code.exe"),
            name: String::from("Visual Studio Code"),
            icon: Icon {},
        },
        Program {
            path: String::from("C:/Program Files/WindowsApps/Microsoft.WindowsCalculator_10.2103.8.0_x64__8wekyb3d8bbwe/Calculator.exe"),
            name: String::from("Calculator"),
            icon: Icon {},
        },
    ];
}
