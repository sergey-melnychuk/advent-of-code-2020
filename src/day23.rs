use crate::utils::lines;

pub fn main() {
    let cups = input(lines());

    let state = (0..100).fold(cups.clone(), |cups, _| step(cups));
    println!("{}", to_str(align(state)));


    let n = 10_000_000;
    let cups = unfold(cups);

    let mut list = as_list(&cups);
    let mut cup = cups[0];

    for _ in 0..n {
        step2(&mut list, &mut cup);
    }

    let a = list[1];
    let b = list[a];
    println!("{}", a * b);
}

fn parse(line: &str) -> Vec<usize> {
    line.chars().into_iter()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect()
}

fn input(lines: Vec<String>) -> Vec<usize> {
    parse(lines.iter().next().unwrap())
}

fn step(cups: Vec<usize>) -> Vec<usize> {
    let (cups, next) = cut(1, 3, cups);
    let dst = find(&cups, &next);
    let cups = put(cups, next, dst);
    shift(1, cups)
}

fn shift(n: usize, cups: Vec<usize>) -> Vec<usize> {
    let mut head = cups.iter().skip(n).cloned().collect::<Vec<_>>();
    let mut tail = cups.iter().take(n).cloned().collect::<Vec<_>>();
    head.append(&mut tail);
    head
}

fn cut(skip: usize, take: usize, cups: Vec<usize>) -> (Vec<usize>, Vec<usize>) {
    let next = cups.iter().skip(skip).take(take).cloned().collect();
    let cups = cups.into_iter()
        .enumerate()
        .filter_map(|(i, x)| {
            if i < skip || i >= skip + take {
                Some(x)
            } else {
                None
            }
        })
        .collect();

    (cups, next)
}

fn put(cups: Vec<usize>, next: Vec<usize>, dst: usize) -> Vec<usize> {
    let mut out = Vec::with_capacity(cups.len() + next.len());

    cups.iter()
        .take(dst + 1)
        .cloned()
        .for_each(|x| out.push(x));

    next.into_iter()
        .for_each(|x| out.push(x));

    cups.iter()
        .skip(dst + 1)
        .cloned()
        .for_each(|x| out.push(x));

    out
}

fn find(cups: &Vec<usize>, next: &Vec<usize>) -> usize {
    let mut x = cups.get(0).unwrap().clone();
    x -= 1;
    while next.contains(&x) {
        x -= 1;
    }

    if !cups.contains(&x) {
        x = cups.iter().max().cloned().unwrap();
    }

    cups.iter().take_while(|y| **y != x).count()

}

fn align(cups: Vec<usize>) -> Vec<usize> {
    let one = cups.iter()
        .take_while(|x| **x != 1)
        .count();
    shift(one, cups)
}

fn to_str(cups: Vec<usize>) -> String {
    cups.into_iter()
        .skip(1)
        .map(|x| ((x as u8 + '0' as u8) as char))
        .collect()
}

fn unfold(cups: Vec<usize>) -> Vec<usize> {
    let mut result = Vec::with_capacity(1_000_000);

    let max = cups.iter().max().cloned().unwrap();
    cups.iter()
        .cloned()
        .for_each(|x| result.push(x));
    (0..(result.capacity() - cups.len()))
        .for_each(|x| result.push(max + (x as usize) + 1));

    assert_eq!(result.len(), 1_000_000);
    result
}

// list[label] = next label clockwise
fn as_list(cups: &[usize]) -> Vec<usize> {
    let mut list = vec![0; cups.len() + 1];
    let n = cups.len();

    for i in 0..n-1 {
        list[cups[i]] = cups[i+1];
    }

    list[cups[n-1]] = cups[0];

    list
}

fn step2(next: &mut [usize], cup: &mut usize) {
    let n = next.len();
    let mut dst = if *cup == 1 {
        n - 1
    } else {
        *cup - 1
    };

    let cut1 = next[*cup];
    let cut2 = next[cut1];
    let cut3 = next[cut2];

    let cut = vec![cut1, cut2, cut3];
    while cut.contains(&dst) {
        if dst == 1 {
            dst = n-1;
        } else {
            dst -= 1;
        }
    }

    next[*cup] = next[cut3];
    next[cut3] = next[dst];
    next[dst] = cut1;

    *cup = next[*cup];
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(parse("853192647"), vec![8, 5, 3, 1, 9, 2, 6, 4, 7]);
    }

    #[test]
    fn test_cut() {
        let cups = vec![3,8,9,1,2,5,4,6,7];
        assert_eq!(cut(1, 3, cups), (vec![3,2,5,4,6,7], vec![8,9,1]));
    }

    #[test]
    fn test_find() {
        assert_eq!(find(&vec![3,2,5,4,6,7], &vec![8,9,1]), 1);
    }

    #[test]
    fn test_shift() {
        assert_eq!(shift(1, vec![3,2,5,4,6,7]), vec![2,5,4,6,7,3]);
    }

    #[test]
    fn test_put() {
        assert_eq!(put(vec![3,2,5,4,6,7], vec![8,9,1], 1), vec![3,2,8,9,1,5,4,6,7]);
    }

    #[test]
    fn test_step() {
        let cups = vec![3,8,9,1,2,5,4,6,7];
        assert_eq!(step(cups), vec![2,8,9,1,5,4,6,7,3]);
    }

    #[test]
    fn test_10() {
        let cups = vec![3,8,9,1,2,5,4,6,7];

        let cups = (0..10).into_iter()
            .fold(cups, |cups, _| step(cups));

        assert_eq!(to_str(align(cups)), "92658374".to_string());
    }

    #[test]
    #[ignore] // takes <2s in release, but too slow in debug
    fn test_part2() {
        let cups = unfold(vec![3,8,9,1,2,5,4,6,7]);

        let mut list = as_list(&cups);
        let mut cup = cups[0];

        for _ in 0..10000000 {
            step2(&mut list, &mut cup);
        }

        let a = list[1];
        let b = list[a];
        assert_eq!(a, 934001);
        assert_eq!(b, 159792);
    }
}
