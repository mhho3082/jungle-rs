mod controller;
mod model;
mod view;

use controller::check_win;

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
    {
        let mut state = *model.history.last().unwrap();
        state.board.blue[3] = 37;
        state.next_blue = false;
        model.history.push(state);
    }
    {
        let mut state = *model.history.last().unwrap();
        state.board.red[4] = 23;
        state.next_blue = true;
        model.history.push(state);
    }

    for x in &model.history {
        print_board(&x.board, true, 1, 0);
    }

    println!("{}", check_win(model.history.last().unwrap()));
}
