use crate::day03::part1::{input, count_trees, Point};

pub fn main() {
    let field = input();

    let slopes = vec![
        (1, 1),
        (3, 1),
        (5, 1),
        (7, 1),
        (1, 2),
    ];

    let trees = slopes.iter()
        .map(|d| count_trees(&field, Point(0,0), *d))
        .fold(1, |acc, t| acc * t);

    println!("{}", trees);
}