use std::collections::HashMap;

fn play(input: &[usize], mut goal: usize) -> usize {
    let mut number: usize = 0;
    let mut turn: usize = 1;

    let mut before: HashMap<usize, usize> = HashMap::new();
    let mut spoken: HashMap<usize, usize> = HashMap::new();

    while goal > 0 && turn - 1 < input.len() {
        number = input[turn - 1];
        spoken.insert(number, turn);
        turn += 1;
        goal -= 1;
    }

    while goal > 0 {
        if before.contains_key(&number) {
            // number was spoken before
            let at = *before.get(&number).unwrap();
            number = (turn-1) - at;
        } else {
            number = 0;
        }

        if spoken.contains_key(&number) {
            let at = *spoken.get(&number).unwrap();
            before.insert(number, at);
        }
        spoken.insert(number, turn);

        turn += 1;
        goal -= 1;
    }

    number
}

pub fn main() {
    let answer = play(&[20,9,11,0,1,2], 2020);
    println!("{}", answer);

    let answer = play(&[20,9,11,0,1,2], 30_000_000);
    println!("{}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_play_tiny_init() {
        assert_eq!(play(&[0,3,6], 1), 0);
        assert_eq!(play(&[0,3,6], 2), 3);
        assert_eq!(play(&[0,3,6], 3), 6);
    }

    #[test]
    fn test_play_tiny_turns() {
        assert_eq!(play(&[0,3,6], 4), 0);
        assert_eq!(play(&[0,3,6], 5), 3);
        assert_eq!(play(&[0,3,6], 6), 3);
        assert_eq!(play(&[0,3,6], 7), 1);
        assert_eq!(play(&[0,3,6], 8), 0);
        assert_eq!(play(&[0,3,6], 9), 4);
        assert_eq!(play(&[0,3,6], 10), 0);
    }

    #[test]
    fn test_play_examples() {
        assert_eq!(play(&[0,3,6], 2020),  436);

        assert_eq!(play(&[1,3,2], 2020),    1);
        assert_eq!(play(&[2,1,3], 2020),   10);
        assert_eq!(play(&[1,2,3], 2020),   27);
        assert_eq!(play(&[2,3,1], 2020),   78);
        assert_eq!(play(&[3,2,1], 2020),  438);
        assert_eq!(play(&[3,1,2], 2020), 1836);
    }

    #[test]
    fn test_answers() {
        assert_eq!(play(&[20,9,11,0,1,2],       2020),  1111);
        assert_eq!(play(&[20,9,11,0,1,2], 30_000_000), 48568);
    }
}
