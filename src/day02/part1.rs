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

pub fn main() {
    let input = input();
    let valid = input.iter().filter(|r| r.is_valid()).count();
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

}