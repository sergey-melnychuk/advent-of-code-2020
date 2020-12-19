use crate::utils::lines;
use std::collections::HashMap;

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

#[allow(dead_code)]
fn is_recursive(id: Id, rule: &Rule) -> bool {
    match rule {
        Rule::Seq(ids) => ids.contains(&id),
        Rule::Or(rules) => rules.iter().any(|r| is_recursive(id, r)),
        _ => false
    }
}

fn apply(line: String, rule: &Rule, map: &HashMap<Id, Rule>) -> Option<String> {
    //println!("apply: line='{}' rule='{:?}'", line, rule);
    if line.is_empty() {
        return None; // 170 - too low
        // return Some(String::default()); // 315 - too high
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

fn verify(line: &str, id: Id, map: &HashMap<Id, Rule>) -> bool {
    let rule = map.get(&id).unwrap();
    apply(line.to_string(), rule, map)
        .map(|s| s.is_empty())
        .unwrap_or_default()
}

// 8: 42 | 42 8
// 11: 42 31 | 42 11 31
fn update(mut rules: HashMap<Id, Rule>) -> HashMap<Id, Rule> {
    rules.insert(8, Rule::Or(vec![
        Rule::Seq(vec![42]),
        Rule::Seq(vec![42, 8]),
    ]));
    rules.insert(11, Rule::Or(vec![
        Rule::Seq(vec![42, 31]),
        Rule::Seq(vec![42, 11, 31]),
    ]));
    rules
}

pub fn main() {
    let lines = lines();
    let (rules, inputs) = input(&lines);

    let n = inputs.iter()
        .filter(|s| verify(s, 0, &rules))
        .count();
    println!("{}", n);

    let rules = update(rules);
    let n = inputs.iter()
        .filter(|s| verify(s, 0, &rules))
        .count();
    println!("{}", n); // correct answer: 306
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
    fn test_validate() {
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

    #[test]
    fn test_part2() {
        let (rules, lines) = input(&part2());
        assert_eq!(rules.len(), 31);
        assert_eq!(lines.len(), 15);
        assert!(rules.contains_key(&8), "has 8");
        assert!(rules.contains_key(&11), "has 11");

        let matched = lines.iter()
            .filter(|s| verify(s, 0, &rules))
            .collect::<Vec<_>>();
        assert_eq!(matched, vec![
            "bbabbbbaabaabba",
            "ababaaaaaabaaab",
            "ababaaaaabbbaba",
        ]);

        let rules = update(rules);
        assert!(rules.contains_key(&8), "has 8");
        assert!(rules.contains_key(&11), "has 11");

        let matched = lines.iter()
            .filter(|s| verify(s, 0, &rules))
            .collect::<Vec<_>>();
        //assert_eq!(matched.len(), 12);

        let xs = vec![
            "bbabbbbaabaabba",
            "ababaaaaaabaaab",
            "ababaaaaabbbaba",
            "baabbaaaabbaaaababbaababb",
            "aaaabbaabbaaaaaaabbbabbbaaabbaabaaa",
            "aaabbbbbbaaaabaababaabababbabaaabbababababaaa",
            "aaaaabbaabaaaaababaa", // fails
            "bbbbbbbaaaabbbbaaabbabaaa", // fails
            "abbbbabbbbaaaababbbbbbaaaababb", // fails
            "babbbbaabbbbbabbbbbbaabaaabaaa", // fails
            "bbbababbbbaaaaaaaabbababaaababaabab", // fails
            "aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba", // fails
        ];
        for x in xs {
            assert!(verify(x, 0, &rules), "{}", x);
        }
    }

    #[test]
    #[ignore]
    fn test_must_verify_1() {
        let (rules, _) = input(&part2());
        let rules = update(rules);

        let s = "aabaaabaaa";
        let r = 8;
        assert!(verify(s, r, &rules));
    }

    fn part2() -> Vec<String> {
        let all = r#"42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"#;
        all.split("\n").into_iter().map(|s| s.to_string()).collect()
    }
}
