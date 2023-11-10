#![allow(unused)]

use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct BaseCli {
    subcommand: String
}

fn main() {
    let args = BaseCli::parse();

    match args.subcommand.as_str() {
        "server" => {

        }
        "ïnit" => {

        }
        _ => {
            println!("oxide: unknown command '{}'", args.subcommand)
        }
    }
}
