use crate::utils::lines;

pub fn input() -> Vec<Vec<char>> {
    lines().into_iter()
        .map(|line| line.chars().collect())
        .collect()
}

enum Cell {
    Tree,
    Empty
}

impl Cell {
    fn check(chr: char) -> Self {
        if chr == '#' {
            Self::Tree
        } else {
            Self::Empty
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Point(pub usize, pub usize);

impl Point {
    pub fn move_by(&mut self, dx: usize, dy: usize) {
        self.0 += dx;
        self.1 += dy;
    }
}

fn find(field: &Vec<Vec<char>>, point: &Point) -> Cell {
    let (x, y) = {
        let Point(mut x, y) = point;
        let _rows = field.len();
        let cols = field[0].len();
        if x >= cols {
            x = x % cols;
        }
        (x, *y)
    };

    let chr = field[y][x];
    Cell::check(chr)
}

pub fn count_trees(field: &Vec<Vec<char>>, mut point: Point, (dx, dy): (usize, usize)) -> usize {
    let mut trees: usize = 0;
    while point.1 < field.len() {
        match find(&field, &point) {
            Cell::Tree => {
                trees += 1;
            },
            _ => ()
        }
        point.move_by(dx, dy);
    }
    trees
}

pub fn main() {
    let field: Vec<Vec<char>> = input();
    let trees = count_trees(&field, Point(0,0), (3, 1));
    println!("{}", trees);
}
