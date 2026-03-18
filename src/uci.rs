use std::{io::stdin, str::FromStr};

use rand::{rng, seq::IndexedRandom};
use shakmaty::{CastlingMode, Chess, Move, Position, fen::Fen, uci::UciMove};

pub(crate) struct Uci {
    chess: Chess,
}

impl Uci {
    pub(crate) fn new() -> Self {
        Self {
            chess: Chess::new(),
        }
    }

    pub(crate) fn run(&mut self) {
        println!(
            "Perch v{}, run 'help' to get more information",
            env!("CARGO_PKG_VERSION")
        );

        let stdin = stdin();

        for line in stdin.lines() {
            let line = line.unwrap();

            match UciCommand::try_from(line.as_str()) {
                Ok(cmd) => match cmd {
                    UciCommand::Display => println!("{}", self.chess.board()),
                    UciCommand::Help => {
                        println!("Perch is a simple chess engine written in Rust by tontsa28!");
                    }
                    UciCommand::Go => println!("bestmove {}", self.go()),
                    UciCommand::Position(chess) => self.chess = chess,
                    UciCommand::Quit => return,
                },
                Err(e) => eprintln!("{e}"),
            }
        }
    }

    fn go(&self) -> String {
        let legal_moves = self.chess.legal_moves();
        let mov: &Move = legal_moves.choose(&mut rng()).unwrap();
        mov.to_uci(CastlingMode::Standard).to_string()
    }
}

pub(crate) enum UciCommand {
    Display,
    Help,
    Go,
    Position(Chess),
    Quit,
}

impl TryFrom<&str> for UciCommand {
    type Error = &'static str;

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        let line = line.trim();

        let cmd = match line {
            "d" => Self::Display,
            "help" => Self::Help,
            "quit" | "exit" => Self::Quit,
            _ => {
                if line.starts_with("position") {
                    Self::position(line)
                } else if line.starts_with("go") {
                    Self::Go
                } else {
                    return Err("Unknown command.");
                }
            }
        };

        Ok(cmd)
    }
}

impl UciCommand {
    const STARTPOS: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

    fn position(line: &str) -> Self {
        let mut parts = line.split_whitespace();

        // Panic if the first part is not position
        assert_eq!(parts.next(), Some("position"));

        let fen_str = match parts.next() {
            Some("startpos") => Self::STARTPOS,
            Some("fen") => {
                let fen_parts: Vec<&str> = parts.by_ref().take(6).collect();
                &fen_parts.join(" ")
            }
            _ => "",
        };

        let moves = if parts.next() == Some("moves") {
            parts.collect::<Vec<&str>>()
        } else {
            Vec::with_capacity(0)
        };

        let fen = Fen::from_str(fen_str).unwrap();
        let mut position: Chess = fen.into_position(shakmaty::CastlingMode::Standard).unwrap();

        for mv in moves {
            let uci = mv.parse::<UciMove>().unwrap();
            let m = uci.to_move(&position).unwrap();
            position = position.play(m).unwrap();
        }

        Self::Position(position)
    }
}
