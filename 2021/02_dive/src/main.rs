use helpers::input_lines;
use std::str::FromStr;

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug)]
enum Move {
    Down(usize),
    Forward(usize),
    Up(usize),
}

impl FromStr for Move {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let mut parts = value.split(' ');
        match parts.next() {
            Some("down") => {
                if let Some(second_part) = parts.next() {
                    Ok(Self::Down(second_part.parse().map_err(|_| {
                        anyhow::anyhow!(
                            "Invalid Input. Expected a number, received {}.",
                            second_part
                        )
                    })?))
                } else {
                    Err(anyhow::anyhow!(
                        "Invalid Input. Expected a number, received nothing."
                    ))
                }
            }
            Some("forward") => {
                if let Some(second_part) = parts.next() {
                    Ok(Self::Forward(second_part.parse().map_err(|_| {
                        anyhow::anyhow!(
                            "Invalid Input. Expected a number, received {}.",
                            second_part
                        )
                    })?))
                } else {
                    Err(anyhow::anyhow!(
                        "Invalid Input. Expected a number, received nothing."
                    ))
                }
            }
            Some("up") => {
                if let Some(second_part) = parts.next() {
                    Ok(Self::Up(second_part.parse().map_err(|_| {
                        anyhow::anyhow!(
                            "Invalid Input. Expected a number, received {}.",
                            second_part
                        )
                    })?))
                } else {
                    Err(anyhow::anyhow!(
                        "Invalid Input. Expected a number, received nothing."
                    ))
                }
            }
            Some(first_part) => Err(anyhow::anyhow!(
                "Invalid Input. Expected `down`, `forward` or `up`, received `{}`",
                first_part
            )),
            None => Err(anyhow::anyhow!(
                "Invalid Input. Expected a move qualifier, received nothing."
            )),
        }
    }
}

fn part01(moves: &[Move]) -> usize {
    let (horizontal_position, vertical_position) = moves.iter().fold(
        (0, 0),
        |(horizontal_position, vertical_position), submarine_move| match submarine_move {
            Move::Down(value) => (horizontal_position, vertical_position + value),
            Move::Forward(value) => (horizontal_position + value, vertical_position),
            Move::Up(value) => (horizontal_position, vertical_position - value),
        },
    );
    horizontal_position * vertical_position
}

fn part02(moves: &[Move]) -> usize {
    let (_, horizontal_position, vertical_position) = moves.iter().fold(
        (0, 0, 0),
        |(aim, horizontal_position, vertical_position), submarine_move| match submarine_move {
            Move::Down(value) => (aim + value, horizontal_position, vertical_position),
            Move::Forward(value) => (
                aim,
                horizontal_position + value,
                vertical_position + value * aim,
            ),
            Move::Up(value) => (aim - value, horizontal_position, vertical_position),
        },
    );
    horizontal_position * vertical_position
}

fn main() -> anyhow::Result<()> {
    let moves: Vec<Move> = input_lines(INPUT)?
        .iter()
        .map(|line| line.parse())
        .collect::<Result<_, _>>()?;

    println!("Part 1: {}", part01(&moves));
    println!("Part 2: {}", part02(&moves));
    Ok(())
}
