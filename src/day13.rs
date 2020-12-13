use crate::utils::lines;

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
    let (_, id) = table[0];
    table.iter().skip(1)
        .fold((0, id), |(time, one), (d, two)| {
            let next = find_pair(time, one, *two, *d);
            let k = one * *two;
            (next, k)
        })
        .0
}

fn find_pair(since: u64, one: u64, two: u64, d: u64) -> u64 {
    let mut time: u64 = since;
    loop {
        let next = next(time, two) - (d % two);
        if time == next {
            return time;
        }
        time += one;
    }
}

pub fn main() {
    let (est, table) = input();

    let ids: Vec<u64> = table.iter().map(|(_, x)| *x).collect();
    let (w, id) = find_closest_departure(est, &ids);
    println!("{}", w * id);

    let t = find_pattern(&table);
    println!("{}", t);
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
    fn test_find_pair1() {
        assert_eq!(find_pair(1068774, 7, 13, 1), 1068781);

        assert_eq!(find_pair(1068781, 7 * 13, 59, 4), 1068781);

        assert_eq!(find_pair(1068781, 7 * 13 * 59, 31, 6), 1068781);
    }

    #[test]
    fn test_find_pair2() {
        // 17,x,13,19 -> 3417
        let t = find_pair(0, 17, 13, 2);
        let v = find_pair(t, 17 * 13, 19, 3);
        assert_eq!(v, 3417);
    }

    #[test]
    fn test_find_pair3() {
        // 1789,37,47,1889 first occurs at timestamp 1202161486
        let a = find_pair(0, 1789, 37, 1);
        let b = find_pair(a, 1789 * 37, 47, 2);
        let c = find_pair(b, 1789 * 37 * 47, 1889, 3);
        assert_eq!(c, 1202161486);
    }

    #[test]
    fn test_find_pair4() {
        // 67,7,59,61 first occurs at timestamp 754018
        let a = find_pair(0, 67, 7, 1);
        let b = find_pair(a, 67 * 7, 59, 2);
        let c = find_pair(b, 67 * 7 * 59, 61, 3);
        assert_eq!(c, 754018);
    }

    #[test]
    fn test_find_pattern() {
        let cases: Vec<(Vec<(u64, u64)>, u64)> = vec![
            // 7,13,x,x,59,x,31,19 -> 1068788
            (vec![(0, 7), (1, 13), (4, 59), (6, 31), (7, 19)], 1068781),

            // 17,x,13,19 -> 3417
            (vec![(0, 17), (2, 13), (3, 19)], 3417),

            // 67,7,59,61 -> 754018
            (vec![(0, 67), (1, 7), (2, 59), (3, 61)], 754018),

            // 67,x,7,59,61 -> 779210
            (vec![(0, 67), (2, 7), (3, 59), (4, 61)], 779210),

            // 67,7,x,59,61 -> 1261476
            (vec![(0, 67), (1, 7), (3, 59), (4, 61)], 1261476),

            // 1789,37,47,1889 -> 1202161486
            (vec![(0, 1789), (1, 37), (2, 47), (3, 1889)], 1202161486),
        ];

        for (vec, result) in cases {
            assert_eq!(find_pattern(&vec), result, "{:?}", vec);
        }
    }
}
