mod controller;
mod model;
mod view;

use crate::model::Model;
use crate::view::{debug_2p, user_2p};

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Whether to run in debug mode
    #[clap(short, long, value_parser, default_value_t = false)]
    debug: bool,
}

// The best explanation of the game:
// https://en.wikipedia.org/wiki/Jungle_(board_game)

fn main() {
    let args = Args::parse();

    let mut model = Model::new();
    if args.debug {
        debug_2p(&mut model);
    } else {
        user_2p(&mut model);
    }
}
