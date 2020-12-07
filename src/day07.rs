use std::collections::{HashMap, VecDeque, HashSet};
use crate::utils::lines;

type Color = String;
type Record = (usize, Color);
type Capacity = Vec<Record>;

fn input(lines: &Vec<String>) -> HashMap<Color, Capacity> {
    lines.iter()
        .map(|line| parse(line))
        .fold(HashMap::new(), |mut acc, (color, capacity)| {
            acc.insert(color, capacity);
            acc
        })
}

fn parse(line: &str) -> (Color, Capacity) {
    fn parse_cap(remaining: &str) -> Record {
        let words: Vec<&str> = remaining.split(" ").collect();
        let count: usize = words[0].parse().unwrap();
        let color = words[1].to_string() + " " + words[2];
        (count, color)
    }

    let mut mid = line.split(" bags contain ");
    let color = mid.next().unwrap().to_string();
    let remaining = mid.next().unwrap();

    if remaining.starts_with("no") {
        (color, vec![])
    } else {
        if remaining.contains(",") {
            let capacity: Vec<Record> = remaining.split(", ")
                .map(|sub_line| parse_cap(sub_line))
                .collect();
            (color, capacity)
        } else {
            (color, vec![parse_cap(remaining)])
        }
    }
}

fn build_reverse_index(index: &HashMap<Color, Capacity>) -> HashMap<Color, Vec<Color>> {
    index.iter()
        .flat_map(|(color, cap)|
            cap.iter().map(move |c| (color.clone(), c.1.clone())))
        .fold(HashMap::new(), |mut acc, (parent, child)| {
            if !acc.contains_key(&child) {
                acc.insert(child.clone(), Vec::new());
            }
            acc.get_mut(&child).unwrap().push(parent.to_owned());
            acc
        })
}

fn bfs(reverse_index: &HashMap<Color, Vec<Color>>, color: &str) -> usize {
    let mut queue: VecDeque<Color> = VecDeque::new();
    reverse_index.get(color).unwrap().iter()
        .for_each(|color| queue.push_back(color.clone()));

    let mut seen: HashSet<Color> = HashSet::new();
    while !queue.is_empty() {
        let color = queue.pop_front().unwrap();
        seen.insert(color.clone());

        reverse_index.get(&color).unwrap_or(&vec![])
            .iter()
            .for_each(|next| {
                if !seen.contains(next) {
                    queue.push_back(next.clone());
                }
            });
    }

    seen.len()
}

fn dfs(index: &HashMap<Color, Capacity>, color: &str) -> usize {
    let capacity = index.get(color).unwrap();
    capacity.iter().fold(1, |acc, (n, color)| {
        acc + n * dfs(index, color)
    })
}

pub fn main() {
    let index = input(&lines());
    let reverse = build_reverse_index(&index);

    let count = bfs(&reverse, "shiny gold");
    println!("{}", count);

    let count = dfs(&index, "shiny gold") - 1; // do not count the 'shiny gold' wrapper
    println!("{}", count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_full() {
        let line = "faded yellow bags contain 4 mirrored fuchsia bags, 4 dotted indigo bags, 3 faded orange bags, 5 plaid crimson bags.";
        let (color, capacity) = parse(line);

        assert_eq!(color, "faded yellow");
        assert_eq!(capacity, vec![
            (4, "mirrored fuchsia".to_string()),
            (4, "dotted indigo".to_string()),
            (3, "faded orange".to_string()),
            (5, "plaid crimson".to_string()),
        ]);
    }

    #[test]
    fn test_parse_one() {
        let line = "muted fuchsia bags contain 3 shiny bronze bags.";
        let (color, capacity) = parse(line);

        assert_eq!(color, "muted fuchsia");
        assert_eq!(capacity, vec![(3, "shiny bronze".to_string())]);
    }

    #[test]
    fn test_parse_none() {
        let line = "plaid gray bags contain no other bags.";
        let (color, capacity) = parse(line);

        assert_eq!(color, "plaid gray");
        assert_eq!(capacity, vec![]);
    }

    #[test]
    fn test_bfs() {
        let lines = vec![
            "light red bags contain 1 bright white bag, 2 muted yellow bags.",
            "dark orange bags contain 3 bright white bags, 4 muted yellow bags.",
            "bright white bags contain 1 shiny gold bag.",
            "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.",
            "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.",
            "dark olive bags contain 3 faded blue bags, 4 dotted black bags.",
            "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.",
            "faded blue bags contain no other bags.",
            "dotted black bags contain no other bags.",
        ];

        let strings: Vec<String> = lines.into_iter().map(|s| s.to_string()).collect();
        let index = input(&strings);
        let reverse_index = build_reverse_index(&index);

        let count = bfs(&reverse_index, "shiny gold");
        assert_eq!(count, 4);
    }

    #[test]
    fn test_dfs() {
        let lines = vec![
            "shiny gold bags contain 2 dark red bags.",
            "dark red bags contain 2 dark orange bags.",
            "dark orange bags contain 2 dark yellow bags.",
            "dark yellow bags contain 2 dark green bags.",
            "dark green bags contain 2 dark blue bags.",
            "dark blue bags contain 2 dark violet bags.",
            "dark violet bags contain no other bags.",
        ];

        let strings: Vec<String> = lines.into_iter().map(|s| s.to_string()).collect();
        let index = input(&strings);

        let count = dfs(&index, "shiny gold");
        assert_eq!(count - 1, 126);
    }
}
