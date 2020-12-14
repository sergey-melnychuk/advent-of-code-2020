use crate::utils::lines;
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Clone)]
enum Op {
    Mask {
        zer: u64,
        one: u64,
        line: String,
    },
    Mem {
        adr: u64,
        val: u64,
    },
}

fn parse(line: &str) -> Op {
    if line.starts_with("mem[") {
        let mut it = line[4..].split("] = ");
        let adr: u64 = it.next().unwrap().parse().unwrap();
        let val: u64 = it.next().unwrap().parse().unwrap();
        Op::Mem { adr, val }
    } else { // "mask = "
        let masked = &line[7..];
        let one = u64::from_str_radix(&masked.replace("X", "0"), 2).unwrap();
        let zer = u64::from_str_radix(&masked.replace("X", "1"), 2).unwrap();
        Op::Mask { zer, one, line: masked.to_string() }
    }
}

fn input() -> Vec<Op> {
    lines()
        .iter()
        .map(|line| parse(line))
        .collect()
}

fn mask1(mask: &Op, adr: u64, val: u64) -> Vec<(u64, u64)> {
    match mask {
        Op::Mask { one, zer, line: _ } => vec![(adr, val & zer | one)],
        _ => vec![(adr, val)]
    }
}

fn expand(line: &str) -> Vec<u64> {
    let len = line.len();
    assert_eq!(len, 36);
    let cap = line.chars()
        .filter(|c| *c == 'X')
        .count();

    fn helper(base: &str, acc: &mut Vec<u64>) {
        if !base.contains("X") {
            let n = u64::from_str_radix(base, 2).unwrap();
            acc.push(n);
        } else {
            helper(&base.replacen("X", "0", 1), acc);
            helper(&base.replacen("X", "1", 1), acc);
        }
    }

    let mut acc = Vec::with_capacity(1usize << (cap - 1));
    helper(line, &mut acc);
    acc
}

fn merge(line: &str, adr: u64) -> String {
    let badr = format!("{:036b}", adr);
    line.chars()
        .zip(badr.chars())
        .map(|(m, a)| match (m, a) {
            ('X', _) => 'X',
            ('1', _) => '1',
            ('0', a) => a,
            _ => unreachable!()
        })
        .collect::<String>()
}

fn mask2(mask: &Op, adr: u64, val: u64) -> Vec<(u64, u64)> {
    match mask {
        Op::Mask { one: _, zer: _, line } => {
            let base = merge(line, adr);
            expand(&base)
                .into_iter()
                .map(|adr| (adr, val))
                .collect()
        },
        _ => vec![(adr, val)]
    }
}


fn exec(ops: &Vec<Op>, f: fn(&Op, u64, u64) -> Vec<(u64, u64)>) -> HashMap<u64, u64> {
    let mut mem: HashMap<u64, u64> = HashMap::new();
    let mut msk: Option<Op> = None;
    for op in ops {
        match (op.to_owned(), msk.as_ref()) {
            (Op::Mem { adr, val }, Some(m)) => {
                f(m, adr, val).into_iter()
                    .for_each(|(a, v)| {
                        let _ = mem.insert(a, v);
                    });
            },
            (m, _) => msk = Some(m)
        }
    }
    mem
}

pub fn main() {
    let ops = input();

    let map = exec(&ops, mask1);
    let sum = map.iter().fold(0u64, |acc, (_, v)| acc + *v);
    println!("{}", sum);

    let map = exec(&ops, mask2);
    let sum = map.iter().fold(0u64, |acc, (_, v)| acc + *v);
    println!("{}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_mem() {
        let op = parse("mem[2953] = 12201");
        assert_eq!(op, Op::Mem {
            adr: 2953,
            val: 12201,
        });
    }

    #[test]
    fn test_parse_mask1() {
        let op = parse("mask = 1000000011XX10X00X0010X1011X111X0111");
        assert_eq!(op, Op::Mask {
            one: 0b100000001100100000001001011011100111u64,
            zer: 0b100000001111101001001011011111110111u64,
            line: "1000000011XX10X00X0010X1011X111X0111".to_string(),
        });
    }

    #[test]
    fn test_parse_mask2() {
        let op = parse("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X");
        assert_eq!(op, Op::Mask {
            //     XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
            zer: 0b111111111111111111111111111111111101u64,
            one: 0b000000000000000000000000000001000000u64,
            line: "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".to_string(),
        });
    }

    #[test]
    fn test_mask1() {
        let m = parse("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X");

        let cases: Vec<(&str, Vec<(u64, u64)>)> = vec![
            ("mem[8] = 11",  vec![(8,  73)]),
            ("mem[7] = 101", vec![(7, 101)]),
            ("mem[8] = 0",   vec![(8,  64)]),
        ];

        for (s, expected) in cases {
            match parse(s) {
                Op::Mem { adr, val } => {
                    assert_eq!(mask1(&m, adr, val), expected);
                },
                _ => unreachable!()
            }
        }
    }

    #[test]
    fn test_part1() {
        let lines = vec![
            "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X",
            "mem[8] = 11",
            "mem[7] = 101",
            "mem[8] = 0",
        ];

        let ops: Vec<Op> = lines.iter().map(|line| parse(*line)).collect();
        let map = exec(&ops, mask1);
        let sum = map.iter().fold(0u64, |acc, (_, v)| acc + *v);

        assert_eq!(sum, 165);
    }

    #[test]
    fn test_expand() {
        assert_eq!(expand("000000000000000000000000000000X1001X"), vec![
            0b000000000000000000000000000000010010u64,
            0b000000000000000000000000000000010011u64,
            0b000000000000000000000000000000110010u64,
            0b000000000000000000000000000000110011u64,
        ]);
    }

    #[test]
    fn test_replacen() {
        assert_eq!("00XXX".replacen("X", "0", 1), "000XX");
    }

    #[test]
    fn test_format() {
        assert_eq!(format!("{:036b}", 0b000000000000000000000000000000010010u64), "000000000000000000000000000000010010");
    }

    #[test]
    fn test_mask2_one() {
        let m = parse("mask = 000000000000000000000000000000X1001X");
        assert_eq!(mask2(&m, 42, 100), vec![
            (26, 100),
            (27, 100),
            (58, 100),
            (59, 100),
        ]);
    }

    #[test]
    fn test_mask2_two() {
        let m = parse("mask = 00000000000000000000000000000000X0XX");
        assert_eq!(mask2(&m, 26, 1), vec![
            (16, 1),
            (17, 1),
            (18, 1),
            (19, 1),
            (24, 1),
            (25, 1),
            (26, 1),
            (27, 1),
        ]);
    }

    #[test]
    fn test_part2() {
        let lines = vec![
            "mask = 000000000000000000000000000000X1001X",
            "mem[42] = 100",
            "mask = 00000000000000000000000000000000X0XX",
            "mem[26] = 1",
        ];

        let ops: Vec<Op> = lines.iter().map(|line| parse(*line)).collect();
        let map = exec(&ops, mask2);
        let sum = map.iter().fold(0u64, |acc, (_, v)| acc + *v);

        assert_eq!(sum, 208);
    }
}
