mod controller;
mod model;
mod view;

use std::io;

use crate::controller::{check_move, check_win, list_piece_moves, make_move};
use crate::model::Model;
use crate::view::print_board;

// The best explanation of the game:
// https://en.wikipedia.org/wiki/Jungle_(board_game)

fn main() {
    let mut model = Model::new();
    let mut input = String::new();
    let mut curr = 0;

    let mut piece: i32;
    let mut move_to: i32 = 0;
    let mut moves: [i32; 4];

    loop {
        // Print map
        print_board(&model.history[curr].board, true, 1, 0);

        // Get which piece to move
        println!(
            "{}'s turn! Please enter piece name.",
            if model.history[curr].cur_blue {
                "Blue"
            } else {
                "Red"
            }
        );
        loop {
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            input = input.trim().to_string();
            if ["r", "c", "d", "w", "o", "t", "l", "e"]
                .contains(&input.as_str())
            {
                break;
            } else {
                println!("Wrong input! Please try again");
            }
        }
        piece = ["r", "c", "d", "w", "o", "t", "l", "e"]
            .iter()
            .position(|&x| x == input.as_str())
            .unwrap() as i32;

        // TEMP: List possible moves
        moves = list_piece_moves(&model.history[curr], piece);

        println!("{moves:?}");
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
                break;
            } else {
                println!("Wrong input! Please try again");
            }
        }
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

        if check_move(&model.history[curr], piece, move_to) {
            model
                .history
                .push(make_move(&model.history[curr], piece, move_to));
            curr += 1;
            println!("Move successful!");
        } else {
            println!("Move illegal! Please try again.");
        }
    }
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
