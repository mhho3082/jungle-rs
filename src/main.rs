mod controller;
mod model;
mod view;

use crate::model::Model;
use crate::view::user_loop;

// The best explanation of the game:
// https://en.wikipedia.org/wiki/Jungle_(board_game)

fn main() {
    let mut model = Model::new();
    user_loop(&mut model);
}
