use std::collections::HashSet;
use crate::utils::lines;

fn any(lines: &Vec<String>) -> Vec<HashSet<char>> {
    lines.split(|line| line.is_empty())
        .map(|lines| {
            let mut set = HashSet::new();
            lines.iter()
                .flat_map(|line| line.chars())
                .for_each(|c| {
                    set.insert(c);
                });
            set
        })
        .collect()
}

fn all(lines: &Vec<String>) -> Vec<HashSet<char>> {
    lines.split(|line| line.is_empty())
        .map(|lines| {
            let sets: Vec<HashSet<char>> = lines.iter()
                .map(|line| line.chars().collect::<HashSet<char>>())
                .collect();

            let head = sets[0].to_owned();
            sets.iter().skip(1)
                .fold(head, |acc, e|
                    acc.intersection(e)
                        .map(|e| *e)
                        .collect())
        })
        .collect()
}

pub fn main() {
    let input = lines();

    let groups = any(&input);
    let sum: usize = groups.iter()
        .map(|set| set.len())
        .sum();
    println!("{}", sum);

    let groups = all(&input);
    let sum: usize = groups.iter()
        .map(|set| set.len())
        .sum();
    println!("{}", sum);
}
