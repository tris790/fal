use std::collections::btree_map::BTreeMap;

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
        commands.insert(String::from("!"), FalCommandEnum::Cmd);
        commands.insert(String::from("#"), FalCommandEnum::Calculation);
        commands.insert(String::from(""), FalCommandEnum::Search);
        FalCommandParser { commands }
    }

    pub fn parse(&self, text: String) -> Option<FalCommand> {
        if text.is_empty() {
            return None;
        };

        let words: Vec<&str> = text.split("").collect();
        if words.is_empty() {
            return None;
        }

        let prefix = words[0].to_lowercase();
        let command = self.commands.get(prefix.as_str()).or(None).unwrap();

        return Some(FalCommand {
            command: FalCommandEnum::Calculation,
            text: text[prefix.len()..].to_string(),
        });
    }
}
