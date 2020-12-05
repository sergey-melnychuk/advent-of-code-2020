use crate::utils::lines;

pub fn input() -> Vec<i64> {
    lines().into_iter()
        .map(|line| line.parse().unwrap())
        .collect()
}

/// Find pair of numbers from input slice that add up to provided sum value.
pub fn find2(sorted: &[i64], sum: i64) -> Option<(i64, i64)> {
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

fn solve2(input: &Vec<i64>) -> i64 {
    let (a, b) = find2(input, 2020).unwrap();
    a * b
}

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

/// Return new Vector with element at given index missing (excluded)
fn exclude<T: Clone + 'static>(slice: &[T], index: usize) -> Vec<T> {
    slice.iter().enumerate()
        .filter(|(i, _)| *i != index)
        .map(|(_, e)| e.clone())
        .collect()
}

fn solve3(input: &Vec<i64>) -> i64 {
    let (a, b, c) = find3(input, 2020).unwrap();
    a * b * c
}

pub fn main() {
    let input = {
        let mut xs = input();
        xs.sort();
        xs
    };

    let answer = solve2(&input);
    println!("{}", answer);

    let answer = solve3(&input);
    println!("{}", answer);
}
