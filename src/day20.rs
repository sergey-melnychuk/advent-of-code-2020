use crate::utils::lines;
use std::collections::HashMap;

#[derive(Clone)]
struct Tile {
    id: u64,
    tile: Vec<Vec<char>>,
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

fn adj(size: usize, grid: &Vec<u64>, idx: usize, index: &HashMap<u64, Vec<u32>>) -> Vec<Vec<u32>> {
    let (row, col) = (idx / size, idx % size);

    let mut result = Vec::with_capacity(2);
    if col > 0 {
        let id = grid.get(idx - 1).unwrap();
        let edges = index.get(id).unwrap();
        result.push(edges.clone());
    }
    if row > 0 {
        let id = grid.get(idx - size).unwrap();
        let edges = index.get(id).unwrap();
        result.push(edges.clone());
    }
    result
}

fn align(size: usize, grid: Vec<u64>, queue: Vec<u64>, idx: usize, index: &HashMap<u64, Vec<u32>>, count: &HashMap<u32, u32>) -> Vec<u64> {
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
            let edges = index.get(tile).unwrap();
            adj.iter()
                .all(|es| es.iter().any(|e| edges.contains(e)))
        })
        .cloned()
        .collect::<Vec<_>>();
    let next = if !is_edge {
        next
    } else {
        next.into_iter()
            .filter(|id| {
                let edges = index.get(id).unwrap();
                edges.iter()
                    .any(|e| *count.get(e).unwrap() == 1)
            })
            .collect()
    };
    //println!("\tnext={:?}", next);

    next.into_iter()
        .map(|tile| {
            let mut g = grid.clone();
            g.push(tile);

            let rest = queue.iter()
                .filter(|t| **t != tile)
                .cloned()
                .collect::<Vec<_>>();

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

fn join(aligned: &Vec<u64>, tiles: &Vec<Tile>) -> Vec<Tile> {
    aligned.iter()
        .map(|id| tiles.iter().find(|tile| tile.id == *id).unwrap())
        .cloned()
        .map(crop)
        .collect()
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
        .map(|tile| {
            let mut grid = Vec::with_capacity(size * size);
            grid.push(tile);
            let queue = tiles.iter()
                .filter(|t| t.id != tile)
                .cloned()
                .map(|t| t.id)
                .collect::<Vec<_>>();

            align(size, grid, queue, 1, &index, &count)
        })
        .find(|a| !a.is_empty())
        .unwrap_or_default();
    assert_eq!(aligned.len(), tiles.len());

    // let joined = join(&aligned, &tiles);
    // TODO align each tile in the alignment
    // TODO merge tiles into large single tile
    // TODO search for pattern (flip/rotate if necessary)
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
            .map(|tile| {
                let mut grid = Vec::with_capacity(size * size);
                grid.push(tile);
                let queue = tiles.iter()
                    .map(|tile| tile.id)
                    .clone()
                    .into_iter()
                    .filter(|t| *t != tile)
                    .collect();

                align(size, grid, queue, 1, &index, &count)
            })
            .find(|a| !a.is_empty())
            .unwrap_or_default();

        assert_eq!(aligned, vec![
            1171, 1489, 2971,
            2473, 1427, 2729,
            3079, 2311, 1951,
        ]);
        //
        // vec![
        //     1951, 2311, 3079,
        //     2729, 1427, 2473,
        //     2971, 1489, 1171,
        // ]
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
}
