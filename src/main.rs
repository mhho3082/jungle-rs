mod controller;
mod model;
mod view;

use std::io;

use crate::controller::{check_move, check_win, make_move};
use crate::model::Model;
use crate::view::print_board;

// The best explanation of the game:
// https://en.wikipedia.org/wiki/Jungle_(board_game)

fn main() {
    let mut model = Model::new();
    let mut input = String::new();
    let mut curr = 0;

    let mut piece: i32;
    let mut move_to: i32;

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
            if ["R", "C", "D", "W", "O", "T", "L", "E"]
                .contains(&input.as_str())
            {
                break;
            } else {
                println!("Wrong input! Please try again");
            }
        }
        piece = ["R", "C", "D", "W", "O", "T", "L", "E"]
            .iter()
            .position(|&x| x == input.as_str())
            .unwrap() as i32;

        // Get where to move to
        println!("Please enter move location.");
        loop {
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            input = input.trim().to_string();
            if let Ok(ok) = input.parse::<i32>() {
                move_to = ok;
                break;
            } else {
                println!("Wrong input! Please try again");
            }
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
