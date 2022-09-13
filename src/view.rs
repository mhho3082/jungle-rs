use crate::controller::*;
use crate::model::*;
use std::io;

use colored::Colorize;

/// The main user loop
pub fn user_loop(model: &mut Model) {
    let mut input = String::new();

    let mut piece: i32;
    let mut move_to: i32 = 0;
    let mut moves: [i32; 4];

    loop {
        // Print map
        print_board(&model.curr().board, true, 1, 0, Vec::new());

        // Get which piece to move
        println!(
            "{}'s turn! Please enter piece name.",
            if model.curr().cur_blue { "Blue" } else { "Red" }
        );

        // TEMP
        println!("{:?}", list_all_moves(model.curr()));

        loop {
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            input = input.trim().to_string();
            if ["r", "c", "d", "w", "o", "t", "l", "e"]
                .contains(&input.as_str())
            {
                piece = ["r", "c", "d", "w", "o", "t", "l", "e"]
                    .iter()
                    .position(|&x| x == input.as_str())
                    .unwrap() as i32;
                break;
            } else {
                println!("Wrong input! Please try again");
            }
        }

        moves = list_piece_moves(model.curr(), piece);
        if moves.iter().all(|&x| x > 62) {
            println!("No moves possible! Please try again.");
            continue;
        }

        print_board(
            &model.curr().board,
            true,
            1,
            0,
            moves.iter().filter(|&&x| x < 63).collect(),
        );

        print!("Legal move directions: ");
        for (i, x) in moves.iter().enumerate() {
            if x < &63 {
                match i {
                    0 => print!("H"),
                    1 => print!("J"),
                    2 => print!("K"),
                    3 => print!("L"),
                    _ => print!("Error"),
                }
            } else {
                print!(" ");
            }
        }
        println!();

        // Get where to move to
        println!("Please enter move direction.");
        loop {
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            input = input.trim().to_string();
            if ["w", "a", "s", "d", "h", "j", "k", "l"]
                .contains(&input.as_str())
            {
                if let Some(dir) = ["h", "j", "k", "l"]
                    .iter()
                    .position(|&x| x == input.as_str())
                {
                    move_to = moves[dir];
                } else if let Some(dir) = ["a", "s", "w", "d"]
                    .iter()
                    .position(|&x| x == input.as_str())
                {
                    move_to = moves[dir];
                }

                if check_move(model.curr(), piece, move_to) {
                    make_move(model, piece, move_to);
                    println!("Move successful!");
                    break;
                } else {
                    println!("Move illegal! Please try again.");
                }
            } else {
                println!("Wrong input! Please try again");
            }
        }
    }
}

/// Prints the board
/// Formats the pieces as R C D W O T L E
pub fn print_board(
    board: &Board,
    border: bool,
    space: i32,
    indent: i32,
    highlight: Vec<&i32>,
) {
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
        let mut out;
        if board.blue.contains(&i) {
            let index = board.blue.iter().position(|&x| x == i).unwrap();
            out = pieces[index].blue();

            if RIVERS.contains(&i) {
                out = out.reversed();
                was_river = !was_river;
            } else if TRAPS_BLUE.contains(&i)
                || TRAPS_RED.contains(&i)
                || i == DEN_BLUE
                || i == DEN_RED
            {
                out = out.reversed();
            }
        } else if board.red.contains(&i) {
            let index = board.red.iter().position(|&x| x == i).unwrap();
            out = pieces[index].red();

            if RIVERS.contains(&i) {
                out = out.reversed();
                was_river = !was_river;
            } else if TRAPS_BLUE.contains(&i)
                || TRAPS_RED.contains(&i)
                || i == DEN_BLUE
                || i == DEN_RED
            {
                out = out.reversed();
            }
        } else if RIVERS.contains(&i) {
            out = "~".on_green();
            was_river = !was_river;
        } else if TRAPS_BLUE.contains(&i) {
            out = "#".on_cyan();
        } else if TRAPS_RED.contains(&i) {
            out = "#".on_magenta();
        } else if i == DEN_BLUE || i == DEN_RED {
            out = "@".reversed();
        } else {
            out = ".".normal();
        }

        if highlight.contains(&&i) {
            out = out.on_yellow();
        }

        print!("{}", out);

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
