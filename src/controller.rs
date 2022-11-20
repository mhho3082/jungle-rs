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
            if out[i] > 62 && RIVERS.contains(&(original + e)) {
                for x in RIVER_MOVES {
                    if x.contains(&original) {
                        let temp = x.iter().filter(|y| y != &&original).sum();
                        if ((e < &0 && temp < original)
                            || (e > &0 && temp > original))
                            && check_move(state, piece, temp)
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

/// Checks if a move is legal
pub fn check_move(state: &State, piece: i32, move_to: i32) -> bool {
    check_walk(state, piece, move_to) && check_capture(state, piece, move_to)
}

/// Checks if the movement is legal
/// Ignores captures (or blockages)
fn check_walk(state: &State, piece: i32, move_to: i32) -> bool {
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

    // Checks if cross-border (except for rat)
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
            RIVER_MOVES.iter().position(|&x| x == [move_to, original])
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
fn check_capture(state: &State, piece: i32, move_to: i32) -> bool {
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

    // Get original location
    let original = if state.cur_blue {
        state.board.blue[piece as usize]
    } else {
        state.board.red[piece as usize]
    };

    // Check if cross-border capture
    if (RIVERS.contains(&move_to) && !RIVERS.contains(&original))
        || (RIVERS.contains(&original) && !RIVERS.contains(&move_to))
    {
        return false;
    }

    // Get enemy piece
    let enemy = if state.cur_blue {
        state.board.red.iter().position(|&x| x == move_to).unwrap() as i32
    } else {
        state.board.blue.iter().position(|&x| x == move_to).unwrap() as i32
    };

    // Check capture
    if piece >= enemy || (piece == 0 && enemy == 7) {
        true
    } else if state.cur_blue {
        TRAPS_BLUE.contains(&move_to)
    } else {
        TRAPS_RED.contains(&move_to)
    }
}

/// Checks that a capture can be made (for AI)
/// Assumes that the capture, if any, is legal
pub fn find_capture(state: &State, move_to: i32) -> bool {
    if state.cur_blue {
        state.board.red.contains(&move_to)
    } else {
        state.board.blue.contains(&move_to)
    }
}

/// Makes a given move
pub fn make_move(
    model: &mut Model,
    piece: i32,
    move_to: i32,
) -> Result<(), ()> {
    let mut state = *model.curr();

    if !check_move(&state, piece, move_to) {
        return Err(());
    }

    // Move piece and make capture if needed
    if state.cur_blue {
        state.board.blue[piece as usize] = move_to;
        if let Some(enemy) = state.board.red.iter().position(|&x| x == move_to)
        {
            state.board.red[enemy] = 63;
        }
    } else {
        state.board.red[piece as usize] = move_to;
        if let Some(enemy) = state.board.blue.iter().position(|&x| x == move_to)
        {
            state.board.blue[enemy] = 63;
        }
    }

    // Toggles switches
    state.cur_blue = !state.cur_blue;
    state.won = check_win(&state);

    // Pop all states after current state
    while model.history.len() > model.current + 1 {
        model.history.pop();
    }

    model.history.push(state);
    model.current += 1;

    Ok(())
}

/// Checks if a winning condition is reached in a state
pub fn check_win(state: &State) -> bool {
    state.board.blue.contains(&DEN_RED)
        || state.board.red.contains(&DEN_BLUE)
        || state.board.blue.iter().all(|x| x > &62)
        || state.board.red.iter().all(|x| x > &62)
}

/// Makes a time travel move
pub fn make_travel(model: &mut Model, jump: i32) -> Result<(), ()> {
    let temp = model.current as i32 + jump;
    if temp < 0 || temp >= model.history.len() as i32 {
        Err(())
    } else {
        model.current = temp as usize;
        Ok(())
    }
}
