use crate::utils::lines;

#[allow(dead_code)] // TODO remove
pub fn main() {
    println!("{}", lines().join("\n"));
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)] // TODO remove
    use super::*;

    #[test]
    fn test_() {
        //
    }
}

/*
fn main() {
    advent_of_code_2020::day00::main();
}
*/
