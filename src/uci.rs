use std::io::stdin;

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
                    UciCommand::Help => {
                        println!("Perch is a simple chess engine written in Rust by tontsa28!");
                    }
                    UciCommand::Quit => return,
                },
                Err(e) => eprintln!("{e}"),
            }
        }
    }
}

pub(crate) enum UciCommand {
    Help,
    Quit,
}

impl TryFrom<&str> for UciCommand {
    type Error = &'static str;

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        let line = line.trim();

        let cmd = match line {
            "help" => Self::Help,
            "quit" | "exit" => Self::Quit,
            _ => return Err("Unknown command."),
        };

        Ok(cmd)
    }
}
