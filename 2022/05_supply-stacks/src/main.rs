use helpers::input_lines;
use scan_fmt::scan_fmt;
use std::collections::VecDeque;
use std::convert::TryFrom;
use std::str::FromStr;

const INPUT: &str = include_str!("../input.txt");

#[derive(Clone, Debug)]
struct Move {
    quantity: usize,
    src: usize,
    dst: usize,
}
impl FromStr for Move {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (quantity, src, dst) = scan_fmt!(s, "move {} from {} to {}", usize, usize, usize)?;
        Ok(Self { quantity, src, dst })
    }
}

#[derive(Clone, Debug)]
struct SupplyStacks {
    stacks: Vec<VecDeque<char>>,
}

impl TryFrom<&[String]> for SupplyStacks {
    type Error = anyhow::Error;
    fn try_from(lines: &[String]) -> Result<Self, Self::Error> {
        fn stacks_in_line(line: &str) -> usize {
            (line.len() + 2) / 4
        }
        let number_of_stacks = stacks_in_line(
            lines
                .last()
                .ok_or_else(|| anyhow::anyhow!("expected to have at least one line"))?,
        );

        let mut stacks: Vec<VecDeque<char>> =
            (0..number_of_stacks).map(|_| VecDeque::new()).collect();
        for line in &lines[..lines.len() - 1] {
            for idx in 0..number_of_stacks.min(stacks_in_line(line)) {
                let stack_crate = line[idx * 4 + 1..=idx * 4 + 1].parse::<char>()?;
                if stack_crate != ' ' {
                    stacks[idx].push_front(stack_crate);
                }
            }
        }
        Ok(Self { stacks })
    }
}

impl SupplyStacks {
    fn apply_crate_mover_9000(&mut self, move_: &Move) -> anyhow::Result<()> {
        for _ in 0..move_.quantity {
            let stack_crate = self.stacks[move_.src - 1]
                .pop_back()
                .ok_or_else(|| anyhow::anyhow!("Expected to have found a crate"))?;
            self.stacks[move_.dst - 1].push_back(stack_crate);
        }
        Ok(())
    }

    fn apply_crate_mover_9001(&mut self, move_: &Move) -> anyhow::Result<()> {
        let mut tmp_stack = VecDeque::with_capacity(move_.quantity);

        for _ in 0..move_.quantity {
            tmp_stack.push_back(
                self.stacks[move_.src - 1]
                    .pop_back()
                    .ok_or_else(|| anyhow::anyhow!("Expected to have found a crate"))?,
            );
        }
        while let Some(stack_crate) = tmp_stack.pop_back() {
            self.stacks[move_.dst - 1].push_back(stack_crate);
        }

        Ok(())
    }
}

#[derive(Clone, Debug)]
struct Input {
    supply_stacks: SupplyStacks,
    moves: Vec<Move>,
}

impl TryFrom<&[String]> for Input {
    type Error = anyhow::Error;
    fn try_from(lines: &[String]) -> Result<Self, Self::Error> {
        let (idx, _) = lines
            .iter()
            .enumerate()
            .find(|(_, line)| line.is_empty())
            .ok_or_else(|| anyhow::anyhow!("Expected empty line, but it is missing"))?;
        Ok(Self {
            supply_stacks: SupplyStacks::try_from(&lines[..idx])?,
            moves: lines[(idx + 1)..]
                .iter()
                .map(|line| line.parse())
                .collect::<Result<_, _>>()?,
        })
    }
}

fn part01(mut input: Input) -> anyhow::Result<String> {
    for move_ in input.moves {
        input.supply_stacks.apply_crate_mover_9000(&move_)?;
    }
    Ok(input
        .supply_stacks
        .stacks
        .iter()
        .filter_map(VecDeque::back)
        .collect())
}

fn part02(mut input: Input) -> anyhow::Result<String> {
    for move_ in input.moves {
        input.supply_stacks.apply_crate_mover_9001(&move_)?;
    }
    Ok(input
        .supply_stacks
        .stacks
        .iter()
        .filter_map(VecDeque::back)
        .collect())
}

fn main() -> anyhow::Result<()> {
    let lines = input_lines(INPUT)?;
    let input = Input::try_from(&lines[..])?;

    println!("Part 1: {}", part01(input.clone())?);
    println!("Part 2: {}", part02(input)?);
    Ok(())
}
