use crate::model::{Board, COL_COUNT, DENS, RIVERS, TRAPS_BLUE, TRAPS_RED};

use colored::Colorize;

pub fn print_board(board: &Board) {
    let pieces = ["R", "C", "D", "W", "O", "T", "L", "E"];

    // Print and add borders
    println!("+-------------+");
    for i in 0..63 {
        if i % COL_COUNT == 0 {
            print!("|");
        }

        // Find and print correct square type
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
