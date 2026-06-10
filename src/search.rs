use std::collections::HashMap;

use crate::{mov::Move, piece::PieceKind, position::Position};

// Define infinity and mate scores
const INF: i32 = 1_073_741_824;
const MATE: i32 = 536_870_912;

/// Run a search up to the given depth in the given position using iterative deepening.
///
/// # Parameters
/// - `pos`: Position to search (mutated during search and restored).
/// - `depth`: Maximum search depth in plies.
///
/// # Returns
/// The best move found, or `None` if no legal moves exist.
pub(crate) fn iterative_deepening(pos: &mut Position, depth: u8) -> Option<Move> {
    // Root moves are generated once; each iteration reorders them using TT
    // which currently stores just best moves, not scores
    let mut tt: HashMap<Position, Move> = HashMap::new();
    let mut best_move = None;

    // Search position up to depth d iteratively
    for d in 1..=depth {
        let (mv, score) = search_root(pos, d, &mut tt);
        best_move = mv;

        // Log findings of this iteration
        println!(
            "info depth {d} score cp {score} pv {}",
            best_move
                .map(|mv| mv.to_string())
                .unwrap_or(String::from("0000"))
        );
    }

    best_move
}

/// Search the root node at a fixed depth.
///
/// # Parameters
/// - `pos`: Position to search (mutated during search and restored).
/// - `depth`: Maximum search depth in plies.
/// - `tt`: Transposition table used for move ordering.
///
/// # Returns
/// A tuple containing:
/// - The best move found, or `None` if no legal moves exist.
/// - The score from the side-to-move perspective.
fn search_root(
    pos: &mut Position,
    depth: u8,
    tt: &mut HashMap<Position, Move>,
) -> (Option<Move>, i32) {
    // Generate legal moves and check if there are any
    let mut moves = pos.legal_moves();
    if moves.is_empty() {
        // If side-to-move is in check, return mate score
        if pos.is_check(pos.turn()) {
            return (None, -MATE);
        } else {
            return (None, 0);
        }
    }

    // Set best score to negative infinity and get TT move if one exists
    let tt_move = tt.get(pos);

    // Improve move order
    moves.sort_by_key(|m| {
        if Some(m) == tt_move {
            // TT move gets highest priority
            0
        } else if m.is_promotion() {
            // Promotions are checked after TT move
            1
        } else if pos.is_capture(*m) {
            // Get attacker and victim of capture
            let (_, attacker) = pos.board().piece_at(m.from).into();
            let victim = if m.is_en_passant {
                PieceKind::Pawn
            } else {
                let (_, v) = pos.board().piece_at(m.to).into();
                v
            };

            // Captures are ordered from most to least valuable
            100 - MVV_LVA[victim as usize][attacker as usize] as i32
        } else {
            // Other moves get lowest priority
            200
        }
    });

    let mut best_move = None;
    let mut best_score = -INF;

    for mv in moves.iter().copied() {
        // Play a move and search its branch using negamax;
        // child score is negated and window is flipped
        let undo = pos.make_move(mv);
        let score = -search(pos, depth - 1, -INF, -best_score, 1, tt, false);
        pos.unmake_move(mv, undo);

        // If a new best score is found, set its move as best move
        if score > best_score {
            best_score = score;
            best_move = Some(mv);
        }
    }

    // Store the root best move so the next iteration can try it first.
    if let Some(mv) = best_move {
        tt.insert(*pos, mv);
    }

    (best_move, best_score)
}

/// Search a position recursively up to the given depth using the negamax algorithm.
///
/// # Parameters
/// - `pos`: Position to search (mutated during search and restored).
/// - `depth`: Remaining depth in plies.
/// - `alpha`: Lower bound of the alpha-beta window.
/// - `beta`: Upper bound of the alpha-beta window.
/// - `ply`: Distance from the root in plies.
/// - `tt`: Transposition table used for move ordering.
/// - `last_was_null`: Whether the previous move was a null move.
///
/// # Returns
/// The score from the side-to-move perspective.
fn search(
    pos: &mut Position,
    depth: u8,
    mut alpha: i32,
    beta: i32,
    ply: i32,
    tt: &mut HashMap<Position, Move>,
    last_was_null: bool,
) -> i32 {
    // At depth 0, switch to quiescence search to avoid horizon effects
    if depth == 0 {
        return quiescence(pos, alpha, beta, ply);
    }

    // Generate legal moves and check if there are any
    let mut moves = pos.legal_moves();
    if moves.is_empty() {
        // If side-to-move is in check, return mate score and take mate distance into account
        if pos.is_check(pos.turn()) {
            return -MATE + ply;
        } else {
            return 0;
        }
    }

    // Check if side-to-move is checked and if they have any non-pawn material
    let in_check = pos.is_check(pos.turn());
    let has_non_pawns = pos.board().has_non_pawns(pos.turn());

    // Null move pruning: try "passing" turn with a null window
    // to prove this node is too good for opponent
    if !in_check && !last_was_null && has_non_pawns && depth >= 3 {
        // Play a null move and search its branch using negamax;
        // child score is negated and window is flipped
        let ep = pos.make_null_move();
        let score = -search(pos, depth - 3, -beta, -beta + 1, ply + 1, tt, true);
        pos.unmake_null_move(ep);

        if score >= beta {
            return score;
        }
    }

    // Get TT move if one exists and improve move order
    let tt_move = tt.get(pos);
    moves.sort_by_key(|m| {
        if Some(m) == tt_move {
            // TT move gets highest priority
            0
        } else if m.is_promotion() {
            // Promotions are checked after TT move
            1
        } else if pos.is_capture(*m) {
            // Get attacker and victim of capture
            let (_, attacker) = pos.board().piece_at(m.from).into();
            let victim = if m.is_en_passant {
                PieceKind::Pawn
            } else {
                let (_, v) = pos.board().piece_at(m.to).into();
                v
            };

            // Captures are ordered from most to least valuable
            100 - MVV_LVA[victim as usize][attacker as usize]
        } else {
            // Other moves get lowest priority
            200
        }
    });

    // Set best score to negative infinity
    let mut best = -INF;
    let mut best_move = None;

    for mv in moves {
        // Play a move and search its branch
        let undo = pos.make_move(mv);
        let eval = -search(pos, depth - 1, -beta, -alpha, ply + 1, tt, false);
        pos.unmake_move(mv, undo);

        // If a new best score is found, set its move as best move
        if eval > best {
            best_move = Some(mv);
        }

        best = best.max(eval);
        alpha = alpha.max(best);

        if alpha >= beta {
            break;
        }
    }

    // If a best move was found, insert it into TT
    if let Some(mv) = best_move {
        tt.insert(*pos, mv);
    }

    best
}

/// Perform a quiescence search on the given position.
///
/// # Parameters
/// - `pos`: Position to search (mutated during search and restored).
/// - `alpha`: Lower bound of the alpha-beta window.
/// - `beta`: Upper bound of the alpha-beta window.
/// - `ply`: Distance from the root in plies.
///
/// # Returns
/// The quiescence score from the side-to-move perspective.
fn quiescence(pos: &mut Position, mut alpha: i32, beta: i32, ply: i32) -> i32 {
    let in_check = pos.is_check(pos.turn());

    // Stand-pat: static eval if we choose not to capture;
    // if already >= beta, cut immediately
    if !in_check {
        let stand_pat = pos.evaluate();
        if stand_pat >= beta {
            return stand_pat;
        }
        alpha = alpha.max(stand_pat);
    }

    // Generate legal moves and check if there are any
    let mut moves = pos.legal_moves();
    if moves.is_empty() {
        // If side-to-move is in check, return mate score and take mate distance into account
        return if in_check { -MATE + ply } else { 0 };
    }

    // If not in check, only search captures and promotions (ignore quiet moves)
    if !in_check {
        moves.retain(|m| pos.is_capture(*m) || m.is_promotion());
    }

    // Improve move order
    moves.sort_by_key(|m| {
        if m.is_promotion() {
            // Promotions get highest priority
            1
        } else if pos.is_capture(*m) {
            // Get attacker and victim of capture
            let (_, attacker) = pos.board().piece_at(m.from).into();
            let victim = if m.is_en_passant {
                PieceKind::Pawn
            } else {
                let (_, v) = pos.board().piece_at(m.to).into();
                v
            };

            // Captures are ordered from most to least valuable
            100 - MVV_LVA[victim as usize][attacker as usize]
        } else {
            // Other moves get lowest priority
            200
        }
    });

    let mut best = if in_check { -INF } else { alpha };

    for mv in moves {
        // Play a move and quiescence search its branch
        let undo = pos.make_move(mv);
        let score = -quiescence(pos, -beta, -alpha, ply + 1);
        pos.unmake_move(mv, undo);

        best = best.max(score);
        alpha = alpha.max(best);

        if alpha >= beta {
            return best;
        }
    }

    best
}

/// Count the number of reachable nodes up to the given depth.
///
/// # Parameters
/// - `pos`: Position to expand (mutated during traversal and restored).
/// - `depth`: Remaining depth in plies.
///
/// # Returns
/// Total node count at the given depth.
pub(crate) fn perft(pos: &mut Position, depth: u8) -> usize {
    // If requested depth is reached, return node count (always 1 for an individual position)
    if depth == 0 {
        return 1;
    }

    let mut nodes = 0;
    for mv in pos.legal_moves() {
        // Play a move and count its child nodes
        let undo = pos.make_move(mv);
        nodes += perft(pos, depth - 1);
        pos.unmake_move(mv, undo);
    }

    nodes
}

// Construct the MVV-LVA table
const MVV_LVA: [[u8; 6]; 6] = [
    [15, 14, 13, 12, 11, 10],
    [25, 24, 23, 22, 21, 20],
    [35, 34, 33, 32, 31, 30],
    [45, 44, 43, 42, 41, 40],
    [55, 54, 53, 52, 51, 50],
    [65, 64, 63, 62, 61, 60],
];
