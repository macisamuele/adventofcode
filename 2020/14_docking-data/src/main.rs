use helpers::input_lines;
use scan_fmt::scan_fmt;
use std::collections::HashMap;
use std::str::FromStr;

const INPUT: &str = include_str!("../input.txt");

#[derive(Copy, Clone, Debug)]
struct MaskPart1 {
    override_: u64,
    and_: u64,
}

#[derive(Clone, Debug)]
struct MaskPart2<'a> {
    mask_str: &'a str,
}

impl MaskPart2<'_> {
    fn get_addresses(&self, address: usize) -> impl Iterator<Item = usize> {
        let result_base: String = format!("{:036b}", address)
            .chars()
            .zip(self.mask_str.chars())
            .map(|(c, mask_c)| match mask_c {
                '0' => c,
                '1' | 'X' => mask_c,
                _ => {
                    unreachable!("Invalid character: {}", mask_c);
                }
            })
            .collect();

        let count_floating = self.mask_str.chars().filter(|c| c == &'X').count();
        (0..(1 << count_floating)).map(move |mut binary_representation_for_floating| {
            usize::from_str_radix(
                &result_base
                    .chars()
                    .map(|c| {
                        if c == 'X' {
                            binary_representation_for_floating >>= 1;
                            if binary_representation_for_floating & 1 == 0 {
                                '0'
                            } else {
                                '1'
                            }
                        } else {
                            c
                        }
                    })
                    .collect::<String>(),
                2,
            )
            .unwrap()
        })
    }
}
#[derive(Debug)]
enum Instruction {
    Mask(String),
    Write { address: usize, value: u64 },
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(mask) = s.strip_prefix("mask = ") {
            Ok(Instruction::Mask(mask.to_string()))
        } else {
            let (address, value) = scan_fmt!(s, "mem[{}] = {}", usize, u64).unwrap();
            Ok(Instruction::Write { address, value })
        }
    }
}

fn part01(instructions: &[Instruction]) -> u64 {
    let mut mask = MaskPart1 {
        override_: 0,
        and_: 0,
    };
    let mut memory_slots: HashMap<usize, u64> = HashMap::new();
    for instruction in instructions {
        match instruction {
            Instruction::Mask(mask_str) => {
                mask = MaskPart1 {
                    override_: u64::from_str_radix(&mask_str.replace('X', "0"), 2).unwrap(),
                    and_: u64::from_str_radix(&mask_str.replace('X', "1"), 2).unwrap(),
                };
            }
            Instruction::Write { address, value } => {
                memory_slots.insert(*address, (value | mask.override_) & mask.and_);
            }
        };
    }
    memory_slots.values().copied().sum()
}

fn part02(instructions: &[Instruction]) -> u64 {
    let mut mask = MaskPart2 {
        mask_str: "000000000000000000000000000000000000",
    };
    let mut memory_slots: HashMap<usize, u64> = HashMap::new();
    for instruction in instructions {
        match instruction {
            Instruction::Mask(mask_str) => {
                mask = MaskPart2 { mask_str };
            }
            Instruction::Write { address, value } => {
                for address in mask.get_addresses(*address) {
                    memory_slots.insert(address, *value);
                }
            }
        };
    }
    memory_slots.values().copied().sum()
}

fn main() -> anyhow::Result<()> {
    let lines = input_lines(INPUT)?;
    let instructions: Vec<Instruction> =
        lines.iter().filter_map(|line| line.parse().ok()).collect();

    println!("Part 1: {}", part01(&instructions));
    println!("Part 2: {}", part02(&instructions));
    Ok(())
}
