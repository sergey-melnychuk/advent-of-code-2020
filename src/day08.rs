use crate::utils::lines;
use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq, Clone)]
enum Op {
    Acc(i64),
    Jmp(i64),
    Nop(i64),
}

struct Run {
    code: Vec<Op>,
    acc: i64,
    op: usize,
    done: bool,
}

impl Run {
    fn new(code: &Vec<Op>) -> Self {
        Self {
            code: code.clone(),
            acc: 0,
            op: 0,
            done: false,
        }
    }

    fn acc(&self) -> i64 {
        self.acc
    }

    fn op(&self) -> usize {
        self.op
    }

    fn terminated(&self) -> bool {
        self.done
    }

    // Run single step, return if program is still running.
    //      "The program is supposed to terminate by attempting to execute
    //      an instruction immediately after the last instruction in the file."
    fn step(&mut self) {
        if self.done {
            return;
        }

        let op = self.code.get(self.op).unwrap();
        match op {
            Op::Jmp(arg) => {
                let offset = arg.abs() as usize;
                if *arg < 0 {
                    self.op -= offset;
                } else {
                    self.op += offset;
                }
            },
            Op::Acc(arg) => {
                self.acc += *arg;
                self.op += 1;
            }
            Op::Nop(_) => {
                self.op += 1;
            }
        }

        if self.op == self.code.len() {
            self.done = true;
        }
    }
}

// Run the code and terminate if an infinite loop is detected (op is revisited)
// or if the program terminated (not running after performing the step).
fn exec(code: &Vec<Op>) -> Run {
    let mut visited: HashSet<usize> = HashSet::with_capacity(code.len());

    let mut run = Run::new(code);
    while !visited.contains(&run.op()) {
        visited.insert(run.op());
        run.step();
        if run.terminated() {
            break;
        }
    }

    run
}

// Pick candidates for jmp <-> nop swap (indices).
fn pick(code: &Vec<Op>) -> Vec<usize> {
    code.iter().enumerate()
        .filter_map(|(i, e)| {
            match e {
                Op::Jmp(_) | Op::Nop(_) => Some(i),
                _ => None
            }
        })
        .collect()
}

// Return copy of code with jmp <-> nop swap at given index
fn swap(mut code: Vec<Op>, at: usize) -> Vec<Op> {
    fn flip(op: &Op) -> Op {
        match op {
            Op::Jmp(arg) => Op::Nop(*arg),
            Op::Nop(arg) => Op::Jmp(*arg),
            x => x.clone()
        }
    }

    let op = code.get(at).unwrap();
    *code.get_mut(at).unwrap() = flip(op);

    code
}

// Find the jmp <-> nop swap that terminates the program
fn find(code: Vec<Op>) -> Option<Run> {
    let swaps = pick(&code);

    for i in swaps {
        let modified = swap(code.clone(), i);
        let run = exec(&modified);
        if run.terminated() {
            return Some(run);
        }
    }

    None
}

fn parse(op: &str) -> Op {
    let tokens: Vec<&str> = op.split(" ").collect();

    let arg: i64 = tokens[1].parse().unwrap();
    match tokens[0] {
        "acc" => Op::Acc(arg),
        "jmp" => Op::Jmp(arg),
        "nop" => Op::Nop(arg),
        _     => unreachable!()
    }
}

fn input() -> Vec<Op> {
    lines().iter()
        .map(|line| parse(line))
        .collect()
}

pub fn main() {
    let code = input();
    let run = exec(&code);
    println!("{}", run.acc());

    let run = find(code);
    println!("{}", run.unwrap().acc());
}

#[cfg(test)]
mod tests {
    use super::*;
    use Op::*;

    #[test]
    fn test_parse() {
        let input = vec![
            ("nop +0", Nop(0)),
            ("acc +1", Acc(1)),
            ("jmp -4", Jmp(-4)),
        ];

        for (s, op) in input {
            assert_eq!(parse(s), op);
        }
    }

    #[test]
    fn test_step() {
        let code = vec![
            Nop(0),
            Acc(1),
            Jmp(-2),
        ];

        let mut run = Run::new(&code);
        assert_eq!(run.op(), 0);
        assert_eq!(run.acc(), 0);

        run.step();
        assert_eq!(run.op(), 1);
        assert_eq!(run.acc(), 0);

        run.step();
        assert_eq!(run.op(), 2);
        assert_eq!(run.acc(), 1);

        run.step();
        assert_eq!(run.op(), 0);
        assert_eq!(run.acc(), 1);
    }
}
