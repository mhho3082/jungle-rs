use crate::model::{Board, COL_COUNT, DENS, RIVERS, TRAPS_BLUE, TRAPS_RED};

use colored::Colorize;

/// Prints the board
/// Formats the pieces as R C D W O T L E
pub fn print_board(board: &Board, border: bool, space: i32, indent: i32) {
    let pieces = ["R", "C", "D", "W", "O", "T", "L", "E"];

    // Print and add borders
    if border {
        println!(
            "{}+-------{}+",
            " ".repeat(indent as usize),
            "-".repeat((space * 6) as usize)
        );
    }
    for i in 0..63 {
        if border && i % COL_COUNT == 0 {
            print!("{}|", " ".repeat(indent as usize));
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
            if border {
                print!("|");
            }
            println!();
        } else {
            print!("{}", " ".repeat(space as usize));
        }
    }
    if border {
        println!(
            "{}+-------{}+",
            " ".repeat(indent as usize),
            "-".repeat((space * 6) as usize)
        );
    }
}
