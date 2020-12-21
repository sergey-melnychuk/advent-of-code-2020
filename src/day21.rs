use crate::utils::lines;
use std::collections::{HashSet, HashMap};

#[derive(Debug, Eq, PartialEq, Clone)]
struct Food {
    ingredients: Vec<String>,
    allergens: Vec<String>,
}

fn parse(line: &str) -> Food {
    let ing = line.chars()
        .take_while(|c| *c != '(')
        .collect::<String>();

    let ingredients = ing.split(" ")
        .into_iter()
        .filter(|w| !w.is_empty())
        .map(|s| s.to_string())
        .collect();

    let alg = line.chars()
        .skip_while(|c| *c != '(')
        .skip("(contains ".len())
        .take_while(|c| *c != ')')
        .collect::<String>();

    let allergens = alg.split(", ")
        .into_iter()
        .filter(|w| !w.is_empty())
        .map(|s| s.to_string())
        .collect();

    Food {
        ingredients,
        allergens,
    }
}

fn collect<F>(foods: &Vec<Food>, f: F) -> HashSet<String>
    where
        F: Fn(&Food) -> &Vec<String>
{
    foods.iter()
        .flat_map(|food| f(food))
        .cloned()
        .collect()
}

fn count(ingredient: &String, foods: &Vec<Food>) -> usize {
    foods.iter()
        .filter(|food| food.ingredients.contains(ingredient))
        .count()
}

pub fn main() {
    let lines = lines();
    let foods = lines.iter()
        .map(|line| parse(line))
        .collect::<Vec<_>>();

    let ingredients = collect(&foods, |f| &f.ingredients)
        .into_iter().collect::<Vec<_>>();
    let allergens = collect(&foods, |f| &f.allergens)
        .into_iter().collect::<Vec<_>>();

    let a = a(&foods, &allergens);
    let b = b(&a);
    let c = c(&b, &ingredients);

    let n = c.iter()
        .map(|ing| count(ing, &foods))
        .sum::<usize>();
    println!("{}", n); // 2595

    let d = d(&a, &c);
    let list = d.iter()
        .map(|(i, _)| i)
        .cloned()
        .collect::<Vec<_>>()
        .join(",");
    println!("{}", list);
}

fn a(foods: &Vec<Food>, allergens: &Vec<String>) -> HashMap<String, HashSet<String>> {
    fn select(foods: &Vec<Food>, allergen: &String) -> Vec<Food> {
        foods.iter()
            .filter(|food| food.allergens.contains(allergen))
            .cloned()
            .collect()
    }

    let mut result: HashMap<String, HashSet<String>> = HashMap::new();

    allergens.iter()
        .for_each(|alg| {
            let selected = select(foods, alg);
            selected.iter()
                .for_each(|food| {
                    let ingredients = food.ingredients.iter().cloned().collect::<HashSet<_>>();
                    result.entry(alg.clone())
                        .and_modify(|set| {
                            *set = set.intersection(&ingredients).cloned().collect();
                        })
                        .or_insert(ingredients);
                })
        });

    result
}

fn b(a: &HashMap<String, HashSet<String>>) -> HashSet<String> {
    a.values()
        .fold(HashSet::new(), |acc, set| acc.union(set).cloned().collect())
}

fn c(b: &HashSet<String>, ingredients: &Vec<String>) -> HashSet<String> {
    ingredients.iter()
        .filter(|i| !b.contains(*i))
        .cloned()
        .collect()
}

fn d(a: &HashMap<String, HashSet<String>>, c: &HashSet<String>) -> Vec<(String, String)> {
    let mut a = a.into_iter()
        .map(|(k, v)| {
            let d = v.difference(c).cloned().collect::<HashSet<_>>();
            (k.clone(), d)
        })
        .collect::<HashMap<_, _>>();

    assert!(a.values().any(|set| set.len() == 1));

    fn find(a: &mut HashMap<String, HashSet<String>>) -> (String, String) {
        let (k, v) = a.iter().find(|(_, v)| v.len() == 1)
            .map(|(k, set)| (k.clone(), set.iter().next().cloned().unwrap()))
            .unwrap();
        a.remove(&k);
        a.values_mut().for_each(|set| {
            set.remove(&v);
        });
        (k, v)
    }

    let mut acc: Vec<(String, String)> = Vec::new();
    while !a.is_empty() {
        let (k, v) = find(&mut a);
        acc.push((v, k));
    }

    acc.sort_by_key(|(_, k)| k.clone());
    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(parse("mxmxvkd kfcds sqjhc nhms (contains dairy, fish)"), Food {
            ingredients: vec!["mxmxvkd", "kfcds", "sqjhc", "nhms"]
                .iter().map(|s| s.to_string()).collect(),
            allergens: vec!["dairy", "fish"]
                .iter().map(|s| s.to_string()).collect(),
        });

        assert_eq!(parse("trh fvjkl sbzzf mxmxvkd (contains dairy)"), Food {
            ingredients: vec!["trh", "fvjkl", "sbzzf", "mxmxvkd"]
                .iter().map(|s| s.to_string()).collect(),
            allergens: vec!["dairy"]
                .iter().map(|s| s.to_string()).collect(),
        });
    }

    #[test]
    fn test_c() {
        let (lines, expected, _) = example();
        let foods = lines.into_iter()
            .map(|line| parse(&line))
            .collect::<Vec<_>>();

        let ingredients = collect(&foods, |f| &f.ingredients)
            .into_iter().collect::<Vec<_>>();
        let allergens = collect(&foods, |f| &f.allergens)
            .into_iter().collect::<Vec<_>>();

        let a = a(&foods, &allergens);
        let b = b(&a);
        let mut c = c(&b, &ingredients).into_iter().collect::<Vec<_>>();
        c.sort();
        assert_eq!(c, expected);
    }

    #[test]
    fn test_d() {
        let (lines, _, expected) = example();
        let foods = lines.into_iter()
            .map(|line| parse(&line))
            .collect::<Vec<_>>();

        let ingredients = collect(&foods, |f| &f.ingredients)
            .into_iter().collect::<Vec<_>>();
        let allergens = collect(&foods, |f| &f.allergens)
            .into_iter().collect::<Vec<_>>();

        let a = a(&foods, &allergens);
        let b = b(&a);
        let c = c(&b, &ingredients);

        let d = d(&a, &c);
        assert_eq!(d, expected);
    }

    fn example() -> (Vec<String>, Vec<String>, Vec<(String, String)>) {
        let lines = vec![
            "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)",
            "trh fvjkl sbzzf mxmxvkd (contains dairy)",
            "sqjhc fvjkl (contains soy)",
            "sqjhc mxmxvkd sbzzf (contains fish)",
        ];

        let mut expected1 = vec!["kfcds", "nhms", "sbzzf", "trh"];
        expected1.sort();

        let expected2 = vec![
            ("mxmxvkd".to_string(), "dairy".to_string()),
            ("sqjhc".to_string(), "fish".to_string()),
            ("fvjkl".to_string(), "soy".to_string()),
        ];

        (
            lines.into_iter().map(|s| s.to_string()).collect(),
            expected1.into_iter().map(|s| s.to_string()).collect(),
            expected2
        )
    }
}
