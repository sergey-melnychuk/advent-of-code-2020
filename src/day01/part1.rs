use crate::utils::lines;

pub fn input() -> Vec<i64> {
    lines().into_iter()
        .map(|line| line.parse().unwrap())
        .collect()
}

/// Find pair of numbers from input slice that add up to provided sum value.
pub fn find2(input: &[i64], sum: i64) -> Option<(i64, i64)> {
    let sorted = {
        let mut copy = input.to_vec();
        copy.sort();
        copy
    };

    let mut lo: usize = 0;
    let mut hi: usize = sorted.len() - 1;
    loop {
        if lo >= hi {
            break;
        }

        let a = sorted[lo];
        let b = sorted[hi];
        let added = a + b;
        if added == sum {
            return Some((a, b));
        } else if added < sum {
            lo += 1;
        } else {
            hi -= 1;
        }
    }

    None
}

fn solve(input: Vec<i64>) -> i64 {
    let (a, b) = find2(&input, 2020).unwrap();
    a * b
}

pub fn main() {
    let answer = solve(input());
    println!("{}", answer);
}
