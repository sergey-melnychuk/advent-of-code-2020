use regex::Regex;

use crate::utils::lines;

#[derive(Eq, PartialEq, Debug)]
pub(crate) struct Record {
    pub(crate) lo: usize,
    pub(crate) hi: usize,
    pub(crate) chr: char,
    pub(crate) pwd: String,
}

impl Record {
    fn is_valid(&self) -> bool {
        let count = self.pwd.chars().filter(|c| *c == self.chr).count();
        (count >= self.lo) && (count <= self.hi)
    }
}

pub(crate) fn parse(line: String) -> Result<Record, ()> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d+)-(\d+)\s(\w): (\w+)").unwrap();
    }
    let cap = RE.captures(&line).unwrap();
    let lo: usize = cap.get(1).unwrap().as_str().parse().unwrap();
    let hi: usize = cap.get(2).unwrap().as_str().parse().unwrap();
    let chr = cap.get(3).unwrap().as_str().chars().next().unwrap();
    let pwd = cap.get(4).unwrap().as_str().to_string();
    Ok(Record { lo, hi, chr, pwd })
}

pub(crate) fn input() -> Vec<Record> {
    lines().into_iter()
        .map(|line| parse(line).unwrap())
        .collect()
}

fn is_valid(record: &Record) -> bool {
    record.pwd.chars().enumerate().into_iter()
        .map(|(i, c)| (i+1, c))
        .filter(|(i, _)| (*i == record.lo) || (*i == record.hi))
        .filter(|(_, c)| *c == record.chr)
        .count() == 1
}

pub fn main() {
    let input = input();
    let valid = input.iter().filter(|r| r.is_valid()).count();
    println!("{}", valid);

    let valid = input.iter().filter(|r| is_valid(r)).count();
    println!("{}", valid);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let line = "10-12 l: kllswlmlglps".to_string();
        let expected = Record { lo: 10, hi: 12, chr: 'l', pwd: "kllswlmlglps".to_string() };
        assert_eq!(parse(line).unwrap(), expected);
    }

    // 1-3 a: abcde is valid: position 1 contains a and position 3 does not.
    // 1-3 b: cdefg is invalid: neither position 1 nor position 3 contains b.
    // 2-9 c: ccccccccc is invalid: both position 2 and position 9 contain c.
    #[test]
    fn test_is_valid() {
        let cases = vec![
            ("1-3 a: abcde", true),
            ("1-3 b: cdefg", false),
            ("2-9 c: ccccccccc", false)
        ];

        for (line, valid) in cases {
            println!("{}", line);
            let rec = parse(line.to_string()).unwrap();
            assert_eq!(is_valid(&rec), valid);
        }
    }

}