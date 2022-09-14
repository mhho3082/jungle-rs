mod ai;
mod controller;
mod model;
mod view;

use crate::{model::Model, view::cli};
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Whether to run in debug mode
    #[clap(short, long, value_parser, default_value_t = false)]
    debug: bool,

    /// Whether to let red play first
    #[clap(short, long, value_parser, default_value_t = false)]
    reverse: bool,

    /// Whether player red is an AI
    #[clap(short, long, value_parser, default_value_t = false)]
    ai: bool,

    /// Whether not to clean the screen after each input
    /// Always off in debug mode
    #[clap(short, long, value_parser, default_value_t = false)]
    no_clean: bool,
}

// The best explanation of the game:
// https://en.wikipedia.org/wiki/Jungle_(board_game)

fn main() {
    let args = Args::parse();

    let mut model = Model::new();
    cli(&mut model, args.ai, args.reverse, args.debug, args.no_clean);
}
