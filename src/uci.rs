use std::{io::stdin, result::Result as StdResult};

use crate::{
    error::{Error, Result},
    position::Position,
    search::{iterative_deepening, perft},
};

/// The entrypoint to the UCI interface.
pub(crate) struct Uci {
    chess: Position,
}

impl Uci {
    /// Initialize a new UCI interface.
    ///
    /// # Returns
    /// A `Uci` instance with the starting position loaded.
    pub(crate) fn new() -> Self {
        Self {
            chess: Position::new(),
        }
    }

    /// Run the UCI interface (effectively the same as running the program).
    ///
    /// # Returns
    /// Nothing. This function runs until input ends or `quit` is received.
    pub(crate) fn run(&mut self) {
        println!(
            "Perch v{}, run 'help' to get more information",
            env!("CARGO_PKG_VERSION")
        );

        let stdin = stdin();

        for line in stdin.lines() {
            let line = line.unwrap();

            // Attempt to convert line into a UCI command
            match UciCommand::try_from(line.as_str()) {
                Ok(cmd) => match cmd {
                    UciCommand::Display => println!("{}", self.chess.board()),
                    UciCommand::Help => {
                        println!("Perch is a simple chess engine written in Rust by tontsa28!");
                    }
                    UciCommand::Go { depth } => println!("bestmove {}", self.go(depth)),
                    UciCommand::Perft { depth } => println!("nodes {}", self.perft(depth)),
                    UciCommand::Position(chess) => self.chess = chess,
                    UciCommand::Quit => return,
                },
                Err(e) => eprintln!("{e}"),
            }
        }
    }

    /// Execute the `go` command and return the best move.
    ///
    /// # Parameters
    /// - `depth`: Optional search depth in plies. Defaults to 6 when `None`.
    ///
    /// # Returns
    /// The best move in UCI format, or `0000` if no move exists.
    fn go(&mut self, depth: Option<u8>) -> String {
        // Run iterative deepening with 6 as default depth and convert move into a string
        iterative_deepening(&mut self.chess, depth.unwrap_or(6))
            .map(|m| m.to_string())
            .unwrap_or(String::from("0000"))
    }

    /// Execute the `perft` command.
    ///
    /// # Parameters
    /// - `depth`: Optional perft depth. Defaults to 0 when `None`.
    ///
    /// # Returns
    /// The node count at the given depth.
    fn perft(&mut self, depth: Option<u8>) -> usize {
        // Run perft with 0 as default depth (count only root node)
        perft(&mut self.chess, depth.unwrap_or(0))
    }
}

/// A valid UCI command.
pub(crate) enum UciCommand {
    Display,
    Help,
    Go { depth: Option<u8> },
    Perft { depth: Option<u8> },
    Position(Position),
    Quit,
}

impl TryFrom<&str> for UciCommand {
    type Error = Error;

    /// Convert a string into a `UciCommand`.
    ///
    /// # Parameters
    /// - `line`: Raw input line from the user.
    ///
    /// # Returns
    /// A parsed `UciCommand`, or an error if the command is unknown.
    fn try_from(line: &str) -> StdResult<Self, Self::Error> {
        let line = line.trim();

        // Map string commands into their respective UciCommand counterparts
        match line {
            "d" => Ok(Self::Display),
            "help" => Ok(Self::Help),
            "quit" | "exit" => Ok(Self::Quit),
            _ => {
                if line.starts_with("position") {
                    Self::position(line)
                } else if line.starts_with("go") {
                    Self::go(line)
                } else if line.starts_with("perft") {
                    Self::perft(line)
                } else {
                    Err("Unknown command.")?
                }
            }
        }
    }
}

impl UciCommand {
    /// The default starting position FEN string.
    const STARTPOS: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

    /// Parse the `position` command and its parameters.
    ///
    /// # Parameters
    /// - `line`: Full `position` command line.
    ///
    /// # Returns
    /// A `UciCommand::Position` with the resulting position, or an error.
    fn position(line: &str) -> Result<Self> {
        let mut parts = line.split_whitespace();

        // Make sure command starts with 'position' and also consume it
        assert_eq!(parts.next(), Some("position"));

        // Capture FEN if provided
        let fen_str = match parts.next() {
            Some("startpos") => Self::STARTPOS,
            Some("fen") => {
                let fen_parts: Vec<&str> = parts.by_ref().take(6).collect();
                &fen_parts.join(" ")
            }
            _ => "",
        };

        // Capture all moves if provided
        let moves = if parts.next() == Some("moves") {
            parts.collect::<Vec<&str>>()
        } else {
            Vec::with_capacity(0)
        };

        // Attempt to convert captured FEN into a `Position`
        let mut position = Position::try_from(fen_str)?;

        // Play all captured moves to get to final position
        for mv in moves {
            let m = position.parse_uci_move(mv).unwrap();
            position.make_move(m);
        }

        Ok(Self::Position(position))
    }

    /// Parse the `go` command and its parameters.
    ///
    /// # Parameters
    /// - `line`: Full `go` command line.
    ///
    /// # Returns
    /// A `UciCommand::Go` with the parsed depth (if any).
    fn go(line: &str) -> Result<Self> {
        let mut parts = line.split_whitespace();

        // Make sure command starts with 'go' and also consume it
        assert_eq!(parts.next(), Some("go"));

        // Capture depth if provided
        if parts.next() == Some("depth") {
            let depth = parts.next().map(|s| s.parse::<u8>()).transpose()?;
            return Ok(Self::Go { depth });
        }

        Ok(Self::Go { depth: None })
    }

    /// Parse the `perft` command and its parameters.
    ///
    /// # Parameters
    /// - `line`: Full `perft` command line.
    ///
    /// # Returns
    /// A `UciCommand::Perft` with the parsed depth (if any).
    fn perft(line: &str) -> Result<Self> {
        let mut parts = line.split_whitespace();

        // Make sure command starts with 'perft' and also consume it
        assert_eq!(parts.next(), Some("perft"));

        // Capture depth if provided
        if let Some(arg) = parts.next() {
            let depth = arg.parse::<u8>().ok();
            return Ok(Self::Perft { depth });
        }

        Ok(Self::Perft { depth: None })
    }
}
