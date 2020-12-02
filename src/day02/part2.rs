use crate::day02::part1::{input, Record};

fn is_valid(record: &Record) -> bool {
    record.pwd.chars().enumerate().into_iter()
        .map(|(i, c)| (i+1, c))
        .filter(|(i, _)| (*i == record.lo) || (*i == record.hi))
        .filter(|(_, c)| *c == record.chr)
        .count() == 1
}

pub fn main() {
    let input = input();
    let valid = input.iter().filter(|r| is_valid(r)).count();
    println!("{}", valid);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::day02::part1::parse;

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