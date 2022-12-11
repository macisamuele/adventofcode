use helpers::input_lines;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::VecDeque;
use std::str::FromStr;

const INPUT: &str = include_str!("../input.txt");

#[derive(Clone, Debug)]
enum Operation {
    SumValue(u64),
    MultiplyValue(u64),
    Square,
}

impl FromStr for Operation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref OPERATION_REGEX: Regex =
                Regex::new(r"^  Operation: new = (old|[0-9]+) ([\+*]) (old|[0-9]+)$")
                    .expect("Regex is correct");
        }

        let matches = OPERATION_REGEX.captures(s).ok_or_else(|| {
            anyhow::anyhow!(
                "Input expected to follow: '{}', found: {}",
                OPERATION_REGEX.as_str(),
                s
            )
        })?;
        let operand1 = matches
            .get(1)
            .expect("As regex matched, group is present")
            .as_str();
        let operator = matches
            .get(2)
            .expect("As regex matched, group is present")
            .as_str();
        let operand2 = matches
            .get(3)
            .expect("As regex matched, group is present")
            .as_str();

        match operator {
            "+" => match (operand1 == "old", operand2 == "old") {
                (true, true) => Ok(Self::MultiplyValue(2)),
                (true, false) => Ok(Self::SumValue(operand2.parse()?)),
                (false, true) => Ok(Self::SumValue(operand1.parse()?)),
                (false, false) => Err(anyhow::anyhow!(
                    "One of the two operans is expected to be old"
                )),
            },
            "*" => match (operand1 == "old", operand2 == "old") {
                (true, true) => Ok(Self::Square),
                (true, false) => Ok(Self::MultiplyValue(operand2.parse()?)),
                (false, true) => Ok(Self::MultiplyValue(operand1.parse()?)),
                (false, false) => Err(anyhow::anyhow!(
                    "One of the two operans is expected to be old"
                )),
            },
            v => Err(anyhow::anyhow!("Unsupported operation {v}")),
        }
    }
}

impl Operation {
    fn result(&self, value: u64, divider: u64) -> u64 {
        match self {
            Self::SumValue(o_value) => (value + o_value) / divider,
            Self::MultiplyValue(o_value) => (value * o_value) / divider,
            Self::Square => (value * value) / divider,
        }
    }
}

#[derive(Clone, Debug)]
struct Monkey {
    id: usize,
    items: VecDeque<u64>,
    operation: Operation,
    divisible_test: u64,
    if_test: usize,
    if_not_test: usize,
}

impl TryFrom<&[String]> for Monkey {
    type Error = anyhow::Error;

    #[allow(clippy::too_many_lines)]
    fn try_from(lines: &[String]) -> Result<Self, Self::Error> {
        if lines.len() == 6 {
            lazy_static! {
                static ref MONKEY_ID_REGEX: Regex =
                    Regex::new(r"Monkey ([0-9]+):$").expect("Regex is correct");
                static ref STARTING_ITEMS_REGEX: Regex =
                    Regex::new(r"^  Starting items: ([0-9]+(, [0-9]+)*)$")
                        .expect("Regex is correct");
                static ref STARTING_ITEMS_PARTS_REGEX: Regex =
                    Regex::new(r"[0-9]+").expect("Regex is correct");
                static ref TEST_REGEX: Regex =
                    Regex::new(r"^  Test: divisible by ([0-9]+)$").expect("Regex is correct");
                static ref POSITIVE_TEST_REGEX: Regex =
                    Regex::new(r"^    If true: throw to monkey ([0-9]+)$")
                        .expect("Regex is correct");
                static ref NEGATIVE_TEST_REGEX: Regex =
                    Regex::new(r"^    If false: throw to monkey ([0-9]+)$")
                        .expect("Regex is correct");
            }
            let id: usize = MONKEY_ID_REGEX
                .captures(&lines[0])
                .ok_or_else(|| {
                    anyhow::anyhow!(
                        "Input expected to follow: '{}', found: {}",
                        MONKEY_ID_REGEX.as_str(),
                        lines[0]
                    )
                })?
                .get(1)
                .expect("As regex matched, group is present")
                .as_str()
                .parse()?;

            let items: VecDeque<u64> = STARTING_ITEMS_PARTS_REGEX
                .find_iter(
                    STARTING_ITEMS_REGEX
                        .captures(&lines[1])
                        .ok_or_else(|| {
                            anyhow::anyhow!(
                                "Input expected to follow: '{}', found: {}",
                                STARTING_ITEMS_REGEX.as_str(),
                                lines[1]
                            )
                        })?
                        .get(1)
                        .expect("As regex matched, group is present")
                        .as_str(),
                )
                .map(|m| m.as_str().parse())
                .collect::<Result<_, _>>()?;

            let operation: Operation = lines[2].parse()?;
            let divisible_test: u64 = TEST_REGEX
                .captures(&lines[3])
                .ok_or_else(|| {
                    anyhow::anyhow!(
                        "Input expected to follow: '{}', found: {}",
                        MONKEY_ID_REGEX.as_str(),
                        lines[3]
                    )
                })?
                .get(1)
                .expect("As regex matched, group is present")
                .as_str()
                .parse()?;

            let if_test = POSITIVE_TEST_REGEX
                .captures(&lines[4])
                .ok_or_else(|| {
                    anyhow::anyhow!(
                        "Input expected to follow: '{}', found: {}",
                        POSITIVE_TEST_REGEX.as_str(),
                        lines[4]
                    )
                })?
                .get(1)
                .expect("As regex matched, group is present")
                .as_str()
                .parse()?;
            let if_not_test = NEGATIVE_TEST_REGEX
                .captures(&lines[5])
                .ok_or_else(|| {
                    anyhow::anyhow!(
                        "Input expected to follow: '{}', found: {}",
                        NEGATIVE_TEST_REGEX.as_str(),
                        lines[5]
                    )
                })?
                .get(1)
                .expect("As regex matched, group is present")
                .as_str()
                .parse()?;

            Ok(Monkey {
                id,
                items,
                operation,
                divisible_test,
                if_test,
                if_not_test,
            })
        } else {
            Err(anyhow::anyhow!(
                "Not enough lines are provided. Expected 6, received {}",
                lines.len()
            ))
        }
    }
}

#[derive(Clone, Debug)]
struct Input {
    monkeys: BTreeMap<usize, Monkey>,
}

impl TryFrom<&Vec<String>> for Input {
    type Error = anyhow::Error;
    fn try_from(lines: &Vec<String>) -> Result<Self, Self::Error> {
        Ok(Self {
            monkeys: (0..lines.len())
                .step_by(7)
                .map(|monkey_start| Monkey::try_from(&lines[monkey_start..monkey_start + 6]))
                .collect::<Result<Vec<_>, _>>()?
                .into_iter()
                .map(|monkey| (monkey.id, monkey))
                .collect(),
        })
    }
}

fn number_of_inspections(
    input: &Input,
    number_of_rounds: usize,
    worry_level_std_divisor: u64,
) -> BTreeMap<usize, u64> {
    let mut input: Input = input.clone();
    let mut number_of_inspections: BTreeMap<usize, u64> = BTreeMap::new();
    let number_of_monkeys = input.monkeys.len();
    let modulo_factor: u64 = input
        .monkeys
        .values()
        .map(|monkey| &monkey.divisible_test)
        .product();

    for _ in 0..number_of_rounds {
        for monkey_id in 0..number_of_monkeys {
            loop {
                let maybe_worry_level_monkey = if let Some(item) = {
                    input
                        .monkeys
                        .get_mut(&monkey_id)
                        .expect("Monkey is present")
                        .items
                        .pop_front()
                } {
                    let worry_level = input.monkeys[&monkey_id]
                        .operation
                        .result(item, worry_level_std_divisor);
                    let next_monkey = if worry_level % input.monkeys[&monkey_id].divisible_test == 0
                    {
                        input.monkeys[&monkey_id].if_test
                    } else {
                        input.monkeys[&monkey_id].if_not_test
                    };
                    Some((worry_level, next_monkey))
                } else {
                    None
                };
                if let Some((worry_level, next_monkey_id)) = maybe_worry_level_monkey {
                    *number_of_inspections.entry(monkey_id).or_default() += 1;
                    input
                        .monkeys
                        .get_mut(&next_monkey_id)
                        .expect("Monkey is present")
                        .items
                        .push_back(worry_level % modulo_factor);
                } else {
                    break;
                }
            }
        }
    }
    number_of_inspections
}

fn part01(input: &Input) -> u64 {
    let inspections: BTreeSet<_> = number_of_inspections(input, 20, 3)
        .values()
        .copied()
        .collect();
    inspections.iter().rev().take(2).product()
}

fn part02(input: &Input) -> u64 {
    let inspections: BTreeSet<_> = number_of_inspections(input, 10000, 1)
        .values()
        .copied()
        .collect();

    inspections.iter().rev().take(2).product()
}

fn main() -> anyhow::Result<()> {
    let input = Input::try_from(&input_lines(INPUT)?)?;

    println!("Part 1: {}", part01(&input));
    println!("Part 2: {}", part02(&input));

    Ok(())
}
