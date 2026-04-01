use crate::{mov::Move, position::Position};

const INF: i32 = 1_073_741_824;
const MATE: i32 = 536_870_912;

// pub(crate) fn iterative_deepening(pos: Position, depth: u8) -> Option<Move> {
//     let mut best = None;

//     for d in 1..=depth {
//         best = best_move(pos, d, best);
//     }

//     best
// }

pub(crate) fn best_move(pos: Position, depth: u8) -> Option<Move> {
    let mut best_score = -INF;
    let mut best_move = None;
    let moves = pos.legal_moves();

    // if let Some(best) = prev_best {
    //     moves.sort_by_key(|m| if *m == best { 0 } else { 1 });
    // }

    for mv in moves {
        let new_pos = pos.make_move_cloned(mv);
        let score = -search(new_pos, depth - 1, -INF, INF);

        if score > best_score {
            best_score = score;
            best_move = Some(mv);
        }
    }

    best_move
}

fn search(pos: Position, depth: u8, mut alpha: i32, beta: i32) -> i32 {
    if depth == 0 {
        return pos.evaluate();
    }

    let moves = pos.legal_moves();
    //moves.sort_by_key(|m| !m.is_capture());
    let mut best = -INF;

    if moves.is_empty() {
        if pos.is_checkmate() {
            return -MATE + depth as i32;
        } else {
            return 0;
        }
    }

    for mv in moves {
        let new_pos = pos.make_move_cloned(mv);
        let eval = -search(new_pos, depth - 1, -beta, -alpha);
        best = best.max(eval);
        alpha = alpha.max(best);

        if alpha >= beta {
            break;
        }
    }

    best
}

// fn quiescence(pos: Position, mut alpha: i32, beta: i32) -> i32 {
//     let stand_pat = pos.evaluate();

//     if stand_pat >= beta {
//         return beta;
//     }

//     alpha = alpha.max(stand_pat);

//     for mv in pos.legal_moves() {
//         if !mv.is_capture() {
//             continue;
//         }

//         let new_pos = pos.make_move_cloned(mv);
//         let score = -quiescence(new_pos, -beta, -alpha);

//         if score >= beta {
//             return beta;
//         }
//         alpha = alpha.max(score);
//     }

//     alpha
// }
