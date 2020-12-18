use std::io::{self, BufRead};

pub fn lines() -> Vec<String> {
    let stdin = io::stdin();
    stdin.lock().lines()
        .map(|line| line.unwrap())
        .collect()
}

#[allow(dead_code)]
pub fn primes(mut x: u64) -> Vec<u64> {
    let mut result = Vec::new();
    while x > 1 {
        let r = (x as f64).sqrt().ceil() as u64;
        for d in 2..r {
            if x > d && x % d == 0 {
                result.push(d);
                x /= d;
            }
        }
        result.push(x);
        break;
    }
    result
}

#[allow(dead_code)]
fn head<T>(vec: Vec<T>) -> (T, Vec<T>) {
    let mut it = vec.into_iter();
    let a = it.next().unwrap();
    let rem = it.collect();
    (a, rem)
}

#[allow(dead_code)]
fn take2<T>(vec: Vec<T>) -> ((T, T), Vec<T>) {
    let mut it = vec.into_iter();
    let a = it.next().unwrap();
    let b = it.next().unwrap();
    let rem = it.collect();
    ((a, b), rem)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primes() {
        let vec: Vec<u64> = vec![13, 17, 19, 23, 29, 37, 41, 521, 661];
        let mul: u64 = vec.iter().fold(1, |acc, x| acc * *x);
        assert_eq!(primes(mul), vec)
    }
}
