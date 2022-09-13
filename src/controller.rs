// My thought is that these would mainly be
// modifications on the model

// Move: (index, moveTo)

// TODO:
// Time machine:
// state_count
// goto_state

use crate::model::*;

/// Gives possible move locations in [hjkl] (or <V^>)
pub fn list_piece_moves(state: &State, piece: i32) -> [i32; 4] {
    let original = if state.cur_blue {
        state.board.blue[piece as usize]
    } else {
        state.board.red[piece as usize]
    };

    let mut out = [63; 4];

    // If dead already
    if original > 62 {
        return out;
    }

    // Find one-step moves
    for (i, e) in [-1, 7, -7, 1].iter().enumerate() {
        if check_move(state, piece, original + e) {
            out[i] = original + e;
        }
    }

    // Find river moves
    if [5, 6].contains(&piece) {
        for (i, e) in [-1, 7, -7, 1].iter().enumerate() {
            if RIVERS.contains(&(original + e)) {
                for x in RIVER_MOVES {
                    if x.contains(&original) {
                        let temp = x.iter().filter(|y| y != &&original).sum();
                        if (e < &0 && temp < original)
                            || (e > &0 && temp > original)
                        {
                            out[i] = temp;
                        }
                    }
                }
            }
        }
    }

    out
}

/// Gives all possible moves for AI
pub fn list_all_moves(state: &State) -> Vec<(i32, i32)> {
    let mut out = Vec::new();

    for piece in 0..8 {
        for move_to in list_piece_moves(state, piece) {
            if move_to < 63 {
                out.push((piece, move_to));
            }
        }
    }

    out
}

pub fn check_move(state: &State, piece: i32, move_to: i32) -> bool {
    check_walk(state, piece, move_to) && check_capture(state, piece, move_to)
}

/// Checks if the movement is legal
/// Ignores captures (or blockages)
pub fn check_walk(state: &State, piece: i32, move_to: i32) -> bool {
    let original = if state.cur_blue {
        state.board.blue[piece as usize]
    } else {
        state.board.red[piece as usize]
    };

    // Checks if out-of-bounds
    if !(0..=62).contains(&move_to) {
        return false;
    }

    // Checks if loop-around
    if (original % COL_COUNT == 0 && (move_to + 1) % COL_COUNT == 0)
        || (move_to % COL_COUNT == 0 && (original + 1) % COL_COUNT == 0)
    {
        return false;
    }

    // Checks if moving into one's own den
    if state.cur_blue {
        if move_to == DEN_BLUE {
            return false;
        }
    } else if move_to == DEN_RED {
        return false;
    }

    // Checks if cross-border (excpet for mouse)
    if piece != 0
        && ((RIVERS.contains(&move_to) && !RIVERS.contains(&original))
            || (RIVERS.contains(&original) && !RIVERS.contains(&move_to)))
    {
        return false;
    }

    // Checks if neither 1-square nor river
    let mut river = false;
    if !([1, 7].contains(&(original - move_to).abs())) {
        if RIVER_MOVES.contains(&[original, move_to])
            || RIVER_MOVES.contains(&[move_to, original])
        {
            river = true;
        } else {
            return false;
        }
    }

    // Checks if legal river
    if river {
        // Only lion and tiger can jump
        if ![5, 6].contains(&piece) {
            return false;
        }

        // Check if rat in (intervening) river
        if let Some(leap) =
            RIVER_MOVES.iter().position(|&x| x == [original, move_to])
        {
            if RIVER_LEAPS[leap].contains(&state.board.blue[0])
                || RIVER_LEAPS[leap].contains(&state.board.red[0])
            {
                return false;
            }
        } else if let Some(leap) =
            RIVER_MOVES.iter().position(|&x| x == [original, move_to])
        {
            if RIVER_LEAPS[leap].contains(&state.board.blue[0])
                || RIVER_LEAPS[leap].contains(&state.board.red[0])
            {
                return false;
            }
        }
    }

    true
}

/// Checks if capture, if any, is legal (or crash into ally)
pub fn check_capture(state: &State, piece: i32, move_to: i32) -> bool {
    // Check if there is anything to capture
    // or crash into ally
    if state.cur_blue {
        if state.board.blue.contains(&move_to) {
            return false;
        } else if !state.board.red.contains(&move_to) {
            return true;
        }
    } else if state.board.red.contains(&move_to) {
        return false;
    } else if !state.board.blue.contains(&move_to) {
        return true;
    }

    // Get enemy piece
    let enemy = if state.cur_blue {
        state.board.red.iter().position(|&x| x == move_to).unwrap() as i32
    } else {
        state.board.blue.iter().position(|&x| x == move_to).unwrap() as i32
    };

    // Check if cross-border
    if (RIVERS.contains(&move_to) && !RIVERS.contains(&enemy))
        || (RIVERS.contains(&enemy) && !RIVERS.contains(&move_to))
    {
        return false;
    }

    // Check capture
    if piece >= enemy || piece == 0 && enemy == 7 {
        true
    } else if state.cur_blue {
        TRAPS_BLUE.contains(&enemy)
    } else {
        TRAPS_RED.contains(&enemy)
    }
}

/// Assumes that the move is legal already
/// Returns a new state where the move is made
pub fn make_move(model: &mut Model, piece: i32, move_to: i32) {
    let mut state = *model.curr();

    // Move piece and make capture if needed
    if state.cur_blue {
        state.board.blue[piece as usize] = move_to;
        if state.board.red.contains(&move_to) {
            let enemy =
                state.board.red.iter().position(|&x| x == move_to).unwrap();
            state.board.red[enemy] = 63;
        }
    } else {
        state.board.red[piece as usize] = move_to;
        if state.board.blue.contains(&move_to) {
            let enemy =
                state.board.blue.iter().position(|&x| x == move_to).unwrap();
            state.board.blue[enemy] = 63;
        }
    }

    // Toggles switches
    state.cur_blue = !state.cur_blue;
    state.won = check_win(&state);

    model.history.push(state);
    model.current += 1;
}

pub fn check_win(state: &State) -> bool {
    state.board.blue.contains(&DEN_RED)
        || state.board.red.contains(&DEN_BLUE)
        || state.board.blue.iter().all(|x| x > &62)
        || state.board.red.iter().all(|x| x > &62)
}

// Arrays of arrays are used,
// since, they are static anyways
static RIVER_MOVES: [[i32; 2]; 10] = [
    [15, 43],
    [16, 44],
    [18, 46],
    [19, 47],
    [21, 24],
    [28, 31],
    [35, 38],
    [24, 27],
    [31, 34],
    [38, 41],
];
static RIVER_LEAPS: [[i32; 3]; 10] = [
    [22, 29, 36],
    [23, 30, 37],
    [25, 32, 39],
    [26, 33, 40],
    [22, 23, 63],
    [29, 30, 63],
    [36, 37, 63],
    [25, 26, 63],
    [32, 33, 63],
    [39, 40, 63],
];
