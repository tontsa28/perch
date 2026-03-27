use shakmaty::{Chess, Move, Position, Role, Square};

const INF: i32 = 1_073_741_824;
const MATE: i32 = 536_870_912;

pub(crate) fn best_move(pos: &Chess, depth: u8) -> Option<Move> {
    let mut best_score = -INF;
    let mut best_move = None;

    for mv in pos.legal_moves() {
        let new_pos = pos.clone().play(mv).unwrap();
        let score = -search(&new_pos, depth - 1, -INF, INF);

        if score > best_score {
            best_score = score;
            best_move = Some(mv);
        }
    }

    best_move
}

fn search(pos: &Chess, depth: u8, mut alpha: i32, beta: i32) -> i32 {
    if depth == 0 {
        return evaluate(pos);
    }

    let moves = pos.legal_moves();
    let mut best = -INF;

    if moves.is_empty() {
        if pos.is_check() {
            return -MATE + depth as i32;
        } else {
            return 0;
        }
    }

    for mv in moves {
        let new_pos = pos.clone().play(mv).unwrap();
        let eval = -search(&new_pos, depth - 1, -beta, -alpha);
        best = best.max(eval);
        alpha = alpha.max(best);

        if alpha >= beta {
            break;
        }
    }

    best
}

fn evaluate(pos: &Chess) -> i32 {
    let mut score = 0;

    for sq in Square::ALL {
        if let Some(piece) = pos.board().piece_at(sq) {
            let value = match piece.role {
                Role::Pawn => 100,
                Role::Knight => 320,
                Role::Bishop => 330,
                Role::Rook => 500,
                Role::Queen => 900,
                Role::King => 0,
            };

            if piece.color == pos.turn() {
                score += value;
            } else {
                score -= value;
            }
        }
    }

    score
}
