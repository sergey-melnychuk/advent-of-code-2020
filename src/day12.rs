use crate::utils::lines;

#[derive(Debug, Eq, PartialEq)]
enum Dir {
    N, S, W, E
}

impl Dir {
    fn reverse(&self) -> Self {
        match self {
            Dir::N => Dir::S,
            Dir::S => Dir::N,
            Dir::W => Dir::E,
            Dir::E => Dir::W,
        }
    }

    fn turn(&self, rot: Rot) -> Self {
        match (self, rot) {
            (Dir::N, Rot::L) => Dir::W,
            (Dir::N, Rot::R) => Dir::E,
            (Dir::S, Rot::L) => Dir::E,
            (Dir::S, Rot::R) => Dir::W,
            (Dir::E, Rot::L) => Dir::N,
            (Dir::E, Rot::R) => Dir::S,
            (Dir::W, Rot::L) => Dir::S,
            (Dir::W, Rot::R) => Dir::N,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Rot {
    L, R
}

// X: west > 0, east < 0
// Y: north > 0, south < 0
#[derive(Debug, Eq, PartialEq)]
enum Op {
    Move(Dir, i64),
    Turn(Rot, i64),
    Fwd(i64),
}

#[derive(Debug, Eq, PartialEq)]
struct Pos(i64, i64);

impl Pos {
    fn add(&self, that: Pos) -> Pos {
        Pos(self.0 + that.0, self.1 + that.1)
    }

    fn mul(&self, x: i64) -> Pos {
        Pos(self.0 * x, self.1 * x)
    }

    fn inv(&self) -> Pos {
        Pos(-self.0, -self.1)
    }

    // turn 90 degrees clockwise (right)
    fn cw(&self) -> Pos {
        Pos(self.1, -self.0)
    }

    // turn 90 degrees counter-clockwise (left)
    fn ccw(&self) -> Pos {
        Pos(-self.1, self.0)
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Vehicle {
    pos: Pos,
    dir: Dir,
    way: Pos,
}

impl Vehicle {
    fn handle1(&mut self, op: &Op) {
        match op {
            Op::Move(Dir::N, d) => self.pos.1 += *d,
            Op::Move(Dir::S, d) => self.pos.1 -= *d,
            Op::Move(Dir::E, d) => self.pos.0 += *d,
            Op::Move(Dir::W, d) => self.pos.0 -= *d,
            Op::Turn(_, 180) => self.dir = self.dir.reverse(),
            Op::Turn(Rot::L, d) if *d < 180 => self.dir = self.dir.turn(Rot::L),
            Op::Turn(Rot::L, _) => self.dir = self.dir.turn(Rot::R),
            Op::Turn(Rot::R, d) if *d < 180 => self.dir = self.dir.turn(Rot::R),
            Op::Turn(Rot::R, _) => self.dir = self.dir.turn(Rot::L),
            Op::Fwd(d) => {
                match self.dir {
                    Dir::N => self.pos.1 += *d,
                    Dir::S => self.pos.1 -= *d,
                    Dir::E => self.pos.0 += *d,
                    Dir::W => self.pos.0 -= *d
                }
            }
        }
    }

    fn handle2(&mut self, op: &Op) {
        match op {
            Op::Move(Dir::N, d) => self.way.1 += *d,
            Op::Move(Dir::S, d) => self.way.1 -= *d,
            Op::Move(Dir::E, d) => self.way.0 += *d,
            Op::Move(Dir::W, d) => self.way.0 -= *d,
            Op::Turn(Rot::L,  90) => self.way = self.way.ccw(),
            Op::Turn(Rot::L, 270) => self.way = self.way.cw(),
            Op::Turn(Rot::R,  90) => self.way = self.way.cw(),
            Op::Turn(Rot::R, 270) => self.way = self.way.ccw(),
            Op::Turn(_, 180) => self.way = self.way.inv(),
            Op::Fwd(n) => self.pos = self.pos.add(self.way.mul(*n)),
            _ => unreachable!()
        }
    }

    fn dist(&self) -> i64 {
        self.pos.0.abs() + self.pos.1.abs()
    }
}

impl Default for Vehicle {
    fn default() -> Self {
        Self {
            pos: Pos(0, 0),
            dir: Dir::E,
            way: Pos(10, 1),
        }
    }
}

fn parse_op(line: &str) -> Op {
    let mut chars = line.chars();
    let chr = chars.next().unwrap();
    let num: i64 = chars.as_str().parse().unwrap();

    match chr {
        'N' => Op::Move(Dir::N, num),
        'S' => Op::Move(Dir::S, num),
        'W' => Op::Move(Dir::W, num),
        'E' => Op::Move(Dir::E, num),
        'L' => Op::Turn(Rot::L, num),
        'R' => Op::Turn(Rot::R, num),
        'F' => Op::Fwd(num),
        _ => unreachable!()
    }
}

fn input() -> Vec<Op> {
    lines().iter()
        .map(|line| parse_op(line))
        .collect()
}

pub fn main() {
    let ops = input();

    let d1 = ops.iter()
        .fold(Vehicle::default(), |mut v, op| {
            v.handle1(op);
            v
        })
        .dist();
    println!("{}", d1);

    let d2 = ops.iter()
        .fold(Vehicle::default(), |mut v, op| {
            v.handle2(op);
            v
        })
        .dist();
    println!("{}", d2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let cases = vec![
            ("F10", Op::Fwd(10)),
            ("N42", Op::Move(Dir::N, 42)),
            ("S43", Op::Move(Dir::S, 43)),
            ("W44", Op::Move(Dir::W, 44)),
            ("E45", Op::Move(Dir::E, 45)),
            ("L90", Op::Turn(Rot::L, 90)),
            ("R30", Op::Turn(Rot::R, 30)),
        ];

        for (s, op) in cases {
            assert_eq!(parse_op(s), op);
        }
    }

    #[test]
    fn test_handle1() {
        let moves: Vec<(Op, Pos)> = vec![
            (Op::Fwd(10),           Pos(10, 0)),
            (Op::Move(Dir::N, 3),   Pos(10, 3)),
            (Op::Fwd(7),            Pos(17, 3)),
            (Op::Turn(Rot::R, 90),  Pos(17, 3)),
            (Op::Fwd(11),           Pos(17, -8)),
        ];
        let mut vehicle = Vehicle::default();

        for (op, pos) in moves {
            vehicle.handle1(&op);
            assert_eq!(vehicle.pos, pos);
        }
    }

    #[test]
    fn test_handle2() {
        let moves: Vec<(Op, Pos, Pos)> = vec![
            (Op::Fwd(10),           Pos(100,  10),   Pos(10,   1)),
            (Op::Move(Dir::N, 3),   Pos(100,  10),   Pos(10,   4)),
            (Op::Fwd(7),            Pos(170,  38),   Pos(10,   4)),
            (Op::Turn(Rot::R, 90),  Pos(170,  38),   Pos( 4, -10)),
            (Op::Fwd(11),           Pos(214, -72),   Pos( 4, -10)),
        ];
        let mut vehicle = Vehicle::default();

        for (op, pos, way) in moves {
            vehicle.handle2(&op);
            assert_eq!(vehicle.pos, pos);
            assert_eq!(vehicle.way, way);
        }
    }

    #[test]
    fn test_dist1() {
        let ops = vec![
            Op::Fwd(10),
            Op::Move(Dir::N, 3),
            Op::Fwd(7),
            Op::Turn(Rot::R, 90),
            Op::Fwd(11),
        ];

        let vehicle = ops.iter()
            .fold(Vehicle::default(), |mut v, op| {
                v.handle1(op);
                v
            });
        assert_eq!(vehicle.dist(), 25);
    }

    #[test]
    fn test_dist2() {
        let ops = vec![
            Op::Fwd(10),
            Op::Move(Dir::N, 3),
            Op::Fwd(7),
            Op::Turn(Rot::R, 90),
            Op::Fwd(11),
        ];

        let vehicle = ops.iter()
            .fold(Vehicle::default(), |mut v, op| {
                v.handle2(op);
                v
            });
        assert_eq!(vehicle.dist(), 286);
    }
}
