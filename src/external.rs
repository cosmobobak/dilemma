use std::{
    io::{BufRead, BufReader, Write},
    process::{Child, ChildStdin, ChildStdout},
    sync::Mutex,
};

use crate::rules::{Choice, Player};

/// A player connected to an external program.
/// Communicates using stdin and stdout.
pub struct ExePlayer {
    process: Child,
    stdout_stdin: Mutex<(BufReader<ChildStdout>, ChildStdin)>,
}

impl Drop for ExePlayer {
    fn drop(&mut self) {
        self.process.kill().unwrap();
    }
}

impl ExePlayer {
    pub fn new(process: Child, stdout: ChildStdout, stdin: ChildStdin) -> Self {
        Self {
            process,
            stdout_stdin: Mutex::new((BufReader::new(stdout), stdin)),
        }
    }
}

impl Player for ExePlayer {
    fn choose(
        &self,
        your_history: &[Choice],
        their_history: &[Choice],
        _: &mut fastrand::Rng,
    ) -> Choice {
        // send the history to the external program
        let mut message = String::new();
        message.push_str(&format!("{};", your_history.len()));
        for choice in your_history {
            message.push(match choice {
                Choice::Cooperate => 'C',
                Choice::Defect => 'D',
            });
        }
        message.push(';');
        for choice in their_history {
            message.push(match choice {
                Choice::Cooperate => 'C',
                Choice::Defect => 'D',
            });
        }
        message.push('\n');
        let mut response = String::with_capacity(64);
        // lock the child process's stdin and stdout
        let mut inout_lock = self.stdout_stdin.lock().unwrap();
        inout_lock.1.write_all(message.as_bytes()).unwrap();
        // read the response
        inout_lock.0.read_line(&mut response).unwrap();
        // drop the lock
        drop(inout_lock);
        match response.trim() {
            "C" => Choice::Cooperate,
            "D" => Choice::Defect,
            _ => panic!("[WARN]: Invalid response from external program: \n> {response}"),
        }
    }
}
