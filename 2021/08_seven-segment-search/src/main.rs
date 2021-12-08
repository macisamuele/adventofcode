use helpers::input_lines;
use std::str::FromStr;

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug)]
struct InputEntry {
    one_representation: Vec<char>,
    four_representation: Vec<char>,
    four_displays: [Vec<char>; 4],
}

impl FromStr for InputEntry {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((combinations, displays)) = s.split_once('|') {
            const EMPTY: Vec<char> = vec![];
            let (one_representation, four_representation) = {
                let (maybe_one_representation, maybe_four_representation) =
                    combinations.split_whitespace().fold(
                        (None, None),
                        move |(maybe_one_representation, maybe_four_representation), segments| {
                            let values: Vec<char> = segments.chars().collect();

                            match values.len() {
                                2 => (Some(values), maybe_four_representation),
                                4 => (maybe_one_representation, Some(values)),
                                _ => (maybe_one_representation, maybe_four_representation),
                            }
                        },
                    );

                if let Some(one_representation) = maybe_one_representation {
                    if let Some(four_representation) = maybe_four_representation {
                        Ok((one_representation, four_representation))
                    } else {
                        Err(anyhow::anyhow!(
                            "Representation of `4` not found in the input"
                        ))
                    }
                } else {
                    Err(anyhow::anyhow!(
                        "Representation of `1` not found in the input"
                    ))
                }
            }?;

            let four_displays: [Vec<char>; 4] = {
                let segments_vec: Vec<Vec<char>> = displays
                    .split_whitespace()
                    .map(|segments| segments.chars().collect())
                    .collect();
                anyhow::ensure!(segments_vec.len() == 4);

                let mut segments: [Vec<char>; 4] = [EMPTY; 4];
                segments.clone_from_slice(&segments_vec);

                segments
            };

            Ok(Self {
                one_representation,
                four_representation,
                four_displays,
            })
        } else {
            Err(anyhow::anyhow!("Invalid input: expected `|` to be present"))
        }
    }
}

impl InputEntry {
    fn to_number(&self) -> Result<usize, anyhow::Error> {
        Ok(self
            .digits()?
            .iter()
            .fold(0, |result, value| result * 10 + *value as usize))
    }

    fn digits(&self) -> Result<[u8; 4], anyhow::Error> {
        let mut result = [0; 4];
        for (index, display_segments) in self.four_displays.iter().enumerate() {
            let number = match display_segments.len() {
                2 => 1,
                4 => 4,
                3 => 7,
                7 => 8,
                5 => {
                    match display_segments.iter().filter(|segment| self.four_representation.contains(segment)).count() {
                        2 => 2,
                        3 => {
                            match display_segments.iter().filter(|segment| self.one_representation.contains(segment)).count() {
                                1 => 5,
                                2 => 3,
                                val => anyhow::bail!(
                                    "It shuld not be possible to have a 5 segments digits matching which overlaps for {val} segments with a 1 representation",
                                    val = val,
                                ),
                            }
                        },
                        val => anyhow::bail!(
                            "It shuld not be possible to have a 5 segments digits matching which overlaps for {val} segments with a 4 representation",
                            val = val,
                        )
                    }
                }
                6 => {
                    match display_segments.iter().filter(|segment| self.four_representation.contains(segment)).count() {
                        3 => {
                            match display_segments.iter().filter(|segment| self.one_representation.contains(segment)).count() {
                                1 => 6,
                                2 => 0,
                                val => anyhow::bail!(
                                    "It shuld not be possible to have a 6 segments digits matching which overlaps for {val} segments with a 1 representation",
                                    val = val,
                                ),
                            }
                        },
                        4 => 9,
                        val => anyhow::bail!(
                            "It shuld not be possible to have a 6 segments digits matching which overlaps for {val} segments with a 4 representation",
                            val = val,
                        )
                    }
                }
                val => anyhow::bail!(
                    "It should not be possible to have a display with {val} segments on",
                    val = val
                ),
            };
            result[index] = number;
        }

        Ok(result)
    }
}

fn part01(input: &[InputEntry]) -> usize {
    input
        .iter()
        .filter_map(|entry| entry.digits().ok())
        .map(|digits| {
            digits
                .iter()
                .filter(|digit| matches!(digit, 1 | 4 | 7 | 8))
                .count()
        })
        .sum()
}

fn part02(input: &[InputEntry]) -> usize {
    input
        .iter()
        .filter_map(|entry| entry.to_number().ok())
        .sum()
}

fn main() -> anyhow::Result<()> {
    let input: Vec<InputEntry> = input_lines(INPUT)?
        .iter()
        .map(|line| line.parse())
        .collect::<Result<_, _>>()?;
    println!("Part 1: {}", part01(&input));
    println!("Part 2: {}", part02(&input));
    Ok(())
}
