use crate::{ai::*, controller::*, model::*, Args};

use colored::Colorize;
use rand::thread_rng;
use std::io;

// The clean-screen print copied from
// https://stackoverflow.com/questions/34837011/how-to-clear-the-terminal-screen-in-rust-after-a-new-line-is-printed

/// The CLI-based user loop
pub fn cli(model: &mut Model, args: Args) {
    // The UI variables
    let border = true;
    let spaces = 1;
    let indent = 0;

    // User input
    let mut input = String::new();

    // Movement variables
    let mut piece: i32;
    let mut move_to: i32;
    let mut moves: [i32; 4];

    // Reverse the order
    if args.reverse {
        model.history[0].cur_blue = false;
    }

    // Prepare RNG for AI
    let mut rng = thread_rng();

    'main: loop {
        if model.curr().won {
            // Clean the screen
            if !args.no_clean && !args.debug {
                print!("\x1B[2J\x1B[1;1H");
            }

            // Print map
            print_board(
                &model.curr().board,
                model.current,
                border,
                spaces,
                indent,
                Vec::new(),
            );

            // Congratulate win and stop the loop
            if args.ai == AIType::None {
                println!(
                    "{} won! Congratulations!",
                    if model.curr().cur_blue {
                        "Red".red()
                    } else {
                        "Blue".blue()
                    }
                );
            } else if model.curr().cur_blue {
                println!("The AI won! Try harder next time...");
            } else {
                println!("You won! Congratulations!");
            }
            break 'main;
        } else if args.ai != AIType::None && !model.curr().cur_blue {
            // AI move

            // Debug: print all moves possible
            if args.debug {
                println!("{:?}", list_all_moves(model.curr()));
            }

            // Decide and make the move
            (piece, move_to) = args.ai.decide_move(model.curr(), &mut rng);
            if make_move(model, piece, move_to).is_err() {
                println!("An error occured with AI move");
            }

            // Debug: print move made
            if args.debug {
                println!("AI move: {:?}", (piece, move_to));
            }
        } else {
            // Human move

            // Clean the screen
            if !args.no_clean && !args.debug {
                print!("\x1B[2J\x1B[1;1H");
            }

            // Print the map
            print_board(
                &model.curr().board,
                model.current,
                border,
                spaces,
                indent,
                Vec::new(),
            );

            // Get which piece to move
            print!(
                "It's {} turn! Please enter the piece name",
                if args.ai != AIType::None {
                    "your".normal()
                } else if model.curr().cur_blue {
                    "blue's".blue()
                } else {
                    "red's".red()
                }
            );
            if args.time_machine {
                print!(", or 'n'/'p' for time travel (next/prev)");
            }
            println!(", or 'q' for quit.");

            // Debug: print all moves possible
            if args.debug {
                println!("{:?}", list_all_moves(model.curr()));
            }

            // The input loop for which piece to move
            'input: loop {
                input.clear();
                io::stdin().read_line(&mut input).unwrap();
                input = input.trim().to_string();

                // Check if the piece input is correct
                if let Some(index) = accept_piece(&input) {
                    // Check if quitting
                    if index == -1 {
                        println!("Goodbye!");
                        break 'main;
                    }
                    // Check if using time machine
                    if [8, 9].contains(&index) {
                        if args.time_machine {
                            let mut temp =
                                if args.ai != AIType::None { 2 } else { 1 };
                            temp *= if index == 8 { 1 } else { -1 };
                            match make_travel(model, temp) {
                                Ok(_) => {
                                    continue 'main;
                                }
                                Err(_) => {
                                    println!("Already at {} of history! Please try again.", if index == 8 { "end" } else {"beginning"});
                                    continue 'input;
                                }
                            }
                        } else {
                            println!(
                                "Time machine not enabled! Please try again."
                            );
                            continue 'input;
                        }
                    }

                    piece = index;

                    // Check if there are any moves for this piece
                    moves = list_piece_moves(model.curr(), piece);
                    if moves.iter().all(|&x| x > 62) {
                        println!("No moves possible! Please try again.");
                        continue 'input;
                    } else {
                        break 'input;
                    }
                } else {
                    println!("Wrong input! Please try again.");
                }
            }

            // Clean the screen
            if !args.no_clean && !args.debug {
                print!("\x1B[2J\x1B[1;1H");
            }

            print_board(
                &model.curr().board,
                model.current,
                border,
                spaces,
                indent,
                moves.iter().filter(|&&x| x < 63).collect(),
            );

            // List the moves
            if args.debug {
                print!("Moves: ");
                for x in moves {
                    if x < 63 {
                        print!("{} ", x);
                    }
                }
                println!();
            }

            // Get where to move to
            println!("Please enter move direction ([wasd] or [hjkl], or 'c'/'n' for cancel).");
            if args.debug {
                println!("You can also enter the move index.");
            }

            'input: loop {
                input.clear();
                io::stdin().read_line(&mut input).unwrap();
                input = input.trim().to_string();
                if let Ok(ok) = input.parse::<i32>() {
                    if !args.debug {
                        println!("Debug mode not enabled! Please try again.");
                        continue 'input;
                    }
                    move_to = ok;

                    match make_move(model, piece, move_to) {
                        Ok(_) => {
                            println!("Move successful!");
                            continue 'main;
                        }
                        Err(_) => {
                            println!("Move illegal! Please try again.");
                            continue 'input;
                        }
                    }
                } else if let Some(dir) = accept_arrow(&input) {
                    // Cancel
                    if dir == 4 {
                        continue 'main;
                    }

                    move_to = moves[dir];

                    match make_move(model, piece, move_to) {
                        Ok(_) => {
                            println!("Move successful!");
                            continue 'main;
                        }
                        Err(_) => {
                            println!("Move illegal! Please try again.");
                            continue 'input;
                        }
                    }
                } else {
                    println!("Wrong input! Please try again.");
                    continue 'input;
                }
            }
        }
    }
}

/// Accepts piece inputs insensitively
/// 1-7: piece, 8: next, 9: prev, -1: quit
fn accept_piece(input: &str) -> Option<i32> {
    let inp = input.to_ascii_lowercase();

    if let Some(index) = ["r", "c", "d", "w", "o", "t", "l", "e"]
        .iter()
        .position(|&x| x == inp)
    {
        Some(index as i32)
    } else if let Some(index) = [
        "rat", "cat", "dog", "wolf", "leopard", "tiger", "lion", "elephant",
    ]
    .iter()
    .position(|&x| x == inp)
    {
        Some(index as i32)
    } else if let Some(index) = ["n", "p"].iter().position(|&x| x == inp) {
        Some((index + 8) as i32)
    } else if let Some(index) = ["next", "prev"].iter().position(|&x| x == inp)
    {
        Some((index + 8) as i32)
    } else if ["q", "quit"].contains(&inp.as_str()) {
        Some(-1)
    } else {
        None
    }
}

/// Accepts direction inputs case-insensitively
/// Accepts `c` and `n` as cancel (return 4)
fn accept_arrow(input: &str) -> Option<usize> {
    let inp = input.to_ascii_lowercase();

    if let Some(dir) = ["h", "j", "k", "l"].iter().position(|&x| x == inp) {
        Some(dir)
    } else if let Some(dir) =
        ["a", "s", "w", "d"].iter().position(|&x| x == inp)
    {
        Some(dir)
    } else if ["c", "n", "cancel"].contains(&inp.as_str()) {
        Some(4)
    } else {
        None
    }
}

/// Prints the board
/// Formats the pieces as R C D W O T L E
pub fn print_board(
    board: &Board,
    step: usize,
    border: bool,
    space: i32,
    indent: i32,
    highlight: Vec<&i32>,
) {
    let pieces = ["R", "C", "D", "W", "O", "T", "L", "E"];

    let mut was_river = false;

    println!(
        "{}{}Step {}",
        " ".repeat((indent + (space * 6 + 7) / 2 - 3) as usize),
        if border { " " } else { "" },
        step + 1
    );

    // Print and add borders
    if border {
        print!(
            "{}+-------{}+ ",
            " ".repeat(indent as usize),
            "-".repeat((space * 6) as usize)
        );
        for (i, e) in board.red.iter().enumerate() {
            if e > &62 {
                print!("{} ", pieces[i].red());
            }
        }
        println!();
    }

    // Print for every square
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
        print!(
            "{}+-------{}+ ",
            " ".repeat(indent as usize),
            "-".repeat((space * 6) as usize)
        );
        for (i, e) in board.blue.iter().enumerate() {
            if e > &62 {
                print!("{} ", pieces[i].blue());
            }
        }
        println!();
    }
}
