use crate::platform_api;
use regex::Regex;

#[derive(Debug)]
pub struct FalCommand {
    pub name: String,
    pub regex_pattern: Regex,
    pub action: fn(String) -> Vec<String>,
}

impl FalCommand {
    pub fn new(
        name: String,
        regex_pattern: Regex,
        action: fn(String) -> Vec<String>,
    ) -> FalCommand {
        FalCommand {
            name,
            regex_pattern,
            action,
        }
    }
}

pub struct FalCommandParser {
    commands: Vec<FalCommand>,
}

impl FalCommandParser {
    pub fn new() -> FalCommandParser {
        let search_command = FalCommand::new(
            String::from("search"),
            Regex::new("^!").expect("invalid regex for search command"),
            |input| match Regex::new(&input.as_str()[1..]) {
                Ok(current_search_regex) => {
                    let programs = platform_api::get_programs();
                    let output = programs
                        .into_iter()
                        .filter(|x| current_search_regex.is_match(x))
                        .collect();
                    println!("programs {:?}", output);
                    output
                }
                Err(_) => vec![],
            },
        );

        let calc_command = FalCommand::new(
            String::from("calc"),
            Regex::new("^=").expect("invalid regex pattern for calc command"),
            |input| vec![String::from("=54")],
        );

        FalCommandParser {
            commands: vec![search_command, calc_command],
        }
    }

    pub fn parse(&self, input: String) -> Vec<String> {
        for command in &self.commands {
            if command.regex_pattern.is_match(input.as_str()) {
                println!("pattern matched {} {}", input, command.regex_pattern);
                return (command.action)(input);
            }
        }
        return vec![];
    }
}
