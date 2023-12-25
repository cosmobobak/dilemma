use std::{io::{BufRead, Write, BufReader}, process::{Child, ChildStdout, ChildStdin}};

use crate::rules::{Player, Choice};

/// A player connected to an external program.
/// Communicates using stdin and stdout.
pub struct ExePlayer {
    name: String,
    _process: Child,
    stdout: BufReader<ChildStdout>,
    stdin: ChildStdin,
}

impl ExePlayer {
    pub fn new(name: String, process: Child, stdout: ChildStdout, stdin: ChildStdin) -> Self {
        let stdout = BufReader::new(stdout);
        Self { name, _process: process, stdout, stdin }
    }
}

impl Player for ExePlayer {
    fn choose(&mut self, your_history: &[Choice], their_history: &[Choice]) -> Choice {
        // send the history to the external program
        let mut message = String::new();
        message.push_str(&format!("{} ", your_history.len()));
        for choice in your_history {
            message.push(match choice {
                Choice::Cooperate => 'C',
                Choice::Defect => 'D',
            });
        }
        message.push(' ');
        for choice in their_history {
            message.push(match choice {
                Choice::Cooperate => 'C',
                Choice::Defect => 'D',
            });
        }
        message.push('\n');
        self.stdin.write_all(message.as_bytes()).unwrap();

        // read the response
        let mut response = String::new();
        self.stdout.read_line(&mut response).unwrap();
        match response.trim() {
            "C" => Choice::Cooperate,
            "D" => Choice::Defect,
            _ => panic!("[WARN]: Invalid response from external program: \n> {response}"),
        }
    }

    fn strategy(&self) -> String {
        self.name.clone()
    }
}