# perch
Perch is a simple chess engine written in Rust, using a bitboard-based board representation and UCI-style command handling.

## Features

- Bitboard board representation
- FEN parsing
- Legal move generation (including castling, en passant, promotions)
- Alpha-beta / negamax style search with iterative deepening
- UCI-like command loop
- ASCII board display (`d` command)

---

## Requirements

- Rust toolchain (stable)
- Cargo

Since cargo is a component of the Rust toolchain, it does not need to be installed manually when installing the whole toolchain.
The Rust toolchain can be installed in many ways.
Primarily, it is recommended to use the distribution package if a system with a package manager is used.
The exact package name may depend on the manager.
As an example, Arch Linux installation goes as follows:
```
sudo pacman -S rustup
```
If you don't have or don't want to use a package manager, rustup installation instructions can be found at [rustup.rs](`https://rustup.rs`).

---

## Build

From the project root:
```
cargo build --release
```

This produces an optimized binary at:
```
target/release/perch
```

---

## Run

Start the engine:
```
cargo run --release
```

or run the binary directly:
```
./target/release/perch
```

On startup, Perch enters a command loop reading from stdin.

---

## Basic commands

Perch currently supports a small command set.

### Help

```
help
```

Shows a short help message.

### Display board

```
d
```

Prints the current board as ASCII.

### Set position

Start position:
```
position startpos
```

Custom FEN:
```
position fen rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1
```

FEN plus played moves:
```
position startpos moves e2e4 e7e5 g1f3
```

### Search move

Default depth:
```
go
```

Specific depth:
```
go depth 8
```

Engine returns:
```
bestmove <uci-move>
```

### Quit

```
quit
```

or

```
exit
```

---

## UCI notes

Perch follows a UCI-style workflow, but is not yet a full Stockfish-compatible UCI implementation in every detail.

If you already know Stockfish/UCI basics, interaction style should feel familiar:
- `position ...`
- `go ...`
- `bestmove ...`

For full UCI protocol background, see Stockfish/UCI docs.

---

## Move notation

Moves are in UCI coordinate notation:
- normal: `e2e4`
- promotion: `e7e8q`
- castling: `e1g1`, `e1c1`, `e8g8`, `e8c8`

---

## Development

Run in debug mode (faster compile, slower engine):
```
cargo run
```

Format:
```
cargo fmt
```

Lint:
```
cargo clippy -- -D warnings
```

---

## Troubleshooting

### `illegal move`
- Ensure the position is set correctly before sending moves.
- Verify move string uses UCI format (e.g., `e2e4`, not SAN like `Nf3`).

### Slow search
- Make sure you run release build.
- Check requested depth.
- Verify no debug assertions are enabled in profiling builds.

---

## Disclaimer

This project is related to a uni course. The course-related documents are available in the course/ folder at least until the end of the course.
