use crate::{
    components::result_component::ResultElement,
    fal_action::{ExecuteAction, NoAction},
    platform_api::{self, Program},
};
use regex::{Regex, RegexBuilder};

pub struct FalCommand {
    pub name: String,
    pub regex_pattern: Regex,
    pub on_command_matched: fn(&FalCommandParser, &str) -> Vec<ResultElement>,
}

impl FalCommand {
    pub fn new(
        name: String,
        regex_pattern: Regex,
        on_command_matched: fn(&FalCommandParser, &str) -> Vec<ResultElement>,
    ) -> FalCommand {
        FalCommand {
            name,
            regex_pattern,
            on_command_matched,
        }
    }
}

pub struct FalCommandParser {
    commands: Vec<FalCommand>,
    programs: Vec<Program>,
}

impl FalCommandParser {
    pub fn new(programs: Vec<Program>) -> FalCommandParser {
        let find_command = FalCommand::new(
            String::from("Find a program to launch"),
            Regex::new("").expect("invalid regex for search command"),
            |parser, input| match RegexBuilder::new(input).case_insensitive(true).build() {
                Ok(reg) => {
                    let output: Vec<ResultElement> = parser
                        .programs
                        .iter()
                        .filter(|x| reg.is_match(x.name.as_str()))
                        .map(|program| ResultElement {
                            text: program.name.to_owned(),
                            action: Box::new(ExecuteAction {
                                launch_cmd: program.launch_cmd.to_owned(),
                            }),
                        })
                        .collect();
                    output
                }
                Err(_) => vec![],
            },
        );

        let calculation_command = FalCommand::new(
            String::from("Calculate an expression"),
            Regex::new("^=").expect("invalid regex pattern for calc command"),
            |_, input| {
                let expression = &input[1..];
                vec![ResultElement {
                    text: expression.to_owned(),
                    action: Box::new(NoAction {}),
                }]
            },
        );
        let commands = vec![calculation_command, find_command];

        FalCommandParser { commands, programs }
    }

    pub fn parse(&self, input: &str) -> Vec<ResultElement> {
        if input.len() == 0 {
            return vec![];
        }

        for command in &self.commands {
            if command.regex_pattern.is_match(input) {
                println!("pattern matched {} {}", input, command.regex_pattern);
                return (command.on_command_matched)(self, input);
            }
        }

        if input == "help" {
            return self
                .commands
                .iter()
                .map(|cmd| ResultElement {
                    text: cmd.name.to_owned(),
                    action: Box::new(NoAction {}),
                })
                .collect();
        }
        return vec![];
    }
}
