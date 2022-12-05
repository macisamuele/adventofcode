use helpers::input_lines;
use std::collections::BTreeSet;
use std::str::FromStr;

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug)]
struct Rucksack {
    first_compartment: BTreeSet<u8>,
    second_compartment: BTreeSet<u8>,
}

impl FromStr for Rucksack {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() % 2 == 0 {
            let first_compartment = s[..s.len() / 2].bytes().collect();
            let second_compartment = s[s.len() / 2..].bytes().collect();
            Ok(Self {
                first_compartment,
                second_compartment,
            })
        } else {
            Err(anyhow::anyhow!(
                "Not even number of elements ({}) in {}",
                s.len(),
                s
            ))
        }
    }
}

fn part01(rucksacks: &[Rucksack]) -> usize {
    rucksacks
        .iter()
        .map(|rucksack| {
            rucksack
                .first_compartment
                .intersection(&rucksack.second_compartment)
                .map(|common_item| match common_item {
                    b'a'..=b'z' => usize::from(common_item - b'a' + 1),
                    b'A'..=b'Z' => usize::from(common_item - b'A' + 27),
                    _ => panic!("Item {common_item} is not recognisable: {rucksack:?}"),
                })
                .sum::<usize>()
        })
        .sum()
}

fn part02(rucksacks: &[Rucksack]) -> usize {
    rucksacks
        .chunks(3)
        .map(|rucksacks_in_group| {
            rucksacks_in_group
                .iter()
                .map(|rucksack| {
                    rucksack
                        .first_compartment
                        .union(&rucksack.second_compartment)
                        .copied()
                        .collect()
                })
                .fold(None, |maybe_acc: Option<BTreeSet<u8>>, value| {
                    if let Some(acc) = maybe_acc {
                        Some(acc.intersection(&value).copied().collect())
                    } else {
                        Some(value)
                    }
                })
                .map_or(0, |common_items| {
                    common_items
                        .iter()
                        .map(|common_item| match common_item {
                            b'a'..=b'z' => usize::from(common_item - b'a' + 1),
                            b'A'..=b'Z' => usize::from(common_item - b'A' + 27),
                            _ => panic!(
                                "Item {common_item} is not recognisable: {rucksacks_in_group:?}"
                            ),
                        })
                        .sum::<usize>()
                })
        })
        .sum::<usize>()
}

fn main() -> anyhow::Result<()> {
    let rucksacks: Vec<Rucksack> = input_lines(INPUT)?
        .iter()
        .map(|line| line.parse())
        .collect::<Result<_, _>>()?;

    println!("Part 1: {}", part01(&rucksacks));
    println!("Part 2: {}", part02(&rucksacks));
    Ok(())
}
