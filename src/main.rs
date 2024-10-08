#[allow(clippy::module_name_repetitions)]
mod actions;
mod command_line_processor;
mod constants;
mod utils;

use std::env;

use actions::action::Action;

fn print_usage_message() {
    let mut msg = "usage: gud [[--version] [--help] <command> <args>]".to_string();

    msg += "\n\ninit: Initialize a Gud repository";

    println!("{msg}");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        print_usage_message();
        return;
    }

    let options = command_line_processor::process_args(&args);

    match options.command {
        command_line_processor::CommandType::Diff => {
            if args.len() < 3 {
                println!("usage: gud init [flags] <name>");
                return;
            }

            let action = actions::diff::DiffAction::new(options);
            action.run();
        }
        command_line_processor::CommandType::Init => {
            if args.len() < 3 {
                println!("usage: gud init [flags] <name>");
                return;
            }

            let ignore = utils::process_ignore_file();

            let action = actions::init::InitAction::new(ignore, options.flags);
            action.run();
        }
        command_line_processor::CommandType::Help => {
            print_usage_message();
        }
        command_line_processor::CommandType::Version => {
            println!("{}", constants::VERSION);
        }
        _ => {
            println!("Unknown command: {}", args.join(" "));
            print_usage_message();
        }
    }
}
