// Connect other source files to the application
mod ai;
mod controller;
mod model;
mod view;

// Imports (or includes)
// `crate` refers to the whole application
use crate::{ai::AIType, model::Model, view::cli};
use clap::Parser;

// Explanation of the game:
// https://en.wikipedia.org/wiki/Jungle_(board_game)

// Just a simple list of arguments for the `clap` library
// `derive` and the `#[...]` are just macros
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Whether to run in debug mode
    #[clap(short, long, value_parser, default_value_t = false)]
    debug: bool,

    /// Whether to let red play first
    #[clap(short, long, value_parser, default_value_t = false)]
    reverse: bool,

    /// Use AI with the picked algorithm for red
    #[clap(short, long, value_enum, default_value_t = AIType::None)]
    ai: AIType,

    /// Whether not to clean the screen after each input
    /// (always off in debug mode)
    #[clap(short, long, value_parser, default_value_t = false)]
    no_clean: bool,

    /// Whether to use time machine
    #[clap(short, long, value_parser, default_value_t = false)]
    time_machine: bool,
}

// The entry point
fn main() {
    // Parse the arguments
    let args = Args::parse();

    // Launch the user loop
    let mut model = Model::new();
    cli(&mut model, args);
}
