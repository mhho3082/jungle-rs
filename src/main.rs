mod controller;
mod model;
mod view;

use crate::controller::{check_win, make_move};
use crate::model::Model;
use crate::view::print_board;

// The best explanation of the game:
// https://en.wikipedia.org/wiki/Jungle_(board_game)

// To print colors:
// https://stackoverflow.com/questions/69981449/how-do-i-print-colored-text-to-the-terminal-in-rust

fn main() {
    _test_connection();
}

fn _test_connection() {
    let mut model = Model::new();

    // Should be in controller
    // btw illegal move XD
    model
        .history
        .push(make_move(model.history.last().unwrap(), 3, 37));
    model
        .history
        .push(make_move(model.history.last().unwrap(), 4, 23));

    for x in &model.history {
        print_board(&x.board, true, 1, 0);
    }

    println!("{}", check_win(model.history.last().unwrap()));
}
