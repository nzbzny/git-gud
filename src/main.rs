#[allow(clippy::module_name_repetitions)]

mod actions;
mod constants;
mod utils;

use std::env;

fn print_usage_message() {
    let mut msg = "usage: gud [--version] [--help] <command> [<args>]".to_string();

    msg += "\n\ninit: Initialize a Gud repository";

    println!("{msg}");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() == 1 {
        print_usage_message();
        return;
    }

    match args[1].as_str() {
        "--help" => {
            print_usage_message();
        }
        "--version" => {
            println!("{}", constants::VERSION);
        }
        "init" => {
            if args.len() < 3 {
                println!("usage: gud init <name>");
                return;
            }

            let ignore = utils::process_ignore_file();

            let action = actions::init::InitAction::from(ignore.clone());
            action.run(&args[2]);
        }
        _ => {
            println!("Unknown command: {}", args.join(" "));
            print_usage_message();
        }
    }
}
