use std::io::Write;
use std::process::{Command, Stdio};

fn run_engine(input: &str) -> String {
    let mut child = Command::new(env!("CARGO_BIN_EXE_perch"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("spawn perch binary");

    {
        let stdin = child.stdin.as_mut().expect("stdin available");
        stdin.write_all(input.as_bytes()).expect("write stdin");
    }

    let output = child.wait_with_output().expect("read stdout");
    assert!(output.status.success(), "engine exited with error");

    String::from_utf8_lossy(&output.stdout).to_string()
}

fn extract_line<'a>(output: &'a str, prefix: &str) -> Option<&'a str> {
    output
        .lines()
        .find(|line| line.trim_start().starts_with(prefix))
}

#[test]
fn uci_perft_startpos_depth_2() {
    let output = run_engine("position startpos\nperft 2\nquit\n");
    assert!(output.lines().any(|line| line.trim() == "nodes 400"));
}

#[test]
fn uci_go_checkmated_returns_0000() {
    let output = run_engine("position fen k7/8/8/8/8/8/1R6/R6K b - - 0 1\ngo depth 1\nquit\n");
    assert!(output.lines().any(|line| line.trim() == "bestmove 0000"));
}

#[test]
fn uci_perft_kiwipete_depth_1() {
    let output = run_engine(
        "position fen r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1\nperft 1\nquit\n",
    );
    assert!(output.lines().any(|line| line.trim() == "nodes 48"));
}

#[test]
fn uci_position_moves_pipeline_produces_nodes() {
    let output = run_engine("position startpos moves e2e4 e7e5 g1f3\nperft 1\nquit\n");
    let line = extract_line(&output, "nodes ").expect("nodes line present");
    let nodes: usize = line
        .trim_start()
        .strip_prefix("nodes ")
        .expect("nodes prefix")
        .parse()
        .expect("nodes number");
    assert!(nodes > 0);
}

#[test]
fn uci_go_returns_a_move_in_startpos() {
    let output = run_engine("position startpos\ngo depth 1\nquit\n");
    let line = extract_line(&output, "bestmove ").expect("bestmove line present");
    assert_ne!(line.trim(), "bestmove 0000");
}
