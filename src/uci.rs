use std::io::stdin;

use crate::bitboard::Bitboard;

pub(crate) struct Uci;

impl Uci {
    pub(crate) fn new() -> Self {
        Self
    }

    pub(crate) fn run(&self) {
        println!(
            "Perch v{}, use 'help' command to get more information",
            env!("CARGO_PKG_VERSION")
        );

        let stdin = stdin();

        for line in stdin.lines() {
            let line = line.unwrap();

            match UciCommand::try_from(line.as_str()) {
                Ok(cmd) => match cmd {
                    UciCommand::Display => println!("Display command invoked"),
                    UciCommand::Help => {
                        println!("Perch is a simple chess engine written in Rust by tontsa28!");
                    }
                    UciCommand::Position(fen) => println!("Interpreted FEN as: {fen}"),
                    UciCommand::Quit => return,
                },
                Err(e) => eprintln!("{e}"),
            }
        }
    }
}

pub(crate) enum UciCommand {
    Display,
    Help,
    Position(Bitboard),
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

        let fen = match parts.next() {
            Some("startpos") => Self::STARTPOS,
            _ => "",
        };

        Self::Position(Bitboard::from(fen))
    }
}
