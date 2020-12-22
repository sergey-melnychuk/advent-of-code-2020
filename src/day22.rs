use crate::utils::lines;
use std::collections::{VecDeque, HashSet};

type Card = usize;

fn parse(lines: &Vec<String>) -> (VecDeque<Card>, VecDeque<Card>) {
    let mut it = lines.split(|s| s.is_empty());
    let a = it.next().unwrap().into_iter()
        .skip(1)
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    let b = it.next().unwrap().into_iter()
        .skip(1)
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    (a, b)
}

fn play(mut a: VecDeque<Card>, mut b: VecDeque<Card>) -> Vec<Vec<Card>> {
    loop {
        if a.is_empty() || b.is_empty() {
            return vec![
                a.into_iter().collect(),
                b.into_iter().collect(),
            ];
        }

        let card_a = a.pop_front().unwrap();
        let card_b = b.pop_front().unwrap();

        if card_a > card_b {
            a.push_back(card_a);
            a.push_back(card_b);
        }
        if card_a < card_b {
            b.push_back(card_b);
            b.push_back(card_a);
        }
    }
}

fn rplay(mut a: VecDeque<Card>, mut b: VecDeque<Card>) -> Vec<Vec<Card>> {
    let mut log = HashSet::new();
    loop {
        if a.is_empty() || b.is_empty() {
            return vec![
                a.into_iter().collect(),
                b.into_iter().collect(),
            ];
        }

        let state = format!("A={:?} B={:?}", a, b);
        if log.contains(&state) {
            return vec![vec![0], vec![]]; // player A wins
        }
        log.insert(state);

        let card_a = a.pop_front().unwrap();
        let card_b = b.pop_front().unwrap();

        if card_a <= a.len() && card_b <= b.len() {
            // play a sub-game
            let sub_a = a.iter().take(card_a).cloned().collect();
            let sub_b = b.iter().take(card_b).cloned().collect();
            let sub_game = rplay(sub_a, sub_b);
            if sub_game.get(0).unwrap().is_empty() {
                // player B wins this round
                b.push_back(card_b);
                b.push_back(card_a);
            } else {
                // player A wins this round
                a.push_back(card_a);
                a.push_back(card_b);
            }
        } else {
            if card_a > card_b {
                a.push_back(card_a);
                a.push_back(card_b);
            } else {
                b.push_back(card_b);
                b.push_back(card_a);
            }
        }

    }
}

fn score(result: &Vec<Vec<Card>>) -> usize {
    result.into_iter()
        .flatten()
        .rev()
        .cloned()
        .enumerate()
        .map(|(i, x)| (i + 1) * x )
        .sum()
}

pub fn main() {
    let (a, b) = parse(&lines());

    let r = play(a.clone(), b.clone());
    let s = score(&r);
    println!("{}", s);

    let r = rplay(a.clone(), b.clone());
    let s = score(&r);
    println!("{}", s);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part2() {
        let a: Vec<Card> = vec![9,2,6,3,1];
        let b: Vec<Card> = vec![5,8,4,7,10];

        let r = rplay(
            a.clone().into_iter().collect(),
            b.clone().into_iter().collect());

        let s = score(&r);
        assert_eq!(s, 291);
    }
}
