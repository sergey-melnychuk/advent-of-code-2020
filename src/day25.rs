use crate::utils::lines;


const D: u64 = 20201227;

fn pk(subject: u64, loop_size: usize) -> u64 {
    let mut value: u64 = 1;
    for _ in 0..loop_size {
        value *= subject;
        value %= D;
    }
    value
}

fn find_loop(subject: u64, key: u64) -> usize {
    let mut value: u64 = 1;

    let mut count = 0;
    loop {
        count += 1;
        value *= subject;
        value %= D;
        if value == key {
            break;
        }
    }

    assert_eq!(pk(subject, count), key);
    count
}

pub fn main() {
    let mut it = lines().into_iter();
    let door_pk: u64 = it.next().unwrap().parse().unwrap();
    let card_pk: u64 = it.next().unwrap().parse().unwrap();

    let door_loop = find_loop(7, door_pk);
    let card_loop = find_loop(7, card_pk);

    let secret1 = pk(door_pk,card_loop);
    let secret2 = pk(card_pk, door_loop);
    assert_eq!(secret1, secret2);

    println!("{}", secret1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_pk() {
        assert_eq!(pk(7, 8), 5764801);
    }

    #[test]
    fn test_door_pk() {
        assert_eq!(pk(7, 11), 17807724);
    }

    #[test]
    fn text_example_part1() {
        assert_eq!(pk(17807724, 8), 14897079);
        assert_eq!(pk(5764801, 11), 14897079);
    }
}
