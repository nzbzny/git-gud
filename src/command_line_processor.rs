use std::collections::HashMap;

#[derive(Clone)]
pub enum CommandType {
    Add,
    Help,
    Init,
    Version,
    Unknown,
}

#[derive(PartialEq, Eq, Hash)]
pub enum FlagOption {
    Compression,
    Name,
}

pub struct Options {
    pub command: CommandType,
    pub flags: HashMap<FlagOption, Vec<String>>,
}

fn command_strs() -> HashMap<String, CommandType> {
    // TODO: lazily evaluate this globally somewhere
    vec![
        ("add".to_string(), CommandType::Add),
        ("--help".to_string(), CommandType::Help),
        ("init".to_string(), CommandType::Init),
        ("--version".to_string(), CommandType::Version),
    ]
    .into_iter()
    .collect()
}

fn process_command_flags(
    command: &CommandType,
    args: Vec<String>,
    flags: &mut HashMap<FlagOption, Vec<String>>,
) {
}

pub fn process_args(args: Vec<String>) -> Options {
    let mut command = CommandType::Unknown;

    let mut flags: HashMap<FlagOption, Vec<String>> = HashMap::new();
    let commands = command_strs();

    for i in 1..args.len() {
        if commands.contains_key(&args[i]) {
            command = commands[&args[i]].clone();
            process_command_flags(&command, args.into_iter().skip(i).collect(), &mut flags);

            break;
        }
        match args[i].as_str() {
            // TODO: support addition args like --path
            "-C" => {
                println!("-C flag not yet supported. Continuing.");
            }
            _ => {
                panic!("Unknown argument: {}", args[i]);
            }
        }
    }

    Options { command, flags }
}
