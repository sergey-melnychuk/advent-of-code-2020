use crate::utils::lines;
use crate::day01::find2;

fn input() -> Vec<i64> {
    lines().into_iter()
        .map(|line| line.parse().unwrap())
        .collect()
}

fn find(numbers: &[i64], k: usize) -> Option<i64> {
    numbers.windows(k + 1)
        .find(|window| {
            let n = *window.last().unwrap();
            let mut preamble: Vec<i64> = window.iter()
                .take(k)
                .map(|x| *x)
                .collect();
            preamble.sort();
            find2(&preamble, n).is_none()
        })
        .map(|window| *window.last().unwrap())
}

// Find a contiguous set of at least two numbers in your list which sum to the target.
fn find_subsum(numbers: &[i64], sum: i64) -> Option<&[i64]> {
    // Check if sub-slice starting at index `at` sums up to target sum, and return length if yes.
    fn check_at(numbers: &[i64], sum: i64, at: usize) -> Option<usize> {
        let mut remaining = sum;
        for i in at..numbers.len() {
            let x = numbers[i];
            if x < remaining {
                remaining -= x;
            } else if x == remaining {
                return Some(i - at + 1);
            } else if x > remaining {
                return None;
            }
        }
        None
    }

    for at in 0..numbers.len()-1 {
        if let Some(n) = check_at(numbers, sum, at) {
            return Some(&numbers[at..(at + n)]);
        }
    }

    None
}

pub fn main() {
    const K: usize = 25;

    let input = input();
    let found = find(&input, K).unwrap();
    println!("{}", found);

    let sum = find_subsum(&input, found)
        .map(|slice| {
            let min = *slice.iter().min().unwrap();
            let max = *slice.iter().max().unwrap();
            min + max
        })
        .unwrap();
    println!("{}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input: Vec<i64> = vec![35,20,15,25,47,40,62,55,65,
                                   95,102,117,150,182, // does not sum up to 127
                                   127,
                                   219,299,277,309,576];
        assert_eq!(find(&input, 5), Some(127));
    }

    #[test]
    fn test_subsum() {
        let input: Vec<i64> = vec![35,20,
                                   15,25,47,40, // sums up to 127
                                   62,55,65,95,102,117,150,182,127,219,299,277,309,576];
        assert_eq!(find_subsum(&input, 127), Some(&input[2..6]));
    }
}