use std::{collections::HashMap, iter::Peekable, slice::Iter};

#[derive(Clone)]
pub enum CommandType {
    Add,
    Diff,
    Help,
    Init,
    Version,
    Unknown,
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum FlagOption {
    Compression,
    Name,
}

pub struct Options {
    pub command: CommandType,
    pub flags: HashMap<FlagOption, Vec<String>>,
}

fn process_command_flags(
    arg_it: &mut Peekable<Iter<String>>,
    flags: &mut HashMap<FlagOption, Vec<String>>,
) {
    while let Some(flag_option) = arg_it.next() {
        match flag_option.as_str() {
            "--compression" => {
                match arg_it.next() {
                    Some(val) => flags.insert(FlagOption::Compression, vec![val.to_string()]),
                    None => panic!("Compression flag set with no option specified."),
                };
            }
            _ => {
                match arg_it.peek() {
                    Some(_) => println!("Unknown argument: {flag_option}"),
                    None => {
                        flags.insert(FlagOption::Name, vec![flag_option.to_string()]);
                    }
                };
            }
        };
    }
}

pub fn process_args(args: &[String]) -> Options {
    let mut command = CommandType::Unknown;

    let mut flags: HashMap<FlagOption, Vec<String>> = HashMap::new();

    let mut arg_it = args.iter().peekable();

    // first argument is the gud filename, so consume the first argument and ignore
    let _ = arg_it.next();

    while let Some(arg) = arg_it.next() {
        match arg.as_str() {
            "add" => command = CommandType::Add,
            "diff" => command = CommandType::Diff,
            "init" => command = CommandType::Init,
            "--help" => command = CommandType::Help,
            "--version" => command = CommandType::Version,
            "-C" => {
                println!("-C flag not yet supported. Continuing.");
                continue;
            }
            _ => println!("Unknown argument: {arg}"),
        }

        process_command_flags(&mut arg_it, &mut flags);
    }

    Options { command, flags }
}
