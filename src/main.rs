mod controller;
mod model;
mod view;

use crate::model::Model;
use crate::view::cli;

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
}

// The best explanation of the game:
// https://en.wikipedia.org/wiki/Jungle_(board_game)

fn main() {
    let args = Args::parse();

    let mut model = Model::new();
    cli(&mut model, args.reverse, args.debug);
}
