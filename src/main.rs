mod controller;
mod model;

use model::{Board, COL_COUNT, DENS, RIVERS, TRAPS_BLUE, TRAPS_RED};

use crate::model::Model;
use colored::Colorize;

// The best explanation of the game:
// https://en.wikipedia.org/wiki/Jungle_(board_game)

// To print colors:
// https://stackoverflow.com/questions/69981449/how-do-i-print-colored-text-to-the-terminal-in-rust

fn main() {
    _test_connection();
}

/// A temporary function for output
fn _print_board(board: &Board) {
    // a lot of `i as usize` here...
    // since indexing only accepts usize
    // but everything else is i32
    let pieces = ["R", "C", "D", "W", "O", "T", "L", "E"];

    // Print and add borders
    println!("+-------------+");
    for i in 0..63 {
        if i % COL_COUNT == 0 {
            print!("|");
        }

        if board.blue.contains(&i) {
            let index = board.blue.iter().position(|&x| x == i).unwrap();
            print!("{}", pieces[index].blue());
        } else if board.red.contains(&i) {
            let index = board.red.iter().position(|&x| x == i).unwrap();
            print!("{}", pieces[index].red());
        } else if RIVERS.contains(&i) {
            print!("{}", "~".on_bright_blue());
        } else if TRAPS_BLUE.contains(&i) {
            print!("{}", "#".on_cyan());
        } else if TRAPS_RED.contains(&i) {
            print!("{}", "#".on_magenta());
        } else if DENS.contains(&i) {
            print!("{}", "@".reversed());
        } else {
            print!(".");
        }

        if i % COL_COUNT == COL_COUNT - 1 {
            println!("|");
        } else {
            print!(" ");
        }
    }
    println!("+-------------+");
}

fn _test_connection() {
    let mut model = Model::new();
    {
        let mut state = model.history[0];
        state.board.blue[0] -= 1;
        state.next_blue = false;
        model.history.push(state);
    }
    for x in model.history {
        _print_board(&x.board);
    }
}
