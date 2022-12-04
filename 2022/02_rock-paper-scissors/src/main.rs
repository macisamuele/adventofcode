use helpers::input_lines;
use std::str::FromStr;
use strum::{EnumIter, IntoEnumIterator};

const INPUT: &str = include_str!("../input.txt");

#[derive(Clone, Copy, Debug, PartialEq)]
enum LoseDrawWin {
    Lose,
    Draw,
    Win,
}

impl FromStr for LoseDrawWin {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Lose),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => Err(anyhow::anyhow!("Unrecognised characted: {s}")),
        }
    }
}

impl LoseDrawWin {
    fn score(self) -> usize {
        match self {
            Self::Win => 6,
            Self::Draw => 3,
            Self::Lose => 0,
        }
    }
}

#[derive(Clone, Copy, Debug, EnumIter)]
enum Shape {
    Rock,
    Paper,
    Scissor,
}

impl FromStr for Shape {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissor),
            _ => Err(anyhow::anyhow!("Unrecognised characted: {s}")),
        }
    }
}

impl Shape {
    fn score(self) -> usize {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissor => 3,
        }
    }

    fn compete(self, opponent: Shape) -> LoseDrawWin {
        match (self, opponent) {
            (Self::Rock, Self::Scissor)
            | (Self::Paper, Self::Rock)
            | (Self::Scissor, Self::Paper) => LoseDrawWin::Win,

            (Self::Rock, Self::Rock)
            | (Self::Paper, Self::Paper)
            | (Self::Scissor, Self::Scissor) => LoseDrawWin::Draw,

            (Self::Rock, Self::Paper)
            | (Self::Paper, Self::Scissor)
            | (Self::Scissor, Self::Rock) => LoseDrawWin::Lose,
        }
    }
}

#[derive(Debug)]
struct Round {
    opponent: Shape,
    player: Shape,          // for part01
    objective: LoseDrawWin, // for part02
}

impl FromStr for Round {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(' ').collect();
        if parts.len() == 2 {
            Ok(Self {
                opponent: parts[0].parse()?,
                player: parts[1].parse()?,
                objective: parts[1].parse()?,
            })
        } else {
            Err(anyhow::anyhow!(
                "Expected only 2 parts, received {}",
                parts.len()
            ))
        }
    }
}

fn part01(rounds: &[Round]) -> usize {
    rounds
        .iter()
        .map(|round| round.player.score() + round.player.compete(round.opponent).score())
        .sum()
}

fn part02(rounds: &[Round]) -> usize {
    rounds
        .iter()
        .map(|round| {
            let player: Shape = Shape::iter()
                .find(|player| player.compete(round.opponent) == round.objective)
                .expect("It does exist a way to reach the objective");
            player.score() + round.objective.score()
        })
        .sum()
}

fn main() -> anyhow::Result<()> {
    let rounds: Vec<Round> = input_lines(INPUT)?
        .iter()
        .map(|line| line.parse())
        .collect::<Result<_, _>>()?;

    println!("Part 1: {}", part01(&rounds));
    println!("Part 2: {}", part02(&rounds));
    Ok(())
}
