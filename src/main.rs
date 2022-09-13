mod controller;
mod model;
mod view;

use crate::controller::{check_move, check_win, make_move};
use crate::model::Model;
use crate::view::print_board;

// The best explanation of the game:
// https://en.wikipedia.org/wiki/Jungle_(board_game)

fn main() {
    _test_connection();
}

fn _test_connection() {
    let mut model = Model::new();

    // Illegal move XD
    // Blue
    let moves = [
        (3, 37), // Blue
        (4, 23), // Red
    ];
    for (piece, move_to) in moves {
        println!(
            "{}",
            check_move(model.history.last().unwrap(), piece, move_to)
        );
        model.history.push(make_move(
            model.history.last().unwrap(),
            piece,
            move_to,
        ));
    }

    for x in &model.history {
        print_board(&x.board, true, 1, 0);
    }

    println!("{}", check_win(model.history.last().unwrap()));
}
