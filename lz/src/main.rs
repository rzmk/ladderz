//! # lz
//! 
//! A command-line interface for various math/tech subjects. Based on the [ladderz](https://github.com/rzmk/ladderz) library.
//! 
//! # Installation
//! 
//! To install the command-line interface, run the following command in your terminal:
//! 
//! ```bash
//! cargo install --git https://github.com/rzmk/ladderz --branch main
//! ```
//! 
//! # Example
//! 
//! ```bash
//! lz prealgebra is-factor 3 12
//! ```
//! 
//! ```console
//! 3 is a factor of 12.
//! ```
//! 
//! You may view the help menu for a subject and a function by running the command with the `-h` or `--help` flag:
//! 
//! ```bash
//! lz prealgebra is-factor -h
//! ```
//! 
//! Learn more on [GitHub](https://github.com/rzmk/ladderz).

// External modules
use clap::{Parser, Subcommand};

// Local modules
pub mod prealgebra;
use prealgebra::{match_prealgebra, Prealgebra};

#[derive(Parser)]
#[command(
    author = "Mueez Khan",
    about = "Run various functions from a range of math/tech subjects on the command line.",
    subcommand_value_name = "SUBJECT",
    arg_required_else_help(true)
)]
struct Cli {
    #[command(subcommand)]
    subject: Option<Subjects>,
}

/// The subjects that can be used.
#[derive(Subcommand)]
#[command(arg_required_else_help(true))]
enum Subjects {
    Prealgebra {
        /// The function (command) to run.
        #[command(subcommand)]
        function: Option<Prealgebra>,
    },
}

fn main() {
    let cli: Cli = Cli::parse();

    // Match the subject to run the correct function.
    match cli.subject {
        Some(Subjects::Prealgebra { function }) => match_prealgebra(function),
        None => {
            println!("Please provide a subject to use.");
        }
    }
}
