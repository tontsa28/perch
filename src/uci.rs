use std::{io::stdin, result::Result as StdResult, str::FromStr};

use shakmaty::{CastlingMode, Chess, Position, fen::Fen, uci::UciMove};

use crate::{
    error::{Error, Result},
    position::Position as ChessPosition,
    search::best_move,
};

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
                    UciCommand::Go { depth } => println!("bestmove {}", self.go(depth)),
                    UciCommand::Position(chess) => self.chess = chess,
                    UciCommand::Quit => return,
                },
                Err(e) => eprintln!("{e}"),
            }
        }
    }

    fn go(&self, depth: Option<u8>) -> String {
        let mv = best_move(&self.chess, depth.unwrap_or(5)).unwrap();
        mv.to_uci(CastlingMode::Standard).to_string()
    }
}

pub(crate) enum UciCommand {
    Display,
    Help,
    Go { depth: Option<u8> },
    Position(Chess),
    Quit,
}

impl TryFrom<&str> for UciCommand {
    type Error = Error;

    fn try_from(line: &str) -> StdResult<Self, Self::Error> {
        let line = line.trim();

        match line {
            "d" => Ok(Self::Display),
            "help" => Ok(Self::Help),
            "quit" | "exit" => Ok(Self::Quit),
            _ => {
                if line.starts_with("position") {
                    Self::position(line)
                } else if line.starts_with("go") {
                    Self::go(line)
                } else {
                    Err("Unknown command.")?
                }
            }
        }
    }
}

impl UciCommand {
    const STARTPOS: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

    fn position(line: &str) -> Result<Self> {
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

        //dbg!(ChessPosition::try_from(fen_str)?);

        let fen = Fen::from_str(fen_str).unwrap();
        let mut position: Chess = fen.into_position(shakmaty::CastlingMode::Standard).unwrap();

        for mv in moves {
            let uci = mv.parse::<UciMove>().unwrap();
            let m = uci.to_move(&position).unwrap();
            position = position.play(m).unwrap();
        }

        Ok(Self::Position(position))
    }

    fn go(line: &str) -> Result<Self> {
        let mut parts = line.split_whitespace();

        assert_eq!(parts.next(), Some("go"));

        if parts.next() == Some("depth") {
            let depth = parts.next().map(|s| s.parse::<u8>()).transpose()?;
            return Ok(Self::Go { depth });
        }

        Ok(Self::Go { depth: None })
    }
}
