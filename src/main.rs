mod controller;
mod model;
mod view;

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
    {
        let mut state = model.history[0];
        state.board.blue[0] -= 1;
        state.next_blue = false;
        model.history.push(state);
    }

    for x in model.history {
        print_board(&x.board, true, 1, 0);
    }
}
