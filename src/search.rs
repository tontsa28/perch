use crate::{mov::Move, piece::PieceKind, position::Position};

const INF: i32 = 1_073_741_824;
const MATE: i32 = 536_870_912;

pub(crate) fn iterative_deepening(pos: &mut Position, depth: u8) -> Option<Move> {
    let mut best = None;

    for d in 1..=depth {
        let mut best_score = -INF;
        let mut moves = pos.legal_moves();

        moves.sort_by_key(|m| {
            if Some(*m) == best {
                0
            } else if m.is_promotion() {
                1
            } else if pos.is_capture(*m) {
                let (_, attacker) = pos.board().piece_at(m.from).into();
                let victim = if m.is_en_passant {
                    PieceKind::Pawn
                } else {
                    let (_, v) = pos.board().piece_at(m.to).into();
                    v
                };
                100 - MVV_LVA[victim as usize][attacker as usize] as i32
            } else {
                200
            }
        });

        for mv in moves {
            let undo = pos.make_move(mv);
            let score = -search(pos, d - 1, -INF, -best_score, 1);
            pos.unmake_move(mv, undo);

            if score > best_score {
                best_score = score;
                best = Some(mv);
            }
        }

        println!(
            "info depth {d} score cp {best_score} pv {}",
            best.map(|mv| mv.to_string())
                .unwrap_or(String::from("0000"))
        );
    }

    best
}

fn search(pos: &mut Position, depth: u8, mut alpha: i32, beta: i32, ply: i32) -> i32 {
    if depth == 0 {
        return quiescence(pos, alpha, beta, ply);
    }

    let mut moves = pos.legal_moves();
    if moves.is_empty() {
        if pos.is_check(pos.turn()) {
            return -MATE + ply;
        } else {
            return 0;
        }
    }

    let mut best = -INF;
    moves.sort_by_key(|m| {
        if m.is_promotion() {
            0
        } else if pos.is_capture(*m) {
            let (_, attacker) = pos.board().piece_at(m.from).into();
            let victim = if m.is_en_passant {
                PieceKind::Pawn
            } else {
                let (_, v) = pos.board().piece_at(m.to).into();
                v
            };
            100 - MVV_LVA[victim as usize][attacker as usize]
        } else {
            200
        }
    });

    for mv in moves {
        let undo = pos.make_move(mv);
        let eval = -search(pos, depth - 1, -beta, -alpha, ply + 1);
        pos.unmake_move(mv, undo);

        best = best.max(eval);
        alpha = alpha.max(best);

        if alpha >= beta {
            break;
        }
    }

    best
}

fn quiescence(pos: &mut Position, mut alpha: i32, beta: i32, ply: i32) -> i32 {
    let in_check = pos.is_check(pos.turn());

    if !in_check {
        let stand_pat = pos.evaluate();
        if stand_pat >= beta {
            return stand_pat;
        }
        alpha = alpha.max(stand_pat);
    }

    let mut moves = pos.legal_moves();

    if moves.is_empty() {
        return if in_check { -MATE + ply } else { 0 };
    }

    if !in_check {
        moves.retain(|m| pos.is_capture(*m) || m.is_promotion());
    }

    moves.sort_by_key(|m| {
        if m.is_promotion() {
            0
        } else if pos.is_capture(*m) {
            let (_, attacker) = pos.board().piece_at(m.from).into();
            let victim = if m.is_en_passant {
                PieceKind::Pawn
            } else {
                let (_, v) = pos.board().piece_at(m.to).into();
                v
            };
            100 - MVV_LVA[victim as usize][attacker as usize]
        } else {
            200
        }
    });

    let mut best = if in_check { -INF } else { alpha };

    for mv in moves {
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

const MVV_LVA: [[u8; 6]; 6] = [
    [15, 14, 13, 12, 11, 10],
    [25, 24, 23, 22, 21, 20],
    [35, 34, 33, 32, 31, 30],
    [45, 44, 43, 42, 41, 40],
    [55, 54, 53, 52, 51, 50],
    [65, 64, 63, 62, 61, 60],
];
