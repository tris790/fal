use regex::Regex;

#[derive(Debug)]
pub struct BestCommand {
    pub name: String,
    pub regex_pattern: Regex,
    pub action: fn(String) -> Vec<String>,
}

impl BestCommand {
    pub fn new(
        name: String,
        regex_pattern: Regex,
        action: fn(String) -> Vec<String>,
    ) -> BestCommand {
        BestCommand {
            name,
            regex_pattern,
            action,
        }
    }
}

pub struct BestCommandParser {
    commands: Vec<BestCommand>,
}

impl BestCommandParser {
    pub fn new() -> BestCommandParser {
        let search_command = BestCommand::new(
            String::from("search"),
            Regex::new("^!").expect("invalid regex for search command"),
            |input| match Regex::new(&input.as_str()[1..]) {
                Ok(current_search_regex) => {
                    let programs = vec![
                        String::from("chrome"),
                        String::from("vs code"),
                        String::from("calc"),
                    ];
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

        let calc_command = BestCommand::new(
            String::from("calc"),
            Regex::new("^=").expect("invalid regex pattern for calc command"),
            |input| vec![String::from("=54")],
        );

        BestCommandParser {
            commands: vec![search_command, calc_command],
        }
    }

    pub fn on_textbox_changed(&self, input: String) -> Vec<String> {
        for command in &self.commands {
            if command.regex_pattern.is_match(input.as_str()) {
                println!("pattern matched {} {}", input, command.regex_pattern);
                return (command.action)(input);
            }
        }
        return vec![];
    }
}
