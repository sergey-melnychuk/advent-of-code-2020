use crate::utils::lines;

fn input() -> Vec<Vec<char>> {
    lines().into_iter()
        .map(|line| line.chars().collect())
        .collect()
}

const EMPTY: char = 'L';
const TAKEN: char = '#';
const FLOOR: char = '.';

fn state(chr: char, cnt: usize, k: usize) -> char {
    match chr {
        EMPTY if cnt == 0 => TAKEN,
        TAKEN if cnt >= k => EMPTY,
        _ => chr
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Pos {
    row: i32,
    col: i32,
}

impl Pos {
    fn is_valid(&self, grid: &Vec<Vec<char>>) -> bool {
        self.row >= 0 && self.row < grid.len() as i32 &&
        self.col >= 0 && self.col < grid[0].len() as i32
    }

    fn is_edge(&self, grid: &Vec<Vec<char>>) -> bool {
        (self.row == 0 || self.row == (grid.len()-1) as i32) &&
        (self.col == 0 || self.col == (grid[0].len()-1) as i32)
    }

    fn add(&self, r: i32, c: i32) -> Pos {
        Pos { row: self.row + r, col: self.col + c }
    }

    fn of(row: i32, col: i32) -> Pos {
        Pos { row, col }
    }
}

fn get<T: Clone>(grid: &Vec<Vec<T>>, pos: &Pos) -> T {
    grid[pos.row as usize][pos.col as usize].clone()
}

fn set<T>(grid: &mut Vec<Vec<T>>, pos: &Pos, val: T) {
    *grid.get_mut(pos.row as usize).unwrap().get_mut(pos.col as usize).unwrap() = val;
}

fn adj(grid: &Vec<Vec<char>>, pos: &Pos) -> Vec<Pos> {
    let mut result = Vec::with_capacity(8);
    for r in -1..=1 {
        for c in -1..=1 {
            if r == 0 && c == 0 {
                continue;
            }
            let p = pos.add(r, c);
            if p.is_valid(grid) {
                result.push(p);
            }
        }
    }
    result
}

fn look(grid: &Vec<Vec<char>>, pos: &Pos, dir: (i32, i32)) -> Option<Pos> {
    let mut p = pos.clone();
    loop {
        let n = p.add(dir.0, dir.1);
        if !n.is_valid(&grid) {
            return None;
        }
        if n.is_edge(&grid) {
            return Some(n);
        }
        let c = get(grid, &n);
        if c != FLOOR {
            return Some(n);
        }
        p = n;
    }
}

// Find closest 'seen' (ignore floor) seat in each of 8 directions
fn seen(grid: &Vec<Vec<char>>, pos: &Pos) -> Vec<Pos> {
    let mut result = Vec::with_capacity(8);
    for r in -1..=1 {
        for c in -1..=1 {
            if r == 0 && c == 0 {
                continue;
            }
            let d = (r, c);
            if let Some(p) = look(grid, &pos, d) {
                result.push(p);
            }
        }
    }
    result
}

fn count(grid: &Vec<Vec<char>>, peers: fn(&Vec<Vec<char>>, &Pos) -> Vec<Pos>) -> Vec<Vec<usize>> {
    let (rows, cols): (i32, i32) = (grid.len() as i32, grid[0].len() as i32);
    let mut result = Vec::with_capacity(rows as usize);
    for _ in 0..rows {
        result.push(vec![0; cols as usize]);
    }
    for row in 0..rows {
        for col in 0..cols {
            let pos = Pos::of(row, col);
            let adj = peers(grid, &pos);
            let cnt = adj.iter()
                .filter(|p| get(grid, p) == TAKEN)
                .count();
            set(&mut result, &pos, cnt);
        }
    }
    result
}

fn update(grid: &mut Vec<Vec<char>>, count: Vec<Vec<usize>>, k: usize) -> usize {
    let mut changes = 0;
    let (rows, cols): (i32, i32) = (grid.len() as i32, grid[0].len() as i32);
    for row in 0..rows {
        for col in 0..cols {
            let pos = Pos::of(row, col);
            let before = get(grid, &pos);
            let taken= get(&count, &pos);
            let after = state(before, taken, k);
            if before != after {
                set(grid, &pos, after);
                changes += 1;
            }
        }
    }
    changes
}

fn stabilize(grid: &mut Vec<Vec<char>>, k: usize, peers: fn(&Vec<Vec<char>>, &Pos) -> Vec<Pos>) {
    loop {
        let counts = count(grid, peers);
        let changed = update(grid, counts, k);
        if changed == 0 {
            break;
        }
    }
}

fn taken(grid: &Vec<Vec<char>>) -> usize {
    grid.iter()
        .flat_map(|vec| vec)
        .filter(|c| **c == TAKEN)
        .count()
}

pub fn main() {
    let input = input();

    let mut grid = input.clone();
    stabilize(&mut grid, 4, adj);
    let n = taken(&grid);
    println!("{}", n);

    let mut grid = input.clone();
    stabilize(&mut grid, 5, seen);
    let n = taken(&grid);
    println!("{}", n);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_grid(input: Vec<&str>) -> Vec<Vec<char>> {
        input.clone()
            .into_iter()
            .map(|s| s.chars().collect())
            .collect()
    }

    #[test]
    fn test_get() {
        let grid = make_grid(vec![
            "L#L",
            "#.#",
            "L#L"
        ]);

        assert_eq!(get(&grid, &Pos::of(0, 0)), EMPTY);
        assert_eq!(get(&grid, &Pos::of(1, 0)), TAKEN);
        assert_eq!(get(&grid, &Pos::of(0, 1)), TAKEN);
        assert_eq!(get(&grid, &Pos::of(1, 1)), FLOOR);
    }

    #[test]
    fn test_adj() {
        let grid = make_grid(vec![
            "L#L",
            "#.#",
            "L#L"
        ]);

        let cases = vec![
            (Pos::of(0, 0), vec![Pos::of(0, 1), Pos::of(1, 0), Pos::of(1, 1)])
        ];

        for (p, out) in cases {
            assert_eq!(adj(&grid, &p), out);
        }
    }

    #[test]
    fn test_look_edge() {
        let grid = make_grid(vec![
            ".......#.",
            "...#.....",
            ".#.......",
            ".........",
            "..#L....#",
            "....#....",
            ".........",
            "#........",
            "...#.....",
        ]);

        let pos = Pos::of(0, 0);
        assert_eq!(look(&grid, &pos, (-1, -1)), None);
    }

    #[test]
    fn test_look() {
        let grid = make_grid(vec![
            ".......#.",
            "...#.....",
            ".#.......",
            ".........",
            "..#L....#",
            "....#....",
            ".........",
            "#........",
            "...#.....",
        ]);

        let pos = Pos::of(4, 3);
        assert_eq!(get(&grid, &pos), 'L');
        assert_eq!(look(&grid, &pos, (-1, -1)), Some(Pos::of(2, 1)));
        assert_eq!(look(&grid, &pos, ( 0, -1)), Some(Pos::of(4, 2)));
        assert_eq!(look(&grid, &pos, ( 1, -1)), Some(Pos::of(7, 0)));
        assert_eq!(look(&grid, &pos, (-1,  0)), Some(Pos::of(1, 3)));
        assert_eq!(look(&grid, &pos, ( 1,  0)), Some(Pos::of(8, 3)));
        assert_eq!(look(&grid, &pos, (-1,  1)), Some(Pos::of(0, 7)));
        assert_eq!(look(&grid, &pos, ( 0,  1)), Some(Pos::of(4, 8)));
        assert_eq!(look(&grid, &pos, ( 1,  1)), Some(Pos::of(5, 4)));
    }

    #[test]
    fn test_seen_all() {
        let grid = make_grid(vec![
            ".......#.",
            "...#.....",
            ".#.......",
            ".........",
            "..#L....#",
            "....#....",
            ".........",
            "#........",
            "...#.....",
        ]);

        let pos = Pos::of(4, 3);
        assert_eq!(seen(&grid, &pos), vec![
            Pos::of(2, 1),
            Pos::of(1, 3),
            Pos::of(0, 7),
            Pos::of(4, 2),
            Pos::of(4, 8),
            Pos::of(7, 0),
            Pos::of(8, 3),
            Pos::of(5, 4),
        ]);
    }

    #[test]
    fn test_count_adj() {
        let grid = make_grid(vec![
            "L#L",
            "#.#",
            "L#L"
        ]);
        assert_eq!(count(&grid, adj), vec![
            vec![2, 2, 2],
            vec![2, 4, 2],
            vec![2, 2, 2],
        ]);
    }

    #[test]
    fn test_count_seen_all() {
        let grid = make_grid(vec![
            ".......#.",
            "...#.....",
            ".#.......",
            ".........",
            "..#L....#",
            "....#....",
            ".........",
            "#........",
            "...#.....",
        ]);

        assert_eq!(grid[4][3], 'L');
        assert_eq!(count(&grid, seen)[4][3], 8);
    }

    #[test]
    fn test_count_seen_none() {
        let grid = make_grid(vec![
            ".##.##.",
            "#.#.#.#",
            "##...##",
            "...L...",
            "##...##",
            "#.#.#.#",
            ".##.##.",
        ]);

        assert_eq!(grid[3][3], 'L');
        assert_eq!(count(&grid, seen)[3][3], 0);
    }

    #[test]
    fn test_count_seen_zero() {
        let grid = make_grid(vec![
            ".............",
            ".L.L.#.#.#.#.",
            ".............",
        ]);

        assert_eq!(grid[1][1], 'L');
        assert_eq!(count(&grid, seen)[1][1], 0);
    }


    #[test]
    fn test_taken_zero() {
        let grid = make_grid(vec![
            "L.LL.LL.LL",
            "LLLLLLL.LL",
            "L.L.L..L..",
            "LLLL.LL.LL",
            "L.LL.LL.LL",
            "L.LLLLL.LL",
            "..L.L.....",
            "LLLLLLLLLL",
            "L.LLLLLL.L",
            "L.LLLLL.LL",
        ]);
        assert_eq!(taken(&grid), 0);
    }

    #[test]
    fn test_taken_one() {
        let grid = make_grid(vec![
            "L.LL.LL.LL",
            "LLLLLLL.LL",
            "L.L.L..L..",
            "LLLL.LL.LL",
            "L.LL#LL.LL",
            "L.LLLLL.LL",
            "..L.L.....",
            "LLLLLLLLLL",
            "L.LLLLLL.L",
            "L.LLLLL.LL",
        ]);
        assert_eq!(taken(&grid), 1);
    }

    #[test]
    fn test_example1() {
        let mut grid = make_grid(vec![
            "L.LL.LL.LL",
            "LLLLLLL.LL",
            "L.L.L..L..",
            "LLLL.LL.LL",
            "L.LL.LL.LL",
            "L.LLLLL.LL",
            "..L.L.....",
            "LLLLLLLLLL",
            "L.LLLLLL.L",
            "L.LLLLL.LL",
        ]);
        stabilize(&mut grid, 4,adj);
        assert_eq!(taken(&grid), 37);
    }

    #[test]
    fn test_example2() {
        let mut grid = make_grid(vec![
            "L.LL.LL.LL",
            "LLLLLLL.LL",
            "L.L.L..L..",
            "LLLL.LL.LL",
            "L.LL.LL.LL",
            "L.LLLLL.LL",
            "..L.L.....",
            "LLLLLLLLLL",
            "L.LLLLLL.L",
            "L.LLLLL.LL",
        ]);
        stabilize(&mut grid, 5, seen);
        assert_eq!(taken(&grid), 26);
    }
}
