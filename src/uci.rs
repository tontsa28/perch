use std::{io::stdin, result::Result as StdResult};

use crate::{
    error::{Error, Result},
    position::Position,
    search::iterative_deepening,
};

pub(crate) struct Uci {
    chess: Position,
}

impl Uci {
    pub(crate) fn new() -> Self {
        Self {
            chess: Position::new(),
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

    fn go(&mut self, depth: Option<u8>) -> String {
        iterative_deepening(&mut self.chess, depth.unwrap_or(6))
            .map(|m| m.to_string())
            .unwrap_or(String::from("0000"))
    }
}

pub(crate) enum UciCommand {
    Display,
    Help,
    Go { depth: Option<u8> },
    Position(Position),
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

        let mut position = Position::try_from(fen_str)?;

        for mv in moves {
            let m = position.parse_uci_move(mv).unwrap();
            position.make_move(m);
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
