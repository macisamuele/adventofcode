use helpers::input_lines;
use std::collections::{BTreeMap, HashMap, HashSet};

const INPUT: &str = include_str!("../input.txt");

#[derive(Clone, Debug)]
struct Input<'a> {
    allergen_to_ingredients: HashMap<&'a str, HashSet<&'a str>>,
    ingredients: Vec<&'a str>,
}

impl<'a> From<&Vec<&'a str>> for Input<'a> {
    fn from(lines: &Vec<&'a str>) -> Self {
        let mut allergen_to_ingredients: HashMap<&'a str, HashSet<&'a str>> = HashMap::new();
        let mut all_ingredients = Vec::new();

        for line in lines {
            let (column_index, _) = line
                .as_bytes()
                .iter()
                .enumerate()
                .find(|(_, character)| *character == &b'(')
                .unwrap();
            let current_ingredients: HashSet<_> = line[..column_index]
                .split(' ')
                .filter(|ingredient| !ingredient.is_empty())
                .collect();

            line[column_index..]
                .strip_prefix("(contains ")
                .and_then(|s| s.strip_suffix(')'))
                .map(|s| s.split(", "))
                .unwrap()
                .for_each(|allergen| {
                    allergen_to_ingredients
                        .entry(allergen)
                        .and_modify(|ingredients: &mut HashSet<&'a str>| {
                            ingredients.retain(|value| current_ingredients.contains(value))
                        })
                        .or_insert_with(|| current_ingredients.clone());
                });
            all_ingredients.extend(current_ingredients.iter());
        }

        Self {
            allergen_to_ingredients,
            ingredients: all_ingredients,
        }
    }
}

fn part01(input: &Input) -> usize {
    let ingredients_with_allergene: HashSet<_> =
        input.allergen_to_ingredients.values().flatten().collect();

    let ingredients_with_no_allergene_count = input
        .ingredients
        .iter()
        .filter(|ingredient| !ingredients_with_allergene.contains(ingredient))
        .count();

    ingredients_with_no_allergene_count
}

fn part02(input: &Input) -> String {
    let mut cloned_input = input.clone();

    // BTreeMap to avoid explicit sorting <- as it does not change complexity
    let mut allergen_to_ingredient: BTreeMap<&str, &str> = BTreeMap::new();
    let mut used_ingredients: HashSet<&str> = HashSet::new();

    while allergen_to_ingredient.len() != cloned_input.allergen_to_ingredients.len() {
        for (allergen, ingredients) in &cloned_input.allergen_to_ingredients {
            if ingredients.len() == 1 && !allergen_to_ingredient.contains_key(allergen) {
                used_ingredients.extend(ingredients.iter());

                allergen_to_ingredient.insert(allergen, ingredients.iter().next().unwrap());
            }
        }

        for ingredients in &mut cloned_input.allergen_to_ingredients.values_mut() {
            ingredients.retain(|ingredient| !used_ingredients.contains(ingredient));
        }
    }

    allergen_to_ingredient
        .values()
        .cloned()
        .collect::<Vec<_>>()
        .join(",")
}

fn main() -> anyhow::Result<()> {
    let lines = input_lines(INPUT)?;
    let input = Input::from(&lines.iter().map(String::as_str).collect());

    println!("Part 1: {}", part01(&input));
    println!("Part 2: {}", part02(&input));

    Ok(())
}
