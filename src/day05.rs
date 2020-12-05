use crate::utils::lines;

#[derive(Debug, Eq, PartialEq)]
enum Half {
    LO,
    HI,
}

#[derive(Debug)]
struct Ticket {
    row: usize,
    col: usize,
}

impl Ticket {
    fn new(row: usize, col: usize) -> Self {
        Ticket {
            row,
            col
        }
    }

    fn id(&self) -> usize {
        self.row * 8 + self.col
    }
}

impl From<usize> for Ticket {
    fn from(id: usize) -> Self {
        let (row, col) = (id / 8, id % 8);
        Self::new(row, col)
    }
}

fn split(lo: usize, hi: usize, half: &Half) -> (usize, usize) {
    assert!(lo <= hi);
    if lo == hi {
        (lo, hi)
    } else {
        let m = lo + (hi - lo) / 2;
        match half {
            Half::LO => (lo, m),
            Half::HI => (m+1, hi)
        }
    }
}

fn find(lo: usize, hi: usize, steps: &Vec<Half>) -> usize {
    let (a, b) = steps.iter()
        .fold((lo, hi), |(lo, hi), half| split(lo, hi, half));
    assert_eq!(a, b);
    a
}

fn parse(line: &str) -> (Vec<Half>, Vec<Half>) {
    let rows = line.chars()
        .filter_map(|c| match c {
            'F' => Some(Half::LO),
            'B' => Some(Half::HI),
            _ => None
        })
        .collect();

    let cols = line.chars()
        .skip_while(|c| *c == 'B' || *c == 'F')
        .filter_map(|c| match c {
            'L' => Some(Half::LO),
            'R' => Some(Half::HI),
            _ => None
        })
        .collect();

    (rows, cols)
}

fn find_missing(sorted: &Vec<usize>) -> usize {
    let (p, n) = sorted.iter().zip(sorted.iter().skip(1))
        .find(|(p, n)| *n - *p > 1)
        .map(|(p, n)| (*p, *n))
        .unwrap();

    assert_eq!(p + 1, n - 1);
    p + 1
}

fn input() -> Vec<(Vec<Half>, Vec<Half>)> {
    lines().into_iter()
        .map(|line| parse(&line))
        .collect()
}

pub fn main() {
    let tickets = input();
    let mut ids: Vec<usize> = tickets.iter()
        .map(|(rows, cols)| {
            let row = find(0, 127, rows);
            let col = find(0, 7, cols);
            Ticket::new(row, col)
        })
        .map(|tkt| tkt.id())
        .collect();

    let max_id = ids.iter().max().unwrap();
    println!("{}", *max_id);

    ids.sort();
    let missing_id = find_missing(&ids);
    println!("{}", missing_id);
}

#[cfg(test)]
mod tests {
    use super::*;
    use Half::*;

    #[test]
    fn test_split() {
        assert_eq!(split(0, 127, &LO), (0, 63));
        assert_eq!(split(0, 127, &HI), (64, 127));

        assert_eq!(split(0, 63, &LO), (0, 31));
        assert_eq!(split(0, 63, &HI), (32, 63));

        assert_eq!(split(32, 64, &LO), (32, 48));
        assert_eq!(split(32, 64, &HI), (49, 64));
    }

    #[test]
    fn test_find() {
        // FBFBBFF
        let steps = vec![LO, HI, LO, HI, HI, LO, LO];
        assert_eq!(find(0, 127, &steps), 44);
    }

    #[test]
    fn test_parse() {
        let input = "FBFBBFFRLR";
        let (rows, cols) = parse(input);
        assert_eq!(rows, vec![LO, HI, LO, HI, HI, LO, LO]);
        assert_eq!(cols, vec![HI, LO, HI]);
    }
}