use crate::day01::part1::{input, find2};

/// Find three numbers from input slice that add up to provided sum value.
fn find3(input: &[i64], sum: i64) -> Option<(i64, i64, i64)> {
    let n = input.len();
    for i in 0..n {
        let a = input[i];
        let remaining = exclude(input, i);

        if let Some((b, c)) = find2(&remaining, sum - a) {
            return Some((a, b, c));
        }
    }

    None
}

fn exclude<T: Clone + 'static>(slice: &[T], index: usize) -> Vec<T> {
    let before = &slice[..index];
    let after = &slice[(index + 1)..];
    let mut result = Vec::with_capacity(slice.len() - 1);
    result.extend_from_slice(before);
    result.extend_from_slice(after);
    result
}

fn solve(input: Vec<i64>) -> i64 {
    let (a, b, c) = find3(&input, 2020).unwrap();
    a * b * c
}

pub fn main() {
    let answer = solve(input());
    println!("{}", answer);
}
