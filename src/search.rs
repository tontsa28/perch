use crate::{mov::Move, position::Position};

const INF: i32 = 1_073_741_824;
const MATE: i32 = 536_870_912;

pub(crate) fn iterative_deepening(pos: &mut Position, depth: u8) -> Option<Move> {
    let mut best = None;
    let mut score;
    let mut nodes = 0;

    for d in 1..=depth {
        (best, score) = best_move(pos, d, best, &mut nodes);
        println!(
            "info depth {d} score cp {score} pv {}",
            best.map(|mv| mv.to_string())
                .unwrap_or(String::from("0000"))
        );
    }

    println!("nodes {nodes}");

    best
}

pub(crate) fn best_move(
    pos: &mut Position,
    depth: u8,
    prev_best: Option<Move>,
    mut nodes: &mut u64,
) -> (Option<Move>, i32) {
    let mut best_score = -INF;
    let mut best_move = None;
    let mut moves = pos.legal_moves();

    if let Some(best) = prev_best {
        moves.sort_by_key(|m| if *m == best { 0 } else { 1 });
    }

    for mv in moves {
        let undo = pos.make_move(mv);
        let score = -search(pos, depth - 1, -INF, INF, 1, &mut nodes);
        pos.unmake_move(mv, undo);

        if score > best_score {
            best_score = score;
            best_move = Some(mv);
        }
    }

    (best_move, best_score)
}

fn search(
    pos: &mut Position,
    depth: u8,
    mut alpha: i32,
    beta: i32,
    ply: i32,
    nodes: &mut u64,
) -> i32 {
    *nodes += 1;
    if depth == 0 {
        return pos.evaluate();
    }

    let mut moves = pos.legal_moves();
    moves.sort_by_key(|m| {
        if m.is_promotion() {
            0
        } else if pos.is_capture(*m) {
            1
        } else {
            2
        }
    });

    if moves.is_empty() {
        if pos.is_check(pos.turn()) {
            return -MATE + ply;
        } else {
            return 0;
        }
    }

    let mut best = -INF;

    for mv in moves {
        let undo = pos.make_move(mv);
        let eval = -search(pos, depth - 1, -beta, -alpha, ply + 1, nodes);
        pos.unmake_move(mv, undo);

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

pub(crate) fn perft(pos: &mut Position, depth: u8) -> usize {
    if depth == 0 {
        return 1;
    }

    let mut nodes = 0;
    for mv in pos.legal_moves() {
        let undo = pos.make_move(mv);
        nodes += perft(pos, depth - 1);
        pos.unmake_move(mv, undo);
    }

    nodes
}
