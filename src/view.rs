use crate::model::{
    Board, COL_COUNT, DEN_BLUE, DEN_RED, RIVERS, TRAPS_BLUE, TRAPS_RED,
};

use colored::Colorize;

/// Prints the board
/// Formats the pieces as R C D W O T L E
pub fn print_board(board: &Board, border: bool, space: i32, indent: i32) {
    let pieces = ["R", "C", "D", "W", "O", "T", "L", "E"];

    let mut was_river = false;

    // Print and add borders
    if border {
        println!(
            "{}+-------{}+",
            " ".repeat(indent as usize),
            "-".repeat((space * 6) as usize)
        );
    }
    for i in 0..63 {
        if i % COL_COUNT == 0 {
            print!("{}", " ".repeat(indent as usize));
            if border {
                print!("|");
            }
        }

        // Find and print correct square type
        if board.blue.contains(&i) {
            let index = board.blue.iter().position(|&x| x == i).unwrap();
            let piece = pieces[index].blue();

            if RIVERS.contains(&i) {
                print!("{}", piece.reversed());
                was_river = !was_river;
            } else if TRAPS_BLUE.contains(&i)
                || TRAPS_RED.contains(&i)
                || i == DEN_BLUE
                || i == DEN_RED
            {
                print!("{}", piece.reversed());
            } else {
                print!("{}", piece);
            }
        } else if board.red.contains(&i) {
            let index = board.red.iter().position(|&x| x == i).unwrap();
            let piece = pieces[index].red();

            if RIVERS.contains(&i) {
                print!("{}", piece.reversed());
                was_river = !was_river;
            } else if TRAPS_BLUE.contains(&i)
                || TRAPS_RED.contains(&i)
                || i == DEN_BLUE
                || i == DEN_RED
            {
                print!("{}", piece.reversed());
            } else {
                print!("{}", piece);
            }
        } else if RIVERS.contains(&i) {
            print!("{}", "~".on_green());
            was_river = !was_river;
        } else if TRAPS_BLUE.contains(&i) {
            print!("{}", "#".on_cyan());
        } else if TRAPS_RED.contains(&i) {
            print!("{}", "#".on_magenta());
        } else if i == DEN_BLUE || i == DEN_RED {
            print!("{}", "@".reversed());
        } else {
            print!(".");
        }

        if i % COL_COUNT == COL_COUNT - 1 {
            if border {
                print!("|");
            }
            println!();
        } else if was_river {
            print!("{}", " ".repeat(space as usize).on_green());
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
