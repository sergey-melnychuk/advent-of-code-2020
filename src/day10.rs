use crate::utils::lines;

fn input() -> Vec<u64> {
    let mut numbers: Vec<u64> = lines().into_iter()
        .map(|line| line.parse().unwrap())
        .collect();
    numbers.sort();
    numbers
}

fn collect(values: &[u64], mut init: u64) -> (u64, u64, u64) {
    let mut acc: [u64; 3] = [0; 3];

    for i in 0..values.len() {
        let x = values[i];
        let d = x - init;
        acc[(d as usize) - 1] += 1;
        init = values[i];
    }

    acc[2] += 1; // built-it adapter has +3 difference
    (acc[0], acc[1], acc[2])
}

fn count(init: u64, at: usize, values: &[u64], cache: &mut [u64]) -> u64 {
    if at >= values.len()-1 {
        return 1;
    }

    if cache[at] > 0 {
        return cache[at];
    }

    let mut acc: u64 = 0;
    let mut i = at;
    while i < values.len() && values[i] - init <= 3 {
        acc += count(values[i], i+1, values, cache);
        i += 1;
    }

    cache[at] = acc;
    acc
}

pub fn main() {
    let input = input();

    let (a, _, b) = collect(&input, 0);
    println!("{}", a * b);

    let n = count(0, 0, &input, &mut vec![0; input.len()]);
    println!("{}", n);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collect_small() {
        let mut numbers = vec![16,10,15,5,1,11,7,19,6,12,4];
        numbers.sort();
        assert_eq!(collect(&numbers, 0), (7, 0, 5));
    }
    
    #[test]
    fn test_collect_large() {
        let mut numbers = vec![28,33,18,42,31,14,46,20,48,47,24,23,49,45,19,38,39,11,1,32,25,35,8,17,7,9,4,2,34,10,3];
        numbers.sort();
        assert_eq!(collect(&numbers, 0), (22, 0, 10));
    }

    #[test]
    fn test_count_tiny() {
        let numbers = vec![1, 2];
        assert_eq!(count(0, 0, &numbers, &mut vec![0; numbers.len()]), 2);
    }

    #[test]
    fn test_count_small() {
        let mut numbers = vec![16,10,15,5,1,11,7,19,6,12,4];
        numbers.sort();
        assert_eq!(count(0, 0, &numbers, &mut vec![0; numbers.len()]), 8);
    }

    #[test]
    fn test_count_large() {
        let mut numbers = vec![28,33,18,42,31,14,46,20,48,47,24,23,49,45,19,38,39,11,1,32,25,35,8,17,7,9,4,2,34,10,3];
        numbers.sort();
        assert_eq!(count(0, 0, &numbers, &mut vec![0; numbers.len()]), 19208);
    }

}
