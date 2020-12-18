use crate::utils::lines;


#[derive(Debug, Clone, Eq, PartialEq)]
enum Token {
    Num(i64),
    Add,
    Mul,
    Open,
    Close,
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Op {
    Mul,
    Add,
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Expr {
    Const(i64),
    Op(Op),
    Group(Vec<Expr>),
}


fn tokenize(line: &str) -> Vec<Token> {
    line.chars()
        .into_iter()
        .filter(|c| *c != ' ')
        .map(|s| match s {
            '+' => Token::Add,
            '*' => Token::Mul,
            '(' => Token::Open,
            ')' => Token::Close,
            n  => Token::Num(n as i64 - '0' as i64)
        })
        .collect()
}

fn group(tokens: &Vec<Token>, mut idx: usize, mut acc: Vec<Expr>) -> (usize, Expr) {
    while idx < tokens.len() {
        let token = tokens[idx].to_owned();
        if token == Token::Close {
            return (idx + 1, Expr::Group(acc));
        }

        if token == Token::Open {
            let (next, expr) = group(tokens, idx + 1, Vec::new());
            acc.push(expr);
            idx = next;
            continue;
        }

        let expr = match token {
            Token::Num(x) => Expr::Const(x),
            Token::Add => Expr::Op(Op::Add),
            Token::Mul => Expr::Op(Op::Mul),
            _ => unreachable!()
        };

        acc.push(expr);
        idx += 1;
    }

    (idx, Expr::Group(acc))
}


#[allow(dead_code)]
fn print(expr: &Expr) -> String {
    match expr {
        Expr::Const(x) => format!("{}", x),
        Expr::Op(Op::Add) => " + ".to_string(),
        Expr::Op(Op::Mul) => " * ".to_string(),
        Expr::Group(items) => {
            format!("({})", items.into_iter()
                .map(|e| print(e))
                .collect::<Vec<_>>()
                .join(""))
        }
    }
}

#[allow(dead_code)]
fn print_vec(expr: &Vec<Expr>) -> String {
    format!("{}", expr.into_iter()
        .map(|e| print(e))
        .collect::<Vec<_>>()
        .join(""))
}

fn parse(tokens: Vec<Token>) -> Expr {
    group(&tokens, 0, Vec::new()).1
}

fn eval1(expr: Expr) -> i64 {
    fn op(a: Expr, op: Expr, b: Expr) -> Expr {
        match (a, op, b) {
            (a, Expr::Op(Op::Add), b) => Expr::Const(eval1(a) + eval1(b)),
            (a, Expr::Op(Op::Mul), b) => Expr::Const(eval1(a) * eval1(b)),
            _ => unreachable!()
        }
    }

    match expr {
        Expr::Const(x) => x,
        Expr::Group(items) if items.len() == 1 => eval1(items[0].to_owned()),
        Expr::Group(mut items) => {
            let head = items.iter().take(3).cloned().collect::<Vec<_>>();
            let (a, x, b) = (head[0].to_owned(), head[1].to_owned(), head[2].to_owned());
            let mut r = op(a, x, b);
            items = items.into_iter().skip(3).collect();
            while items.len() > 1 {
                let next = items.iter().take(2).cloned().collect::<Vec<_>>();
                let (x, b) = (next[0].to_owned(), next[1].to_owned());
                items = items.into_iter().skip(2).collect();
                r = op(r, x, b);
            }
            if items.len() == 1 {
                r = items[0].to_owned();
            }
            eval1(r)
        },
        _ => unreachable!()
    }
}

fn once(items: Vec<Expr>) -> (Vec<Expr>, bool) {
    let opt = items.iter()
        .enumerate()
        .filter(|(_, e)| **e == Expr::Op(Op::Add))
        .filter_map(|(i, _)| {
            match (items.get(i-1).unwrap(), items.get(i+1).unwrap()) {
                (Expr::Const(x), Expr::Const(y)) => Some((i, *x + *y)),
                _ => None
            }
        })
        .next();

    if let Some((index, added)) = opt {
        let mut before = items.iter().take(index-1).cloned().collect::<Vec<_>>();
        let mut after = items.iter().skip(index+2).cloned().collect::<Vec<_>>();

        let mut result = Vec::with_capacity(after.len() + before.len() + 1);
        result.append(&mut before);
        result.push(Expr::Const(added));
        result.append(&mut after);
        (result, false)
    } else {
        let next = items.into_iter()
            .map(|expr| match expr {
                Expr::Group(g) => Expr::Group(reduce(g)),
                e => e
            })
            .collect();
        (next, true)
    }
}

fn done(items: &Vec<Expr>) -> bool {
    items.iter()
        .all(|expr| match expr {
            Expr::Group(xs) => done(xs),
            e => e != &Expr::Op(Op::Add)
        })
}

// reduce all sum operations, e.g.:
// 1 + 2 * 3 + 4 + 5 ->
//   (1 + 2) * (3 + 4) + 5 ->
//     3 * 7 + 5 ->
//       3 * (7 + 5)
//         3 * 12
//           36

// 2 * 3 + (4 * 5)"
//   2 * 3 + (4 * 5)
//     2 * 3 + 20
//       2 * 23
//         46
fn reduce(mut items: Vec<Expr>) -> Vec<Expr> {
    if done(&items) {
        let evaluated = eval1(Expr::Group(items));
        return vec![Expr::Const(evaluated)];
    }

    items = items.into_iter()
        .map(|expr| {
            match expr {
                Expr::Group(es) => Expr::Group(reduce(es)),
                e => e
            }
        })
        .map(|expr| {
            match expr {
                Expr::Group(es) if es.len() == 1 => es[0].to_owned(),
                e => e
            }
        })
        .collect();

    loop {
        let (next, done) = once(items);
        items = next;
        if done {
            break;
        }
    }

    items
}

/// Instead, addition is evaluated before multiplication.
fn eval2(expr: Expr) -> i64 {
    match expr {
        Expr::Const(x) => x,
        Expr::Group(items) if items.len() == 1 => eval2(items[0].to_owned()),
        Expr::Group(items) => eval2(Expr::Group(reduce(items))),
        _ => unreachable!()
    }
}

pub fn main() {
    let expr = lines()
        .into_iter()
        .map(|line| tokenize(&line))
        .map(|tokens| parse(tokens))
        .collect::<Vec<_>>();

    let sum = expr.iter()
        .cloned()
        .map(|expr| eval1(expr))
        .sum::<i64>();
    println!("{}", sum);

    let sum = expr.iter()
        .cloned()
        .map(|expr| eval2(expr))
        .sum::<i64>();
    println!("{}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval1() {
        let cases = vec![
            ("1 + 2 * 3 + 4 * 5 + 6", 71),
            ("2 * 3 + (4 * 5)", 26),
            ("5 + (8 * 3 + 9 + 3 * 4 * 3)", 437),
            ("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 12240),
            ("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 13632),
        ];

        for (s, expected) in cases {
            let result = eval1(parse(tokenize(s)));
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_eval2() {
        let cases = vec![
            ("1 + 2 * 3 + 4 * 5 + 6", 231),
            ("1 + (2 * 3) + (4 * (5 + 6))", 51),
            ("2 * 3 + (4 * 5)", 46),
            ("5 + (8 * 3 + 9 + 3 * 4 * 3)", 1445),
            ("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 669060),
            ("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 23340),
        ];

        for (s, expected) in cases {
            let result = eval2(parse(tokenize(s)));
            assert_eq!(result, expected, "{:?}", s);
        }
    }

    #[test]
    fn test_once1() {
        let expr = match parse(tokenize("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))")) {
            Expr::Group(items) => items,
            _ => unreachable!()
        };

        let (a, _) = once(expr);
        assert_eq!("5 * 9 * (7 * 3 * 12 * 3 + (56))", print_vec(&a));

        let (b, _) = once(a);
        assert_eq!("5 * 9 * (7 * 3 * 12 * 59)", print_vec(&b));

        let (c, _) = once(b);
        assert_eq!("5 * 9 * (14868)", print_vec(&c));

        let (d, done) = once(c);
        assert_eq!("5 * 9 * (14868)", print_vec(&d));
        assert!(done);

        assert_eq!(eval1(Expr::Group(vec![
            Expr::Const(5), Expr::Op(Op::Mul),
            Expr::Const(9), Expr::Op(Op::Mul),
            Expr::Group(vec![Expr::Const(14868)]),
        ])), 669060);
    }

    #[test]
    fn test_reduce1() {
        assert_eq!(reduce(vec![
            Expr::Const(1),
            Expr::Op(Op::Add),
            Expr::Const(3),
            Expr::Op(Op::Mul),
            Expr::Const(8),
        ]), vec![
            Expr::Const(4),
            Expr::Op(Op::Mul),
            Expr::Const(8),
        ]);
    }

    #[test]
    fn test_reduce2() {
        assert_eq!(reduce(vec![
            Expr::Const(2),
            Expr::Op(Op::Mul),
            Expr::Const(3),
        ]), vec![
            Expr::Const(6),
        ]);
    }

    #[test]
    fn test_reduce3() {
        assert_eq!(reduce(vec![
            Expr::Const(1),
            Expr::Op(Op::Mul),
            Expr::Const(2),
            Expr::Op(Op::Add),
            Expr::Const(3),
        ]), vec![
            Expr::Const(1),
            Expr::Op(Op::Mul),
            Expr::Const(5),
        ]);
    }

    #[test]
    fn test_reduce4() {
        // 1 + 2 * 3 + 4 * 5 + 6
        assert_eq!(reduce(vec![
            Expr::Const(1), Expr::Op(Op::Add), Expr::Const(2),
            Expr::Op(Op::Mul),
            Expr::Const(3), Expr::Op(Op::Add), Expr::Const(4),
            Expr::Op(Op::Mul),
            Expr::Const(5), Expr::Op(Op::Add), Expr::Const(6),
        ]), vec![
            Expr::Const(3),
            Expr::Op(Op::Mul),
            Expr::Const(7),
            Expr::Op(Op::Mul),
            Expr::Const(11),
        ]);
    }

    #[test]
    fn test_reduce5() {
        // "2 * 3 + (4 * 5)"
        assert_eq!(reduce(vec![
            Expr::Const(2), Expr::Op(Op::Mul), Expr::Const(3),
            Expr::Op(Op::Add),
            Expr::Group(vec![
                Expr::Const(4),
                Expr::Op(Op::Mul),
                Expr::Const(5),
            ]),
        ]), vec![
            Expr::Const(2), Expr::Op(Op::Mul), Expr::Const(23),
        ]);
    }

    #[test]
    fn test_reduce6() {
        // "5 + (8 * 3 + 9 + 3 * 4 * 3)"
        assert_eq!(reduce(vec![
            Expr::Const(5), Expr::Op(Op::Add),
            Expr::Group(vec![
                Expr::Const(8), Expr::Op(Op::Mul),
                Expr::Const(3), Expr::Op(Op::Add),
                Expr::Const(9), Expr::Op(Op::Add),
                Expr::Const(3), Expr::Op(Op::Mul),
                Expr::Const(4), Expr::Op(Op::Mul),
                Expr::Const(3),
            ]),
        ]), vec![
            Expr::Const(5), Expr::Op(Op::Add),
            Expr::Group(vec![Expr::Const(1440)]),
        ]);
    }

    #[test]
    fn test_reduce7() {
        // 3 + 9 + 3
        assert_eq!(reduce(vec![
            Expr::Const(3), Expr::Op(Op::Add),
            Expr::Const(9), Expr::Op(Op::Add),
            Expr::Const(3),
        ]), vec![
            Expr::Const(15),
        ]);
    }

    #[test]
    fn test_tokenize1() {
        assert_eq!(tokenize("(6 + 5) * 6"), vec![
            Token::Open, Token::Num(6), Token::Add, Token::Num(5), Token::Close,
            Token::Mul, Token::Num(6)
        ]);
    }

    #[test]
    fn test_tokenize2() {
        let tokens = tokenize("(8 + 4 * (2 * 9) + 6 + 6 + 3) + 4");
        assert_eq!(tokens, vec![
            Token::Open,
                Token::Num(8),
                Token::Add,
                Token::Num(4),
                Token::Mul,
                Token::Open,
                    Token::Num(2),
                    Token::Mul,
                    Token::Num(9),
                Token::Close,
                Token::Add, Token::Num(6),
                Token::Add, Token::Num(6),
                Token::Add, Token::Num(3),
            Token::Close,
            Token::Add,
            Token::Num(4),
        ]);
    }

    #[test]
    fn test_group1() {
        let tokens = vec![
            Token::Open, Token::Num(6), Token::Add, Token::Num(5), Token::Close,
            Token::Mul, Token::Num(6)
        ];

        assert_eq!(parse(tokens), Expr::Group(vec![
            Expr::Group(vec![
                Expr::Const(6),
                Expr::Op(Op::Add),
                Expr::Const(5),
            ]),
            Expr::Op(Op::Mul),
            Expr::Const(6),
        ]));
    }

    #[test]
    fn test_group2() {
        let tokens = vec![
            Token::Open,
                Token::Num(8),
                Token::Add,
                Token::Num(4),
                Token::Mul,
                Token::Open,
                    Token::Num(2),
                    Token::Mul,
                    Token::Num(9),
                Token::Close,
                Token::Add, Token::Num(6),
                Token::Add, Token::Num(6),
                Token::Add, Token::Num(3),
            Token::Close,
            Token::Add,
            Token::Num(4),
        ];

        let expr = parse(tokens);

        assert_eq!(expr, Expr::Group(vec![
            Expr::Group(vec![
                Expr::Const(8), Expr::Op(Op::Add), Expr::Const(4), Expr::Op(Op::Mul),
                Expr::Group(vec![
                    Expr::Const(2), Expr::Op(Op::Mul), Expr::Const(9),
                ]),
                Expr::Op(Op::Add), Expr::Const(6),
                Expr::Op(Op::Add), Expr::Const(6),
                Expr::Op(Op::Add), Expr::Const(3),
            ]),
            Expr::Op(Op::Add), Expr::Const(4),
        ]));
    }
}
