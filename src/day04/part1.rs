
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

pub fn main() {
    let passports = input();
    let valid = passports.iter().filter(|p| is_valid(p)).count();
    println!("{}", valid);
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
