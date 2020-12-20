use crate::utils::lines;
use std::collections::HashMap;

#[derive(Clone)]
struct Tile {
    id: u64,
    tile: Vec<Vec<char>>,
}

impl Tile {
    fn vflip(&self) -> Tile {
        let tile = self.tile
            .iter()
            .cloned()
            .map(|row| row.into_iter().rev().collect())
            .collect();

        Tile {
            id: self.id,
            tile
        }
    }

    fn hflip(&self) -> Tile {
        let tile = self.tile
            .iter()
            .cloned()
            .rev()
            .collect();

        Tile {
            id: self.id,
            tile
        }
    }

    fn cw(&self) -> Tile {
        let n = self.tile.len();

        let tile = (0..n).into_iter()
            .map(|i| self.tile
                .iter()
                .map(|vec| vec.get(i).unwrap())
                .cloned()
                .rev()
                .collect())
            .collect();

        Tile {
            id: self.id,
            tile
        }
    }

    fn ccw(&self) -> Tile {
        let n = self.tile.len();

        let tile = (0..n).into_iter()
            .map(|i| self.tile
                .iter()
                .map(|vec| vec.get(i).unwrap())
                .cloned()
                .collect())
            .rev()
            .collect();

        Tile {
            id: self.id,
            tile
        }
    }

    fn hfit(&self, lft: &Tile) -> bool {
        let a = lft.tile.iter()
            .map(|v| v.last().unwrap());

        let b = self.tile.iter()
            .map(|v| v.get(0).unwrap());

        a.zip(b).all(|(x, y)| x == y)
    }

    fn vfit(&self, top: &Tile) -> bool {
        let a: &Vec<char> = top.tile.iter()
            .last()
            .unwrap();

        let b: &Vec<char> = self.tile
            .get(0)
            .unwrap();

        a == b
    }
}

fn parse(lines: &[String]) -> Tile {
    let mut it = lines.iter();
    let header = it.next().unwrap();

    let id: u64 = header.chars()
        .into_iter()
        .skip_while(|c| !c.is_numeric())
        .take_while(|c| c.is_numeric())
        .collect::<String>()
        .parse()
        .unwrap();

    let tile: Vec<Vec<char>> = it
        .map(|s| s.chars().collect())
        .collect();

    Tile { id, tile }
}

fn input(lines: Vec<String>) -> Vec<Tile> {
    lines.split(|s| s.is_empty())
        .into_iter()
        .map(|chunk| parse(chunk))
        .collect()
}

fn pack(vec: &Vec<char>, one: char) -> u32 {
    assert!(vec.len() <= 32);
    vec.into_iter()
        .map(|c| if *c == one {1u32} else {0u32})
        .fold(0u32, |acc, x| (acc << 1) + x)
}

fn edges(tile: &Tile) -> Vec<u32> {
    let top = tile.tile.get(0).unwrap();
    let bot = tile.tile.last().unwrap();

    let lft = tile.tile.iter()
        .map(|row| row.get(0).unwrap().clone())
        .collect::<Vec<_>>();
    let rgt = tile.tile.iter()
        .map(|row| row.last().unwrap().clone())
        .collect::<Vec<_>>();

    vec![
        pack(top, '#'),
        pack(bot, '#'),
        pack(&lft, '#'),
        pack(&rgt, '#'),

        // each image tile has been rotated and flipped to a random orientation
        pack(&top.iter().cloned().rev().collect(), '#'),
        pack(&bot.iter().cloned().rev().collect(), '#'),
        pack(&lft.iter().cloned().rev().collect(), '#'),
        pack(&rgt.iter().cloned().rev().collect(), '#'),
    ]
}

fn index(tiles: &Vec<Tile>) -> HashMap<u64, Vec<u32>> {
    tiles.iter()
        .map(|tile| (tile.id, edges(tile)))
        .fold(
            HashMap::with_capacity(tiles.len()),
            |mut map, (id, vec)| {
                map.insert(id, vec);
                map
            })
}

fn count(tiles: &Vec<Tile>) -> HashMap<u32, u32> {
    tiles.iter()
        .flat_map(|tile| edges(tile))
        .fold(HashMap::new(), |mut map, edge| {
            *map.entry(edge).or_default() += 1;
            map
        })
}

fn corners(index: &HashMap<u64, Vec<u32>>, count: &HashMap<u32, u32>) -> Vec<u64> {
    let keys = {
        let mut ks = index.keys()
            .into_iter()
            .cloned()
            .collect::<Vec<_>>();
        ks.sort();
        ks
    };
    keys.iter()
        .map(|k| (k, index.get(k).unwrap()))
        .filter(|(_, edges)| {
            let unique = edges.iter()
                .map(|e| count.get(e).cloned().unwrap_or_default())
                .filter(|n| *n == 1)
                .count();
            unique == 4 // tile has 2 unique edges - it is a corner tile
        })
        .map(|(id, _)| id.clone())
        .collect()
}

fn adj(size: usize, grid: &Vec<Tile>, idx: usize, index: &HashMap<u64, Vec<u32>>) -> Vec<Vec<u32>> {
    let (row, col) = (idx / size, idx % size);

    let mut result = Vec::with_capacity(2);
    if col > 0 {
        let id = grid.get(idx - 1).unwrap().id;
        let edges = index.get(&id).unwrap();
        result.push(edges.clone());
    }
    if row > 0 {
        let id = grid.get(idx - size).unwrap().id;
        let edges = index.get(&id).unwrap();
        result.push(edges.clone());
    }
    result
}

fn align(size: usize, grid: Vec<Tile>, queue: Vec<Tile>, idx: usize, index: &HashMap<u64, Vec<u32>>, count: &HashMap<u32, u32>) -> Vec<Tile> {
    if grid.len() == size * size {
        return grid;
    }
    if queue.is_empty() {
        return vec![];
    }
    let (row, col) = (idx / size, idx % size);
    let is_edge = col == 0 || col == size-1 || row == 0 || row == size-1;
    //println!("align: idx={} is_edge={}, prev={}", idx, is_edge, grid[idx-1]);

    let adj = adj(size, &grid, idx, index);
    //println!("\tadj={:?}", adj);

    let next = queue.iter()
        .filter(|tile| {
            let edges = index.get(&tile.id).unwrap();
            adj.iter()
                .all(|es| es.iter().any(|e| edges.contains(e)))
        })
        .cloned()
        .collect::<Vec<_>>();
    let next = if !is_edge {
        next
    } else {
        next.into_iter()
            .filter(|tile| {
                let edges = index.get(&tile.id).unwrap();
                edges.iter()
                    .any(|e| *count.get(e).unwrap() == 1)
            })
            .collect()
    };
    //println!("\tnext={:?}", next);

    next.into_iter()
        .map(|tile| {
            let rest = queue.iter()
                .filter(|t| t.id != tile.id)
                .cloned()
                .collect::<Vec<_>>();

            let mut g = grid.clone();
            g.push(tile);
            align(size, g, rest, idx + 1, index, count)
        })
        .find(|vec| !vec.is_empty())
        .unwrap_or_default()
}

fn crop(tile: Tile) -> Tile {
    let n = tile.tile.len();

    let cropped = tile.tile
        .into_iter()
        .skip(1)
        .take(n - 2)
        .map(|row| row.into_iter().skip(1).take(n-2).collect())
        .collect();

    Tile {
        id: tile.id,
        tile: cropped,
    }
}

fn iterate(tile: &Tile) -> Vec<Tile> {
    // TODO FIXME: make all meaningful combinations

    vec![
        tile.clone(),

        tile.cw(),
        tile.ccw(),
        tile.vflip(),
        tile.hflip(),

        tile.cw().cw(),
        tile.vflip().cw(),
        tile.hflip().cw(),

        tile.ccw().ccw(),
        tile.vflip().ccw(),
        tile.hflip().ccw(),

        tile.cw().vflip(),
        tile.ccw().vflip(),
        tile.hflip().vflip(),

        tile.cw().hflip(),
        tile.ccw().hflip(),
        tile.vflip().hflip(),
    ]
}

fn pair<F>(a: &Tile, b: &Tile, f: F) -> (Tile, Tile)
    where
        F: Fn(&Tile, &Tile) -> bool
{
    let pairs: Vec<(Tile, Tile)> = iterate(a)
        .iter()
        .flat_map(|x| {
            iterate(b)
                .iter()
                .map(|y| (x.clone(), y.clone()))
                .collect::<Vec<_>>()
        })
        .collect();

    pairs.into_iter()
        .filter(|(x, y)| f(x, y))
        .next()
        .unwrap()
}

fn align2(size: usize, grid: Vec<Tile>, queue: Vec<Tile>, idx: usize) -> Vec<Tile> {
    if grid.len() == size * size {
        return grid;
    }
    if queue.is_empty() {
        return vec![];
    }

    let (row, col) = (idx / size, idx % size);

    let next = queue.iter()
        .flat_map(|tile| iterate(tile))
        .filter(|tile| row == 0 || {
            let top = grid.get(idx - size).unwrap();
            tile.vfit(top)
        })
        .filter(|tile| col == 0 || {
            let lft = grid.get(idx - 1).unwrap();
            lft.hfit(tile) // fast but wrong! TODO FIXME
            // tile.hfit(lft)
        })
        .collect::<Vec<_>>();

    next.iter()
        .map(|tile| {
            let queue = queue.iter()
                .filter(|t| t.id != tile.id)
                .cloned()
                .collect::<Vec<_>>();

            let mut grid = grid.clone();
            grid.push(tile.clone());

            align2(size, grid, queue, idx + 1)
        })
        .filter(|v| !v.is_empty())
        .next()
        .unwrap_or_default()
}

fn join(size: usize, aligned: &Vec<Tile>) -> Tile {
    fn append(a: Tile, b: Tile) -> Tile {
        let tile = a.tile.into_iter()
            .zip(b.tile.into_iter())
            .map(|(mut x, mut y)| {
                x.append(&mut y);
                x
            })
            .collect();

        Tile {
            id: 0,
            tile
        }
    }

    let mut tile = vec![];

    let mut it = &aligned[..];
    while it.len() > 0 {
        let chunk = &it[0..size];

        let mut row = chunk[0].to_owned();
        for i in 1..size {
            row = append(row, chunk[i].to_owned());
        }
        tile.append(&mut row.tile);

        it = &it[size..];
    }

    Tile {
        id: 0,
        tile,
    }
}

fn lookup(tile: &Vec<Vec<char>>, pattern: &Vec<Vec<char>>) -> usize {
    let (t_rows, t_cols) = (tile.len(), tile.get(0).unwrap().len());
    let (p_rows, p_cols) = (pattern.len(), pattern.get(0).unwrap().len());

    let mut count = 0;
    for r in 0..(t_rows + 1 - p_rows) {
        'outer:
        for c in 0..(t_cols + 1 - p_cols) {

            for i in 0..p_rows {
                for j in 0..p_cols {
                    let t = char_at(tile, r + i, c + j);
                    let p = char_at(pattern, i, j);
                    if p == '#' {
                        if t != '#' {
                            continue 'outer;
                        }
                    }
                }
            }
            count += 1;

        }
    }
    count
}

fn char_at(tile: &Vec<Vec<char>>, row: usize, col: usize) -> char {
    *tile.get(row).unwrap().get(col).unwrap()
}

fn chars(tile: &Vec<Vec<char>>, x: char) -> usize {
    tile.iter()
        .flat_map(|v| v)
        .filter(|c| **c == x)
        .count()
}

pub fn main() {
    let tiles = input(lines());
    let size = (tiles.len() as f64).sqrt().trunc() as usize;

    let index = index(&tiles);
    let count = count(&tiles);

    let corners = corners(&index, &count);
    assert_eq!(corners.len(), 4);
    let prod = corners.iter().product::<u64>();
    println!("{}", prod);

    let aligned = corners.into_iter()
        .map(|id| tiles.iter().find(|t| t.id == id).unwrap())
        .map(|tile| {
            let queue = tiles.iter()
                .filter(|t| t.id != tile.id)
                .cloned()
                .collect::<Vec<_>>();

            let mut grid = Vec::with_capacity(size * size);
            grid.push(tile.clone());

            align2(size, grid, queue, 1)
        })
        .find(|a| !a.is_empty())
        .unwrap_or_default();
    assert_eq!(aligned.len(), tiles.len());

    //let aligned = aligned.into_iter().map(crop).collect();
    let joined = join(size, &aligned);

    let txt = joined.tile.iter()
        .map(|row| row.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n");
    println!("{}", txt);

    let pattern = vec![
        "                  # ",
        "#    ##    ##    ###",
        " #  #  #  #  #  #   ",
    ].into_iter()
        .map(|row| row.chars().collect())
        .collect();

    let n = iterate(&joined)
        .iter()
        .map(|tile| lookup(&tile.tile, &pattern))
        .max()
        .unwrap();
    println!("n = {}", n);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_vec(line: &str) -> Vec<char> {
        line.chars()
            .into_iter()
            .collect()
    }

    fn to_vec_rev(line: &str) -> Vec<char> {
        line.chars()
            .into_iter()
            .rev()
            .collect()
    }

    fn to_tile(lines: Vec<&str>) -> Tile {
        parse(&lines.into_iter().map(|s| s.to_string()).collect::<Vec<_>>())
    }

    #[test]
    fn test_vflip() {
        let tile = Tile {
            id: 0,
            tile: vec![
                vec!['1', '2', '3'],
                vec!['4', '5', '6'],
                vec!['7', '8', '9'],
            ],
        };

        assert_eq!(tile.vflip().tile, vec![
            vec!['3', '2', '1'],
            vec!['6', '5', '4'],
            vec!['9', '8', '7'],
        ]);
    }

    #[test]
    fn test_hflip() {
        let tile = Tile {
            id: 0,
            tile: vec![
                vec!['1', '2', '3'],
                vec!['4', '5', '6'],
                vec!['7', '8', '9'],
            ],
        };

        assert_eq!(tile.hflip().tile, vec![
            vec!['7', '8', '9'],
            vec!['4', '5', '6'],
            vec!['1', '2', '3'],
        ]);
    }

    #[test]
    fn test_cw() {
        let tile = Tile {
            id: 0,
            tile: vec![
                vec!['1', '2', '3'],
                vec!['4', '5', '6'],
                vec!['7', '8', '9'],
            ],
        };

        assert_eq!(tile.cw().tile, vec![
            vec!['7', '4', '1'],
            vec!['8', '5', '2'],
            vec!['9', '6', '3'],
        ]);
    }

    #[test]
    fn test_ccw() {
        let tile = Tile {
            id: 0,
            tile: vec![
                vec!['1', '2', '3'],
                vec!['4', '5', '6'],
                vec!['7', '8', '9'],
            ],
        };

        assert_eq!(tile.ccw().tile, vec![
            vec!['3', '6', '9'],
            vec!['2', '5', '8'],
            vec!['1', '4', '7'],
        ]);
    }

    #[test]
    fn test_hfit() {
        let tile1 = Tile {
            id: 0,
            tile: vec![
                vec!['a', 'b', 'x'],
                vec!['c', 'd', 'x'],
                vec!['e', 'f', 'x'],
            ],
        };

        let tile2 = Tile {
            id: 0,
            tile: vec![
                vec!['x', 'b', 'u'],
                vec!['x', 'd', 'v'],
                vec!['x', 'f', 'w'],
            ],
        };

        assert!(tile2.hfit(&tile1));
    }

    #[test]
    fn test_vfit() {
        let tile1 = Tile {
            id: 0,
            tile: vec![
                vec!['x', 'x', 'x'],
                vec!['E', 'D', 'A'],
                vec!['F', 'C', 'B'],
            ],
        };

        let tile2 = Tile {
            id: 0,
            tile: vec![
                vec!['1', 'b', 'u'],
                vec!['2', 'd', 'v'],
                vec!['x', 'x', 'x'],
            ],
        };

        assert!(tile1.vfit(&tile2));
    }

    #[test]
    fn test_pack() {
        assert_eq!(pack(&to_vec("..#..#..##"), '#'), 0b010010011);
    }

    #[test]
    fn test_edges() {
        let tile = to_tile(vec![
            "Tile 1753:",
            "..#..#..##",
            "##.......#",
            ".#...#....",
            "#.##....##",
            "#....#...#",
            "......#...",
            ".....#....",
            "......#..#",
            "..##...#.#",
            "##.#.#.##.",
        ]);

        assert_eq!(edges(&tile), vec![
            pack(&to_vec("..#..#..##"), '#'), // top
            pack(&to_vec("##.#.#.##."), '#'), // bottom
            pack(&to_vec(".#.##....#"), '#'), // left
            pack(&to_vec("##.##..##."), '#'), // right

            pack(&to_vec_rev("..#..#..##"), '#'),
            pack(&to_vec_rev("##.#.#.##."), '#'),
            pack(&to_vec_rev(".#.##....#"), '#'),
            pack(&to_vec_rev("##.##..##."), '#'),
        ]);
    }

    #[test]
    fn test_align() {
        let tiles = part1();
        let size = (tiles.len() as f64).sqrt().trunc() as usize;

        let index = index(&tiles);
        let count = count(&tiles);

        let corners = corners(&index, &count);
        let aligned = corners.into_iter()
            .map(|id| tiles.iter().find(|t| t.id == id).unwrap())
            .map(|tile| {
                let queue = tiles.iter()
                    .clone()
                    .into_iter()
                    .filter(|t| t.id != tile.id)
                    .cloned()
                    .collect();

                let mut grid = Vec::with_capacity(size * size);
                grid.push(tile.clone());

                align(size, grid, queue, 1, &index, &count)
            })
            .find(|a| !a.is_empty())
            .unwrap_or_default();

        assert_eq!(aligned.into_iter().map(|t| t.id).collect::<Vec<_>>(), vec![
            1171, 1489, 2971,
            2473, 1427, 2729,
            3079, 2311, 1951,
        ]);
    }

    #[test]
    fn test_align2() {
        let tiles = part1();
        let size = (tiles.len() as f64).sqrt().trunc() as usize;

        let index = index(&tiles);
        let count = count(&tiles);

        let corners = corners(&index, &count)
            .into_iter()
            .map(|id| tiles.iter().find(|t| t.id == id).unwrap())
            .cloned()
            .collect::<Vec<_>>();

        let aligned = corners.into_iter()
            .flat_map(|tile| iterate(&tile))
            .map(|tile| {
                let queue = tiles.iter()
                    .filter(|t| t.id != tile.id)
                    .cloned()
                    .collect::<Vec<_>>();

                let grid = vec![tile];

                align2(size, grid, queue, 1)
            })
            .filter(|v| !v.is_empty())
            .next()
            .unwrap_or_default();

        assert_eq!(aligned.into_iter().map(|t| t.id).collect::<Vec<_>>(), vec![
            1171, 1489, 2971,
            2473, 1427, 2729,
            3079, 2311, 1951,
        ]);
    }

    #[test]
    fn test_corners() {
        let tiles = part1();
        let index = index(&tiles);
        let count = count(&tiles);

        let corners = {
            let mut cs = corners(&index, &count);
            cs.sort();
            cs
        };
        assert_eq!(corners, vec![1171, 1951, 2971, 3079]);
    }

    #[test]
    fn test_pattern() {
        let pattern = vec![
            "                  # ",
            "#    ##    ##    ###",
            " #  #  #  #  #  #   ",
        ].into_iter()
            .map(|row| row.chars().collect())
            .collect();

        let tile = Tile { id: 0, tile: part2() };

        let n = iterate(&tile)
            .iter()
            .map(|tile| lookup(&tile.tile, &pattern))
            .max()
            .unwrap();
        assert_eq!(n, 2);
    }

    #[test]
    fn test_pattern_tiny() {
        let pattern = vec![
            "                  # ",
            "#    ##    ##    ###",
            " #  #  #  #  #  #   ",
        ].into_iter()
            .map(|row| row.chars().collect())
            .collect();

        let tile = vec![
            "                  # ",
            "                  # ",
            "#....##....##....###",
            " #  #  #  #  #  #   ",
            " #  #  #  #  #  #   ",
        ].into_iter()
            .map(|row| row.chars().collect())
            .collect();

        assert_eq!(lookup(&tile, &pattern), 1);
    }

    #[test]
    fn test_pattern_trivial() {
        let pattern = vec![
            "                  # ",
            "#    ##    ##    ###",
            " #  #  #  #  #  #   ",
        ].into_iter()
            .map(|row| row.chars().collect())
            .collect();
        assert_eq!(lookup(&pattern, &pattern), 1);
    }

    fn part1() -> Vec<Tile> {
        let lines =
r#"Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###..."#;
        input(lines.lines().map(|s| s.to_string()).collect())
    }

    fn part2() -> Vec<Vec<char>> {
        let raw =
r#".#.#..#.##...#.##..#####
###....#.#....#..#......
##.##.###.#.#..######...
###.#####...#.#####.#..#
##.#....#.##.####...#.##
...########.#....#####.#
....#..#...##..#.#.###..
.####...#..#.....#......
#..#.##..#..###.#.##....
#.####..#.####.#.#.###..
###.#.#...#.######.#..##
#.####....##..########.#
##..##.#...#...#.#.#.#..
...#..#..#.#.##..###.###
.#.#....#.##.#...###.##.
###.#...#..#.##.######..
.#.#.###.##.##.#..#.##..
.####.###.#...###.#..#.#
..#.#..#..#.#.#.####.###
#..####...#.#.#.###.###.
#####..#####...###....##
#.##..#..#...#..####...#
.#.###..##..##..####.##.
...###...##...#...#..###"#;
        raw.split("\n").into_iter().map(|row| row.chars().collect()).collect()
    }
}
