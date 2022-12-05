use helpers::input_lines;
use scan_fmt::scan_fmt;
use std::ops::{Deref, RangeInclusive};
use std::str::FromStr;

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug)]
struct Range(RangeInclusive<usize>);
impl Deref for Range {
    type Target = RangeInclusive<usize>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<RangeInclusive<usize>> for Range {
    fn from(value: RangeInclusive<usize>) -> Self {
        Self(value)
    }
}
impl Range {
    fn fully_includes(&self, other: &Range) -> bool {
        self.contains(other.start()) && self.contains(other.end())
    }

    fn overlaps(&self, other: &Range) -> bool {
        !(self.end() < other.start() || self.start() > other.end())
    }
}

#[derive(Debug)]
struct Assignment {
    first: Range,
    second: Range,
}
impl FromStr for Assignment {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first_start, first_end, second_start, second_end) =
            scan_fmt!(s, "{}-{},{}-{}", usize, usize, usize, usize)?;
        Ok(Self {
            first: (first_start..=first_end).into(),
            second: (second_start..=second_end).into(),
        })
    }
}

fn part01(assignments: &[Assignment]) -> usize {
    assignments
        .iter()
        .filter(|assignment| {
            assignment.first.fully_includes(&assignment.second)
                || assignment.second.fully_includes(&assignment.first)
        })
        .count()
}

fn part02(assignments: &[Assignment]) -> usize {
    assignments
        .iter()
        .filter(|assignment| {
            assignment.first.overlaps(&assignment.second)
                || assignment.second.overlaps(&assignment.first)
        })
        .count()
}

fn main() -> anyhow::Result<()> {
    let assignments: Vec<Assignment> = input_lines(INPUT)?
        .iter()
        .map(|line| line.parse())
        .collect::<Result<_, _>>()?;

    println!("Part 1: {}", part01(&assignments));
    println!("Part 2: {}", part02(&assignments));
    Ok(())
}
