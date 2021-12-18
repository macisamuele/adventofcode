use helpers::input_lines;
use std::iter::Peekable;
use std::str::Chars;
use std::str::FromStr;

const INPUT: &str = include_str!("../input.txt");

#[derive(Clone, Debug)]
struct ValueDepth {
    value: usize,
    depth: usize,
}

// Ideally we could represent the input as a tree
// where the leaf are the literal values and the pairs
// Doing so would be nice, but it ends up being some work
// to get the value to the left/right of a given value
// Considering the writing notation though, we can make
// a strong assumtpion on the input and actually ignore
// the fact that it is a tree (even if this would prevent
// us to rebuild the input string from the data-structure)
// By storing the values into an array/array-like structure
// allows to simplify the identification of left/right
// values of a given value
#[derive(Clone, Debug, Default)]
struct SnailfishNumber {
    value_depths: Vec<ValueDepth>,
}

impl FromStr for SnailfishNumber {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn read_expected(
            characters: &mut Peekable<Chars<'_>>,
            expected_character: char,
        ) -> Result<(), anyhow::Error> {
            match characters.next() {
                Some(value) if value == expected_character => Ok(()),
                Some(value) => Err(anyhow::anyhow!(
                    "Unexpected input. Expected '{expected_character}', received '{value}'",
                    expected_character = expected_character,
                    value = value
                )),
                None => Err(anyhow::anyhow!(
                    "Unexpected input. Expected '{expected_character}', received no character",
                    expected_character = expected_character
                )),
            }
        }

        fn recurse(
            snailfish_number: &mut SnailfishNumber,
            characters: &mut Peekable<Chars<'_>>,
            depth: usize,
        ) -> Result<(), anyhow::Error> {
            if let Some('0'..='9') = characters.peek() {
                let mut value = 0;
                while let Some(character) = characters.next_if(char::is_ascii_digit) {
                    value = value * 10 + (character as usize - '0' as usize);
                }
                let index = snailfish_number.value_depths.len();
                snailfish_number.value_depths.insert(
                    index,
                    ValueDepth {
                        value,
                        depth: depth - 1,
                    },
                );
            } else {
                read_expected(characters, '[')?;
                recurse(snailfish_number, characters, depth + 1)?;
                read_expected(characters, ',')?;
                recurse(snailfish_number, characters, depth + 1)?;
                read_expected(characters, ']')?;
            }
            Ok(())
        }

        let mut number = SnailfishNumber::default();
        recurse(&mut number, &mut s.chars().peekable(), 0)?;
        Ok(number)
    }
}

impl SnailfishNumber {
    fn explode(&mut self) -> bool {
        let index_to_explode =
            self.value_depths
                .iter()
                .enumerate()
                .find_map(
                    |(index, ValueDepth { depth, .. })| {
                        if depth == &4 {
                            Some(index)
                        } else {
                            None
                        }
                    },
                );

        if let Some(index) = index_to_explode {
            if index > 0 {
                self.value_depths[index - 1].value += self.value_depths[index].value;
            }

            if index + 2 < self.value_depths.len() {
                self.value_depths[index + 2].value += self.value_depths[index + 1].value;
            }

            self.value_depths[index] = ValueDepth { value: 0, depth: 3 };

            if index + 1 < self.value_depths.len() {
                self.value_depths.remove(index + 1);
            }

            true
        } else {
            false
        }
    }

    fn split(&mut self) -> bool {
        let index_to_split =
            self.value_depths
                .iter()
                .enumerate()
                .find_map(
                    |(index, ValueDepth { value, .. })| {
                        if value > &9 {
                            Some(index)
                        } else {
                            None
                        }
                    },
                );

        if let Some(index) = index_to_split {
            if let Some(ValueDepth { value, depth }) = self.value_depths.get(index).cloned() {
                self.value_depths[index].value = value / 2;
                self.value_depths[index].depth = depth + 1;
                self.value_depths.insert(
                    index + 1,
                    ValueDepth {
                        depth: depth + 1,
                        value: value / 2 + value % 2,
                    },
                );
                true
            } else {
                // This is odd, it should not have happened
                false
            }
        } else {
            false
        }
    }

    fn reduce(&mut self) {
        loop {
            if !self.explode() && !self.split() {
                break;
            }
        }
    }

    fn add(&mut self, other: &SnailfishNumber) {
        for value_depth in &mut self.value_depths {
            value_depth.depth += 1;
        }

        self.value_depths
            .extend(other.value_depths.iter().map(|value_depth| {
                let mut new_value_depth = value_depth.clone();
                new_value_depth.depth += 1;
                new_value_depth
            }));

        self.reduce();
    }

    fn magnitude(&self) -> usize {
        let mut value_depths = self.value_depths.clone();

        while value_depths.len() > 1 {
            for index in 0..(value_depths.len() - 1) {
                if value_depths[index].depth == value_depths[index + 1].depth {
                    value_depths[index].value =
                        3 * value_depths[index].value + 2 * value_depths[index + 1].value;
                    value_depths[index].depth = value_depths[index].depth.saturating_sub(1);
                    value_depths.remove(index + 1);
                    break;
                }
            }
        }

        value_depths[0].value
    }
}

fn part01(numbers: &[SnailfishNumber]) -> usize {
    if numbers.len() > 2 {
        numbers[1..]
            .iter()
            .fold(numbers[0].clone(), |mut result, number| {
                result.add(number);
                result
            })
            .magnitude()
    } else {
        0
    }
}

fn part02(numbers: &[SnailfishNumber]) -> usize {
    (0..numbers.len())
        .flat_map(|index_1| {
            (0..numbers.len()).filter_map(move |index_2| {
                if index_1 == index_2 {
                    None
                } else {
                    let mut number = numbers[index_1].clone();
                    number.add(&numbers[index_2]);
                    Some(number.magnitude())
                }
            })
        })
        .max()
        .expect("Expected to have at least 1 value")
}

fn main() -> anyhow::Result<()> {
    let numbers: Vec<SnailfishNumber> = input_lines(INPUT)?
        .iter()
        .map(|line| line.parse())
        .collect::<Result<_, _>>()?;

    println!("Part 1: {}", part01(&numbers));
    println!("Part 2: {}", part02(&numbers));

    Ok(())
}
