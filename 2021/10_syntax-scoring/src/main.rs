use helpers::input_lines;
use std::convert::TryFrom;
use std::ops::Deref;

const INPUT: &str = include_str!("../input.txt");

#[derive(Clone, Copy, Debug)]
enum Bracket {
    OpenRound,
    CloseRound,
    OpenSquare,
    CloseSquare,
    OpenCurly,
    CloseCurly,
    OpenAngle,
    CloseAngle,
}

impl TryFrom<char> for Bracket {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '(' => Ok(Self::OpenRound),
            ')' => Ok(Self::CloseRound),
            '[' => Ok(Self::OpenSquare),
            ']' => Ok(Self::CloseSquare),
            '{' => Ok(Self::OpenCurly),
            '}' => Ok(Self::CloseCurly),
            '<' => Ok(Self::OpenAngle),
            '>' => Ok(Self::CloseAngle),
            _ => Err(anyhow::anyhow!("Unexpected:  {value}", value = value)),
        }
    }
}

impl Bracket {
    fn is_opening(self) -> bool {
        matches!(
            self,
            Self::OpenRound | Self::OpenSquare | Self::OpenCurly | Self::OpenAngle
        )
    }

    fn is_closing_of(self, other: Bracket) -> bool {
        matches!(
            (self, other),
            (Self::CloseRound, Self::OpenRound)
                | (Self::CloseSquare, Self::OpenSquare)
                | (Self::CloseCurly, Self::OpenCurly)
                | (Self::CloseAngle, Self::OpenAngle)
        )
    }

    fn score_if_incorrect(self) -> usize {
        match self {
            Self::CloseRound => 3,
            Self::CloseSquare => 57,
            Self::CloseCurly => 1197,
            Self::CloseAngle => 25137,
            _ => 0,
        }
    }

    fn to_closing(self) -> Self {
        match self {
            Self::OpenRound => Self::CloseRound,
            Self::OpenSquare => Self::CloseSquare,
            Self::OpenCurly => Self::CloseCurly,
            Self::OpenAngle => Self::CloseAngle,
            val => val,
        }
    }

    fn score_if_filling(self) -> usize {
        match self {
            Self::CloseRound => 1,
            Self::CloseSquare => 2,
            Self::CloseCurly => 3,
            Self::CloseAngle => 4,
            _ => 0,
        }
    }
}

#[derive(Debug)]
struct Chunk {
    inner: Vec<Bracket>,
}

impl Deref for Chunk {
    type Target = Vec<Bracket>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl TryFrom<&String> for Chunk {
    type Error = anyhow::Error;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        Ok(Self {
            inner: value
                .chars()
                .map(Bracket::try_from)
                .collect::<Result<_, _>>()?,
        })
    }
}

impl Chunk {
    fn residual_stack(&self) -> Result<Vec<Bracket>, Bracket> {
        let mut stack: Vec<Bracket> = vec![];

        for bracket in self.iter() {
            if bracket.is_opening() {
                stack.push(*bracket);
            } else if let Some(stack_head) = stack.last() {
                if bracket.is_closing_of(*stack_head) {
                    stack.pop();
                } else {
                    return Err(*bracket);
                }
            } else {
                return Err(*bracket);
            }
        }

        Ok(stack)
    }
}

fn part01(chunks: &[Chunk]) -> usize {
    chunks
        .iter()
        .filter_map(|chunk| chunk.residual_stack().err())
        .map(Bracket::score_if_incorrect)
        .sum()
}

fn part02(chunks: &[Chunk]) -> usize {
    let mut scores: Vec<_> = chunks
        .iter()
        .filter_map(|chunk| chunk.residual_stack().ok())
        .map(|residual| {
            residual.iter().rev().fold(0, |result, bracket| {
                result * 5 + bracket.to_closing().score_if_filling()
            })
        })
        .collect();

    let central_index = scores.len() / 2;
    *scores.select_nth_unstable(central_index).1
}

fn main() -> anyhow::Result<()> {
    let chunks = input_lines(INPUT)?
        .iter()
        .map(Chunk::try_from)
        .collect::<Result<Vec<Chunk>, _>>()?;

    println!("Part 1: {}", part01(&chunks));
    println!("Part 2: {}", part02(&chunks));
    Ok(())
}
