
// byr (Birth Year)
// iyr (Issue Year)
// eyr (Expiration Year)
// hgt (Height)
// hcl (Hair Color)
// ecl (Eye Color)
// pid (Passport ID)
// cid (Country ID)

use std::collections::HashMap;
use crate::utils::lines;

pub const BYR: &str = "byr";
pub const IYR: &str = "iyr";
pub const EYR: &str = "eyr";
pub const HGT: &str = "hgt";
pub const HCL: &str = "hcl";
pub const ECL: &str = "ecl";
pub const PID: &str = "pid";
pub const CID: &str = "cid";

pub type Passport = HashMap<String, String>;

pub fn input() -> Vec<Passport> {
    lines().split(|line| line.is_empty())
        .map(|entry| parse(entry.join(" ")))
        .collect()
}

pub fn parse(line: String) -> Passport {
    line.split(" ")
        .map(|kv| {
            let tokens: Vec<&str> = kv.split(":").collect();
            (tokens[0].to_owned(), tokens[1].to_owned())
        })
        .fold(Passport::new(), |mut p, (k ,v)| {
            p.insert(k, v);
            p
        })
}

pub fn is_valid(passport: &Passport) -> bool {
    passport.contains_key(BYR) &&
        passport.contains_key(IYR) &&
        passport.contains_key(EYR) &&
        passport.contains_key(HGT) &&
        passport.contains_key(HCL) &&
        passport.contains_key(ECL) &&
        passport.contains_key(PID)
}

fn is_valid_year(s: &str, lo: usize, hi: usize) -> bool {
    let all_digits = s.len() == 4 && s.chars().all(|c| c.is_digit(10));
    if !all_digits {
        return false;
    }

    let year: usize = s.parse().unwrap();
    if year < lo || year > hi {
        return false;
    }

    true
}

// byr (Birth Year) - four digits; at least 1920 and at most 2002.
fn is_byr_valid(byr: &str) -> bool {
    is_valid_year(byr, 1920, 2002)
}

// iyr (Issue Year) - four digits; at least 2010 and at most 2020.
fn is_iyr_valid(iyr: &str) -> bool {
    is_valid_year(iyr, 2010, 2020)
}

// eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
fn is_eyr_valid(eyr: &str) -> bool {
    is_valid_year(eyr, 2020, 2030)
}

// hgt (Height) - a number followed by either cm or in:
// If cm, the number must be at least 150 and at most 193.
// If in, the number must be at least 59 and at most 76.
fn is_hgt_valid(hgt: &str) -> bool {
    let len = hgt.len();
    let suffix: String = hgt.chars().skip(len - 2).collect();
    let n: usize = hgt.chars().take(len - 2).collect::<String>().parse().unwrap();
    match suffix.as_ref() {
        "cm" => n >= 150 && n <= 193,
        "in" => n >= 59 && n <= 76,
        _ => false
    }
}

// hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
fn is_hcl_valid(hcl: &str) -> bool {
    hcl.chars().next().unwrap() == '#' &&
        hcl.to_lowercase().chars().skip(1).all(|c| c.is_digit(16))
}

// ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
fn is_ecl_valid(ecl: &str) -> bool {
    ecl == "amb" ||
        ecl == "blu" ||
        ecl == "brn" ||
        ecl == "gry" ||
        ecl == "grn" ||
        ecl == "hzl" ||
        ecl == "oth"
}

// pid (Passport ID) - a nine-digit number, including leading zeroes.
fn is_pid_valid(pid: &str) -> bool {
    pid.len() == 9 && pid.chars().all(|c| c.is_digit(10))
}

// cid (Country ID) - ignored, missing or not.

fn is_really_valid(passport: &Passport) -> bool {
    is_valid(&passport) &&
        is_byr_valid(passport.get(BYR).unwrap()) &&
        is_eyr_valid(passport.get(EYR).unwrap()) &&
        is_iyr_valid(passport.get(IYR).unwrap()) &&
        is_hgt_valid(passport.get(HGT).unwrap()) &&
        is_hcl_valid(passport.get(HCL).unwrap()) &&
        is_ecl_valid(passport.get(ECL).unwrap()) &&
        is_pid_valid(passport.get(PID).unwrap())
}

pub fn main() {
    let passports = input();
    let valid = passports.iter().filter(|p| is_valid(p)).count();
    println!("{}", valid);

    let really_valid = passports.iter()
        .filter(|p| is_really_valid(p))
        .count();
    println!("{}", really_valid);

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let line: String = "ecl:lzr cid:279 pid:192cm hcl:1f7352 iyr:2014 hgt:70cm eyr:1983 byr:2004".to_string();
        let passport = parse(line);

        assert_eq!(passport.len(), 8);
        assert_eq!(passport.get(ECL).unwrap(), "lzr");
        assert_eq!(passport.get(CID).unwrap(), "279");
        assert_eq!(passport.get(PID).unwrap(), "192cm");
        assert_eq!(passport.get(HCL).unwrap(), "1f7352");
        assert_eq!(passport.get(IYR).unwrap(), "2014");
        assert_eq!(passport.get(HGT).unwrap(), "70cm");
        assert_eq!(passport.get(EYR).unwrap(), "1983");
        assert_eq!(passport.get(BYR).unwrap(), "2004");
    }
}
