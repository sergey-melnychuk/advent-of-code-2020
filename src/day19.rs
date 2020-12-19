use crate::utils::lines;
use std::collections::{HashMap, HashSet};

type Id = usize;

#[derive(Debug, Clone, Eq, PartialEq)]
enum Rule {
    Char(char),
    Seq(Vec<Id>),
    Or(Vec<Rule>),
}

fn parse(line: &str) -> (Id, Rule) {
    fn seq(s: &str) -> Vec<Id> {
        s.split(" ")
            .into_iter()
            .map(|id| id.parse::<Id>().unwrap())
            .collect()
    }

    let mut split = line.split(": ");
    let id: Id = split.next().unwrap().parse().unwrap();

    let rest = split.next().unwrap();
    let rule = if rest.contains('\"') {
        let c = rest.chars().skip(1).next().unwrap();
        Rule::Char(c)
    } else if rest.contains('|') {
        let mut it = rest.split(" | ");
        Rule::Or(vec![
            Rule::Seq(seq(it.next().unwrap())),
            Rule::Seq(seq(it.next().unwrap())),
        ])
    } else {
        Rule::Seq(seq(rest))
    };

    (id, rule)
}

fn input(lines: &Vec<String>) -> (HashMap<Id, Rule>, Vec<String>) {
    let mut it = lines.split(|line| line.is_empty());

    let rules = it.next().unwrap()
        .into_iter()
        .map(|s| parse(s))
        .fold(HashMap::new(), |mut map, (id, rule)| {
            map.insert(id, rule);
            map
        });

    let remaining = it.next().unwrap()
        .into_iter()
        .cloned()
        .collect();

    (rules, remaining)
}

fn build1(rule: &Rule, map: &HashMap<Id, Rule>) -> Vec<String> {
    let mut acc: Vec<String> = Vec::new();
    match rule {
        Rule::Char(c) => {
            acc.push(format!("{}", c));
        },
        Rule::Seq(ids) => {
            let vec = ids.into_iter()
                .map(|id| map.get(id).unwrap())
                .map(|rule| build1(rule, map))
                .collect::<Vec<_>>();

            if vec.len() == 1 {
                for s in vec.into_iter().next().unwrap() {
                    acc.push(s);
                }
            } else if vec.len() == 2 {
                let xs = vec[0].to_owned();
                let ys = vec[1].to_owned();
                for x in xs.iter() {
                    for y in ys.iter() {
                        let s = x.clone() + y;
                        acc.push(s);
                    }
                }
            }
        },
        Rule::Or(rules) => {
            rules.into_iter()
                .flat_map(|rule| build1(rule, map))
                .for_each(|s| acc.push(s));
        }
    }
    acc
}

#[allow(dead_code)]
fn apply(line: String, rule: &Rule, map: &HashMap<Id, Rule>) -> Option<String> {
    if line.is_empty() {
        return None;
    }
    match rule {
        Rule::Char(c) => {
            if line.chars().next().unwrap() == *c {
                Some(line.chars().skip(1).collect::<String>())
            } else {
                None
            }
        }
        Rule::Seq(ids) => {
            ids.iter()
                .fold(Some(line), |opt, id| {
                    opt.into_iter()
                        .flat_map(|s| {
                            let rule = map.get(id).unwrap();
                            apply(s, rule, map)
                        })
                        .next()
                })
        }
        Rule::Or(rules) => {
            let mut it = rules.into_iter();
            let one = it.next().unwrap();
            let two = it.next().unwrap();
            apply(line.clone(), one, map)
                .or_else(|| apply(line, two, map))
        }
    }
}

#[allow(dead_code)]
fn verify(line: &str, id: Id, map: &HashMap<Id, Rule>) -> bool {
    let rule = map.get(&id).unwrap();
    apply(line.to_string(), rule, map)
        .map(|s| s.is_empty())
        .unwrap_or_default()
}

pub fn main() {
    let lines = lines();
    let (rules, inputs) = input(&lines);

    let r0 = rules.get(&0).unwrap();
    let all = build1(r0, &rules)
        .into_iter()
        .collect::<HashSet<_>>();

    let n = inputs.iter()
        .filter(|s| all.contains(*s))
        .count();
    println!("{}", n); // 132

    let r31 = rules.get(&31).unwrap();
    let v31 = build1(r31, &rules).into_iter().collect::<HashSet<_>>();

    let r42 = rules.get(&42).unwrap();
    let v42 = build1(r42, &rules).into_iter().collect::<HashSet<_>>();

    let n = inputs.iter()
        .filter(|s| {
            let n = s.len();
            let a = &s[0..8];
            let b = &s[8..16];
            let z = &s[n-8..];
            v42.contains(a) && v42.contains(b) && v31.contains(z)
        })
        .map(|s| {
            let n = s.len();
            s[16..(n-8)].to_string()
        })
        .filter(|s| {
            let mut z: &str = s;
            let mut n42: usize = 0;
            while z.len() > 0 {
                let t = &z[0..8];
                if v42.contains(t) {
                    z = &z[8..];
                    n42 += 1;
                } else {
                    break;
                }
            }

            let mut n31: usize = 0;
            while z.len() > 0 {
                let t = &z[0..8];
                if v31.contains(t) {
                    z = &z[8..];
                    n31 += 1;
                } else {
                    break;
                }
            }

            z.is_empty() && n42 >= n31
        })
        .count();
    println!("{}", n); // 306
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let cases = vec![
            ("12: \"a\"", (12, Rule::Char('a'))),
            ("56: 17 12 | 129 7", (56, Rule::Or(vec![
                    Rule::Seq(vec![17, 12]),
                    Rule::Seq(vec![129, 7]),
                ]))
            ),
            ("21: 12 12", (21, Rule::Seq(vec![12, 12]))),
        ];

        for (s, (id, rule)) in cases {
            assert_eq!(parse(s), (id, rule), "{}", s)
        }
    }

    #[test]
    fn test_verify() {
        let lines = vec![
            "0: 4 1 5",
            "1: 2 3 | 3 2",
            "2: 4 4 | 5 5",
            "3: 4 5 | 5 4",
            "4: \"a\"",
            "5: \"b\"",
            "",
        ];

        let (rules, _) = input(&lines.iter().map(|s| s.to_string()).collect());

        assert!(verify("ababbb", 0, &rules));
        assert!(verify("abbbab", 0, &rules));

        assert!(!verify("bababa", 0, &rules));
        assert!(!verify("aaabbb", 0, &rules));
        assert!(!verify("aaaabbb", 0, &rules));
    }
}
