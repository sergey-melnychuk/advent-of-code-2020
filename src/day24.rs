use crate::utils::lines;
use std::collections::{HashMap, HashSet};


#[derive(Debug, Eq, PartialEq)]
enum Dir {
    E,
    SE,
    SW,
    W,
    NW,
    NE,
}

#[derive(Debug, Default, Eq, PartialEq, Hash, Clone)]
struct Cell(i32, i32);

impl Cell {
    fn zero() -> Self {
        Cell(0, 0)
    }

    fn step(&self, dir: &Dir) -> Cell {
        match dir {
            Dir::NE => Cell(self.0 - 1, self.1 + 1),
            Dir::E  => Cell(self.0    , self.1 + 1),
            Dir::SE => Cell(self.0 + 1, self.1    ),
            Dir::NW => Cell(self.0 - 1, self.1    ),
            Dir::W  => Cell(self.0    , self.1 - 1),
            Dir::SW => Cell(self.0 + 1, self.1 - 1)
        }
    }

    fn adj(&self) -> Vec<Cell> {
        vec![
            self.step(&Dir::NE),
            self.step(&Dir::E),
            self.step(&Dir::SE),
            self.step(&Dir::NW),
            self.step(&Dir::W),
            self.step(&Dir::SW),
        ]
    }
}

fn parse(line: &str) -> Vec<Dir> {
    let mut it = line.chars().into_iter();

    let mut acc = Vec::new();
    let mut p: char = 'X';
    loop {
        let next = it.next();
        if next.is_none() {
            break;
        }
        let c = next.unwrap();

        if p == 's' || p == 'n' {
            match (p, c) {
                ('s', 'e') => acc.push(Dir::SE),
                ('s', 'w') => acc.push(Dir::SW),
                ('n', 'e') => acc.push(Dir::NE),
                ('n', 'w') => acc.push(Dir::NW),
                _ => unreachable!()
            }
            p = 'X';
        } else {
            match c {
                'e' => acc.push(Dir::E),
                'w' => acc.push(Dir::W),
                _ => p = c
            }
        }
    }

    acc
}

fn input(lines: &Vec<String>) -> Vec<Vec<Dir>> {
    lines.into_iter()
        .map(|line| parse(line))
        .collect()
}

fn reduce(cell: Cell, path: &Vec<Dir>) -> Cell {
    path.into_iter()
        .fold(cell, |acc, dir| acc.step(dir))
}

fn flip(floor: &mut HashMap<Cell, usize>, path: &Vec<Dir>) {
    let cell = reduce(Cell::zero(), path);
    *floor.entry(cell).or_default() += 1;
}

fn count(floor: &HashMap<Cell, usize>) -> usize {
    floor.values()
        .into_iter()
        .cloned()
        .filter(|n| n % 2 > 0)
        .count()
}

fn day(floor: &mut HashMap<Cell, usize>) {
    let all = floor.keys()
        .into_iter()
        .cloned()
        .flat_map(|cell| {
            let mut adj = cell.adj();
            adj.push(cell);
            adj
        })
        .collect::<HashSet<_>>();

    let index = all.iter()
        .map(|cell| {
            let black = cell.adj().into_iter()
                .filter(|c| floor.get(c)
                    .map(|n| n % 2 > 0)
                    .unwrap_or_default())
                .count();
            (cell.clone(), black)
        })
        .collect::<HashMap<_, _>>();

    all.into_iter()
        .for_each(|cell| {
            let is_black = floor.get(&cell)
                .map(|n| n % 2 > 0)
                .unwrap_or_default();
            let n = index.get(&cell).cloned().unwrap_or_default();

            // Any black tile with zero or more than 2 black tiles immediately adjacent to it is flipped to white.
            // Any white tile with exactly 2 black tiles immediately adjacent to it is flipped to black.
            match (is_black, n) {
                (true, n) if n == 0 || n > 2 => {
                    floor.insert(cell, 0);
                },
                (false, n) if n == 2 => {
                    floor.insert(cell, 1);
                },
                _ => ()
            }
        })
}


pub fn main() {
    let paths = input(&lines());

    let mut floor: HashMap<Cell, usize> = HashMap::new();
    paths.iter()
        .for_each(|path| flip(&mut floor, path));

    let n = count(&floor);
    println!("{}", n);

    for _ in 0..100 {
        day(&mut floor);
    }
    let n = count(&floor);
    println!("{}", n);
}

#[cfg(test)]
mod tests {
    use super::*;
    use Dir::*;

    #[test]
    fn test_parse() {
        assert_eq!(parse("nwwwewwewww"), vec![NW, W, W, E, W, W, E, W, W, W]);
    }

    #[test]
    fn test_example_part1() {
        let paths = example();

        let mut floor: HashMap<Cell, usize> = HashMap::new();
        paths.iter()
            .for_each(|path| flip(&mut floor, path));

        let n = count(&floor);
        assert_eq!(n, 10);
    }

    #[test]
    fn test_step() {
        let path = parse("nwwswee");
        let cell = reduce(Cell::zero(), &path);
        assert_eq!(cell, Cell::zero());
    }

    #[test]
    fn test_example_part2() {
        let paths = example();

        let mut floor: HashMap<Cell, usize> = HashMap::new();
        paths.iter()
            .for_each(|path| flip(&mut floor, path));

        day(&mut floor);
        assert_eq!(count(&floor), 15);

        for _ in 0..99 {
            day(&mut floor);
        }
        assert_eq!(count(&floor), 2208);
    }

    fn example() -> Vec<Vec<Dir>> {
        let raw = vec![
            "sesenwnenenewseeswwswswwnenewsewsw",
            "neeenesenwnwwswnenewnwwsewnenwseswesw",
            "seswneswswsenwwnwse",
            "nwnwneseeswswnenewneswwnewseswneseene",
            "swweswneswnenwsewnwneneseenw",
            "eesenwseswswnenwswnwnwsewwnwsene",
            "sewnenenenesenwsewnenwwwse",
            "wenwwweseeeweswwwnwwe",
            "wsweesenenewnwwnwsenewsenwwsesesenwne",
            "neeswseenwwswnwswswnw",
            "nenwswwsewswnenenewsenwsenwnesesenew",
            "enewnwewneswsewnwswenweswnenwsenwsw",
            "sweneswneswneneenwnewenewwneswswnese",
            "swwesenesewenwneswnwwneseswwne",
            "enesenwswwswneneswsenwnewswseenwsese",
            "vwnwnesenesenenwwnenwsewesewsesesew",
            "nenewswnwewswnenesenwnesewesw",
            "eneswnwswnwsenenwnwnwwseeswneewsenese",
            "neswnwewnwnwseenwseesewsenwsweewe",
            "wseweeenwnesenwwwswnew",
        ];

        raw.into_iter()
            .map(|line| parse(line))
            .collect()
    }
}
