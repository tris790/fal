use crate::{
    components::result_component::ResultElement,
    fal_action::{ExecuteAction, NoAction},
    platform_api,
};
use regex::{Regex, RegexBuilder};

pub struct FalCommand {
    pub name: String,
    pub regex_pattern: Regex,
    pub on_command_matched: fn(&str) -> Vec<ResultElement>,
}

impl FalCommand {
    pub fn new(
        name: String,
        regex_pattern: Regex,
        on_command_matched: fn(&str) -> Vec<ResultElement>,
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
}

impl FalCommandParser {
    pub fn new() -> FalCommandParser {
        let search_command = FalCommand::new(
            String::from("Search"),
            Regex::new("^!").expect("invalid regex for search command"),
            |input| match RegexBuilder::new(&input[1..])
                .case_insensitive(true)
                .build()
            {
                Ok(reg) => {
                    let programs = platform_api::get_programs();
                    let output: Vec<ResultElement> = programs
                        .into_iter()
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
            String::from("Calc"),
            Regex::new("^=").expect("invalid regex pattern for calc command"),
            |input| {
                let expression = &input[1..];
                vec![ResultElement {
                    text: expression.to_owned(),
                    action: Box::new(NoAction {}),
                }]
            },
        );
        let commands = vec![search_command, calculation_command];

        FalCommandParser { commands }
    }

    pub fn parse(&self, input: &str) -> Vec<ResultElement> {
        for command in &self.commands {
            if command.regex_pattern.is_match(input) {
                println!("pattern matched {} {}", input, command.regex_pattern);
                return (command.on_command_matched)(input);
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
