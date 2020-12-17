use crate::utils::lines;
use std::collections::{HashSet, HashMap};

type Cell = (i32, i32, i32, i32);

type State = HashSet<Cell>;

type Index = HashMap<Cell, u32>;


fn parse(lines: Vec<String>) -> Vec<Cell> {
    lines.into_iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().into_iter()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(|(x, _)| (x as i32, y as i32, 0i32, 0i32))
                .collect::<Vec<_>>()
        })
        .collect()
}

fn adj3(cell: &Cell) -> Vec<Cell> {
    let mut result = Vec::with_capacity(26);
    for x in -1..=1 {
        for y in -1..=1 {
            for z in -1..=1 {
                if x == 0 && y == 0 && z == 0 {
                    continue;
                }
                let cell = (cell.0 + x, cell.1 + y, cell.2 + z, 0i32);
                result.push(cell);
            }
        }
    }
    result
}

fn adj4(cell: &Cell) -> Vec<Cell> {
    let mut adj: Vec<Cell> = (-1..=1)
        .into_iter()
        .flat_map(|w| {
            adj3(cell)
                .into_iter()
                .map(|mut c| {
                    c.3 = cell.3 + w;
                    c
                })
                .collect::<Vec<_>>()
        })
        .collect();
    adj.push((cell.0, cell.1, cell.2, cell.3 + 1));
    adj.push((cell.0, cell.1, cell.2, cell.3 - 1));
    adj
}

fn index(state: &State, adj: fn(&Cell) -> Vec<Cell>) -> Index {
    let mut index = HashMap::new();
    state.iter()
        .flat_map(|cell| adj(cell))
        .for_each(|cell| *index.entry(cell.clone()).or_default() += 1);
    index
}

fn apply(state: &State, index: &Index) -> State {
    index.iter()
        .map(|(cell, n)| (cell, n, state.contains(cell)))
        .filter(|(_, n, exists)| match (n, exists) {
            (n,  true) => **n == 2 || **n == 3,
            (n, false) => **n == 3,
        })
        .map(|(cell, _, _)| cell)
        .cloned()
        .collect()
}

pub fn main() {
    let state = parse(lines())
        .into_iter()
        .collect::<HashSet<_>>();

    let count = (0..6)
        .into_iter()
        .fold(state.clone(), |state, _| {
            let idx = index(&state, adj3);
            apply(&state, &idx)
        })
        .len();
    println!("{}", count);

    let count = (0..6)
        .into_iter()
        .fold(state.clone(), |state, _| {
            let idx = index(&state, adj4);
            apply(&state, &idx)
        })
        .len();
    println!("{}", count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let lines = vec![
            ".#.",
            "..#",
            "###",
        ];

        let parsed = parse(lines.into_iter().map(|s| s.to_owned()).collect());
        assert_eq!(parsed, vec![
            (1, 0, 0, 0),
            (2, 1, 0, 0),
            (0, 2, 0, 0),
            (1, 2, 0, 0),
            (2, 2, 0, 0),
        ]);
    }

    #[test]
    fn test_apply3() {
        let cells = vec![
            (1, 0, 0, 0),
            (2, 1, 0, 0),
            (0, 2, 0, 0),
            (1, 2, 0, 0),
            (2, 2, 0, 0),
        ];

        let state = cells.into_iter().collect::<HashSet<_>>();

        let count = (0..6)
            .into_iter()
            .fold(state, |state, _| {
                let idx = index(&state, adj3);
                apply(&state, &idx)
            })
            .len();

        assert_eq!(count, 112);
    }

    #[test]
    fn test_apply4() {
        let cells = vec![
            (1, 0, 0, 0),
            (2, 1, 0, 0),
            (0, 2, 0, 0),
            (1, 2, 0, 0),
            (2, 2, 0, 0),
        ];

        let state = cells.into_iter().collect::<HashSet<_>>();

        let count = (0..6)
            .into_iter()
            .fold(state, |state, _| {
                let idx = index(&state, adj4);
                apply(&state, &idx)
            })
            .len();

        assert_eq!(count, 848);
    }
}
