use std::{borrow::BorrowMut, collections::{HashMap, HashSet}};

#[derive(Clone)]
enum CommandType {
    ADD,
    INIT,
    UNKNOWN,
}

#[derive(PartialEq, Eq, Hash)]
enum ArgumentType {
    HELP,
    VERSION,
}

pub struct ArgsProcessor {
    command: CommandType,
    args: HashMap<ArgumentType, Vec<String>>,
}

fn command_strs() -> HashMap<String, CommandType> {
    // TODO: lazily evaluate this globally somewhere
    vec![("init".to_string(), CommandType::INIT), ("add".to_string(), CommandType::ADD)]
        .into_iter()
        .collect()
}

pub fn process_args(args: Vec<String>) -> ArgsProcessor {
    let mut command = CommandType::UNKNOWN;

    let mut args_map: HashMap<ArgumentType, Vec<String>> = HashMap::new();
    let commands = command_strs();
    let mut command_idx: usize = 0;
    for i in 1..args.len() {
        if commands.contains_key(&args[i]) {
            // process command specific args
            command = commands[&args[i]].clone();
            command_idx = i;
            break;
        }
        match args[i].as_str() {
            "--version" => {
                args_map.insert(ArgumentType::VERSION, vec![]);
            }
            "--help" => {
                args_map.insert(ArgumentType::HELP, vec![]);
            }
            _ => {
                panic!("Unknown argument: {}", args[i]);
            }
        }
    }

    ArgsProcessor {
        command,
        args: args_map,
    }
}
