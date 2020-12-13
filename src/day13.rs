use crate::utils::{lines, primes};

fn input() -> (u64, Vec<(u64, u64)>) {
    let lines = lines();
    let mut it = lines.iter();
    let estimate: u64 = it.next().unwrap().parse().unwrap();
    let timetable = it.next().unwrap()
        .split(",").into_iter().enumerate()
        .filter(|(_, x)| *x != "x")
        .map(|(i, t)| (i as u64, t.parse().unwrap()))
        .collect();

    (estimate, timetable)
}

fn next(since: u64, period: u64) -> u64 {
    since + (period - (since % period))
}

fn find_closest_departure(est: u64, table: &Vec<u64>) -> (u64, u64) {
    table.iter()
        .map(|id| (next(est, *id), *id))
        .min_by_key(|(n, _)| *n)
        .map(|(n, id)| (n - est, id))
        .unwrap()
}

fn find_pattern(table: &Vec<(u64, u64)>) -> u64 {
    0
}

pub fn main() {
    let (est, table) = input();

    let ids: Vec<u64> = table.iter().map(|(_, x)| *x).collect();
    let (w, id) = find_closest_departure(est, &ids);
    println!("{}", w * id);

    // let k = ids[0];
    // let mul = table.iter()
    //     .map(|(_, id)| *id)
    //     .fold(1, |acc, x| acc * x) / k;
    // println!("vec={:?}\nlen={} mul={} log2={}", table, table.len(), mul, (mul as f64).log2().ceil());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_closest_departure() {
        let est = 939;
        let vec = vec![7, 13, 59, 31, 19];
        assert_eq!(find_closest_departure(est, &vec), (5, 59));
    }

    #[test]
    fn test_find_pattern1() {
        let cases: Vec<(Vec<(u64, u64)>, u64)> = vec![
            // 7,13,x,x,59,x,31,19 -> 1068788
            (vec![(0, 7), (1, 13), (4, 59), (6, 31), (7, 19)], 1068788),

            // 17,x,13,19 -> 3417
            // 67,7,59,61 -> 754018
            // 67,x,7,59,61 -> 779210
            // 67,7,x,59,61 -> 1261476
            // 1789,37,47,1889 -> 1202161486
        ];

        for (vec, result) in cases {
            assert_eq!(find_pattern(&vec), result, "{:?}", vec);
        }
    }
}
