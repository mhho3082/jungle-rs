use crate::{ai::*, controller::*, model::*};

use colored::Colorize;
use rand::thread_rng;
use std::io;

// The clean-screen print copied from
// https://stackoverflow.com/questions/34837011/how-to-clear-the-terminal-screen-in-rust-after-a-new-line-is-printed

/// The CLI-based user loop
pub fn cli(
    model: &mut Model,
    ai: AIType,
    reverse: bool,
    debug: bool,
    no_clean: bool,
) {
    let mut input = String::new();

    let mut piece: i32;
    let mut move_to: i32;
    let mut moves: [i32; 4];

    // Reverse the order
    if reverse {
        model.history[0].cur_blue = false;
    }

    let mut rng = thread_rng();

    'main: loop {
        if model.curr().won {
            // Clean the screen
            if !no_clean && !debug {
                print!("\x1B[2J\x1B[1;1H");
            }

            // Print map
            print_board(
                &model.curr().board,
                model.current,
                true,
                1,
                0,
                Vec::new(),
            );

            // Congratulate win and stop the loop
            println!(
                "{} won! Congratulations!",
                if model.curr().cur_blue {
                    "Red".red()
                } else {
                    "Blue".blue()
                }
            );
            break 'main;
        } else if ai != AIType::Null && !model.curr().cur_blue {
            // AI move

            // Debug: print all moves possible
            if debug {
                println!("{:?}", list_all_moves(model.curr()));
            }

            // Pick algorithm
            (piece, move_to) = match ai {
                AIType::Random | AIType::Null => {
                    ai_random(model.curr(), &mut rng)
                }
                AIType::NaiveDefensive => {
                    ai_naive_defensive(model.curr(), &mut rng)
                }
                AIType::NaiveNeutral => {
                    ai_naive_neutral(model.curr(), &mut rng)
                }
                AIType::NaiveAggressive => {
                    ai_naive_aggressive(model.curr(), &mut rng)
                }
            };

            make_move(model, piece, move_to);

            // Debug: print move made
            if debug {
                println!("AI move: {:?}", (piece, move_to));
            }
        } else {
            // Human move

            // Clean the screen
            if !no_clean && !debug {
                print!("\x1B[2J\x1B[1;1H");
            }

            // Print map
            print_board(
                &model.curr().board,
                model.current,
                true,
                3,
                0,
                Vec::new(),
            );

            // Get which piece to move
            println!(
                "It's {}'s turn! Please enter the piece name.",
                if model.curr().cur_blue {
                    "blue".blue()
                } else {
                    "red".red()
                }
            );

            // Debug: print all moves possible
            if debug {
                println!("{:?}", list_all_moves(model.curr()));
            }

            'input: loop {
                input.clear();
                io::stdin().read_line(&mut input).unwrap();
                input = input.trim().to_string();
                if let Some(index) = accept_piece(&input) {
                    piece = index;
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

            if debug {
                // Manual input of square index
                moves = list_piece_moves(model.curr(), piece);

                print_board(
                    &model.curr().board,
                    model.current,
                    true,
                    1,
                    0,
                    moves.iter().filter(|&&x| x < 63).collect(),
                );

                print!("Moves: ");
                for x in moves {
                    if x < 63 {
                        print!("{} ", x);
                    }
                }
                println!();

                // Get where to move to
                println!("Please enter move index.");
                loop {
                    input.clear();
                    io::stdin().read_line(&mut input).unwrap();
                    input = input.trim().to_string();
                    if let Ok(ok) = input.parse::<i32>() {
                        move_to = ok;

                        if check_move(model.curr(), piece, move_to) {
                            make_move(model, piece, move_to);
                            println!("Move successful!");
                            continue 'main;
                        } else {
                            println!("Move illegal! Please try again.");
                        }
                    } else {
                        println!("Wrong input! Please try again.");
                    }
                }
            } else {
                // Automatic arrow-key indexing

                // Clean the screen
                if !no_clean {
                    print!("\x1B[2J\x1B[1;1H");
                }

                print_board(
                    &model.curr().board,
                    model.current,
                    true,
                    1,
                    0,
                    moves.iter().filter(|&&x| x < 63).collect(),
                );

                // Get where to move to
                println!("Please enter move direction ([wasd] or [hjkl], or [cn] cancel).");
                loop {
                    input.clear();
                    io::stdin().read_line(&mut input).unwrap();
                    input = input.trim().to_string();
                    if let Some(dir) = accept_arrow(&input) {
                        // Cancel
                        if dir == 4 {
                            break;
                        }

                        move_to = moves[dir];

                        if check_move(model.curr(), piece, move_to) {
                            make_move(model, piece, move_to);
                            println!("Move successful!");
                            continue 'main;
                        } else {
                            println!("Move illegal! Please try again.");
                        }
                    } else {
                        println!("Wrong input! Please try again.");
                    }
                }
            }
        }
    }
}

/// Accepts piece inputs insensitively
fn accept_piece(input: &str) -> Option<i32> {
    if !["r", "c", "d", "w", "o", "t", "l", "e"]
        .contains(&input.to_lowercase().as_str())
        && ![
            "rat", "cat", "dog", "wolf", "leopard", "tiger", "lion", "elephant",
        ]
        .contains(&input.to_lowercase().as_str())
    {
        return None;
    }

    if let Some(index) = ["r", "c", "d", "w", "o", "t", "l", "e"]
        .iter()
        .position(|&x| x == input.to_lowercase().as_str())
    {
        Some(index as i32)
    } else {
        [
            "rat", "cat", "dog", "wolf", "leopard", "tiger", "lion", "elephant",
        ]
        .iter()
        .position(|&x| x == input.to_lowercase().as_str())
        .map(|x| x as i32)
    }
}

/// Accepts direction inputs case-insensitively
/// Accepts `c` and `n` as cancel
fn accept_arrow(input: &str) -> Option<usize> {
    if !["w", "a", "s", "d", "h", "j", "k", "l", "c", "n"]
        .contains(&input.to_lowercase().as_str())
    {
        return None;
    }

    if let Some(dir) = ["h", "j", "k", "l"]
        .iter()
        .position(|&x| x == input.to_lowercase().as_str())
    {
        Some(dir)
    } else if let Some(dir) = ["a", "s", "w", "d"]
        .iter()
        .position(|&x| x == input.to_lowercase().as_str())
    {
        Some(dir)
    } else if ["c", "n"].contains(&input.to_lowercase().as_str()) {
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
        "{}    Step {}",
        " ".repeat((indent + space * 2) as usize),
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
