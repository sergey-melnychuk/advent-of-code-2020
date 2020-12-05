use std::io::{self, BufRead};

pub fn lines() -> Vec<String> {
    let stdin = io::stdin();
    stdin.lock().lines()
        .map(|line| line.unwrap())
        .collect()
}
