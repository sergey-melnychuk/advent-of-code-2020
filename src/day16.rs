use crate::utils::lines;
use std::collections::HashSet;

type Rule = (String, u64, u64, u64, u64);

type Ticket = Vec<u64>;

fn parse_rule(line: &str) -> Rule {
    let mut split1 = line.split(": ");
    let name = split1.next().unwrap().to_string();
    let mut split2 = split1.next().unwrap().split(" or ");
    let mut split3 = split2.next().unwrap().split("-");
    let mut split4 = split2.next().unwrap().split("-");
    let a: u64 = split3.next().unwrap().parse().unwrap();
    let b: u64 = split3.next().unwrap().parse().unwrap();
    let c: u64 = split4.next().unwrap().parse().unwrap();
    let d: u64 = split4.next().unwrap().parse().unwrap();
    (name, a, b, c, d)
}

fn parse_ticket(line: &str) -> Ticket {
    line.split(",")
        .map(|number| number.parse().unwrap())
        .collect()
}

fn parse_input(lines: Vec<String>) -> (Vec<Rule>, Ticket, Vec<Ticket>) {
    let mut split = lines
        .split(|line| line.is_empty());

    let rules = split.next().unwrap()
        .into_iter()
        .map(|line| parse_rule(&line))
        .collect();

    let ticket = split.next().unwrap()
        .into_iter()
        .skip(1)
        .next()
        .unwrap();

    let tickets = split.next().unwrap()
        .into_iter()
        .skip(1)
        .map(|line| parse_ticket(line))
        .collect();

    (rules, parse_ticket(ticket), tickets)
}

fn is_valid(rule: &Rule, val: u64) -> bool {
    let (_, a, b, c, d) = rule.to_owned();
    (val >= a && val <= b) || (val >= c && val <= d)
}

fn validate(rules: &Vec<Rule>, ticket: &Ticket) -> Vec<u64> {
    ticket.iter()
        .filter(|n| !rules.iter().any(|r| is_valid(r, **n)))
        .map(|n| *n)
        .collect()
}

fn pick(rules: &Vec<Rule>, ticket: &Ticket, index: usize) -> HashSet<usize> {
    (0..rules.len())
        .into_iter()
        .filter(|i| {
            let rule = rules.get(*i).unwrap();
            let num = ticket[index];
            is_valid(rule, num)
        })
        .collect()
}

fn picks(rules: &Vec<Rule>, tickets: &Vec<Ticket>, index: usize) -> Vec<HashSet<usize>> {
    tickets.iter()
        .map(|t| pick(rules, t, index))
        .collect()
}

fn reduce(rules: &Vec<Rule>, tickets: &Vec<Ticket>) -> Vec<HashSet<usize>> {
    let n = rules.len();
    (0..n)
        .into_iter()
        .map(|i| {
            let picks = picks(rules, tickets, i);
            let acc: HashSet<usize> = (0..n).into_iter().collect();
            picks.iter()
                .fold(acc, |acc, set| {
                    acc.intersection(&set).into_iter().cloned().collect()
                })
        })
        .collect()
}

fn resolve(reduced: Vec<HashSet<usize>>) -> Vec<usize> {
    fn helper(reduced: Vec<HashSet<usize>>, mut acc: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
        let found = reduced.iter().enumerate()
            .find(|(_, set)| set.len() == 1)
            .map(|(i, set)| (i, *set.iter().next().unwrap()))
            .to_owned();
        if let Some((idx, one)) = found {
            let next = reduced.into_iter()
                .map(|mut set| {
                    set.remove(&one);
                    set
                })
                .collect();
            acc.push((one, idx)); // NOTE: swap (idx, one) to (one, idx) to build proper index
            helper(next, acc)
        } else {
            acc
        }
    }

    fn collect(mut acc: Vec<(usize, usize)>) -> Vec<usize> {
        acc.sort_by_key(|(i, _)| *i);
        acc.into_iter()
            .map(|(_, x)| x)
            .collect()
    }

    collect(helper(reduced, Vec::new()))
}

pub fn main() {
    let (rules, own, tickets) = parse_input(lines());

    let sum: u64 = tickets.iter()
        .map(|ticket| validate(&rules, ticket).into_iter().sum::<u64>())
        .sum();
    println!("{}", sum);

    let valid: Vec<Ticket> = tickets.into_iter()
        .filter(|ticket| validate(&rules, ticket).is_empty())
        .collect();

    let reduced = reduce(&rules, &valid);
    let resolved = resolve(reduced);
    let values: Vec<u64> = rules.iter().enumerate()
        .filter(|(_, r)| r.0.starts_with("departure"))
        .map(|(i, _)| i)
        .map(|i| resolved[i])
        .map(|i| own[i])
        .collect();
    let mul = values.iter().fold(1u64, |acc, x| acc * x);
    println!("{}", mul);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ticket() {
        let line = "row: 45-461 or 467-955";
        assert_eq!(parse_rule(line), ("row".to_string(), 45, 461, 467, 955));
    }

    #[test]
    fn test_parse_input() {
        let lines = vec![
            "wagon: 38-111 or 127-963",
            "zone: 28-226 or 234-951",
            "",
            "your ticket:",
            "107,109,163,127,167,157,139,67,131,59,151,53,73,83,61,89,71,149,79,137",
            "",
            "nearby tickets:",
            "910,308,590,919,735,895,709,238,269,618,606,437,541,938,912,811,330,80,283,210",
            "256,680,352,64,818,63,168,78,564,179,859,908,724,306,312,719,624,866,929,451",
        ];

        let (rules, ticket, tickets) =
            parse_input(lines.into_iter().map(|s| s.to_string()).collect());

        assert_eq!(rules, vec![
            ("wagon".to_string(), 38, 111, 127, 963),
            ("zone".to_string(), 28, 226, 234, 951),
        ]);
        assert_eq!(ticket,
                   vec![107,109,163,127,167,157,139,67,131,59,151,53,73,83,61,89,71,149,79,137]);
        assert_eq!(tickets, vec![
            vec![910,308,590,919,735,895,709,238,269,618,606,437,541,938,912,811,330,80,283,210],
            vec![256,680,352,64,818,63,168,78,564,179,859,908,724,306,312,719,624,866,929,451],
        ]);
    }

    #[test]
    fn test_validate() {
        let rules = vec![
            ("a".to_string(),  1,  3,  5,  7),
            ("b".to_string(),  6, 11, 33, 44),
            ("c".to_string(), 13, 40, 45, 50),
        ];

        assert_eq!(validate(&rules, &vec![ 7, 3, 47]), vec![]);
        assert_eq!(validate(&rules, &vec![40, 4, 50]), vec![4]);
        assert_eq!(validate(&rules, &vec![55, 2, 20]), vec![55]);
        assert_eq!(validate(&rules, &vec![38, 6, 12]), vec![12]);
    }
}
