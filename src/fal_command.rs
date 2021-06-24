use std::collections::btree_map::BTreeMap;

struct FalResult {
    text: String,
    action: bool,
}

struct CommandResult {
    results: Vec<FalResult>,
}

#[derive(Copy, Clone)]
pub enum FalCommandEnum {
    Calculation,
    Search,
    Cmd,
}

pub struct FalCommand {
    command: FalCommandEnum,
    text: String,
}

impl FalCommand {
    pub fn execute(&self) -> String {
        match self.command {
            FalCommandEnum::Calculation => String::from("Calculation output"),
            FalCommandEnum::Search => String::from("Search output"),
            FalCommandEnum::Cmd => String::from("Cmd output"),
        }
    }
}
pub struct FalCommandParser {
    commands: BTreeMap<String, FalCommandEnum>,
}

impl FalCommandParser {
    pub fn new() -> FalCommandParser {
        let mut commands = BTreeMap::new();
        commands.insert(String::from("$"), FalCommandEnum::Search);
        commands.insert(String::from("!"), FalCommandEnum::Cmd);
        commands.insert(String::from("#"), FalCommandEnum::Calculation);
        FalCommandParser { commands }
    }

    pub fn parse(&self, text: String) -> Option<FalCommand> {
        if text.is_empty() {
            return None;
        };

        let text_lower_case = text.to_lowercase();
        for (prefix, command) in &self.commands {
            let pattern = prefix.as_str();
            if text_lower_case.starts_with(pattern) {
                return Some(FalCommand {
                    command: command.to_owned(),
                    text: text[prefix.len()..].to_string(),
                });
            }
        }

        return None;
    }
}
