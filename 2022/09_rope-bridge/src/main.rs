use helpers::input_lines;
use scan_fmt::scan_fmt;
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::convert::TryFrom;
use std::str::FromStr;

const INPUT: &str = include_str!("../input.txt");
impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({},{})", self.row, self.column)
    }
}
impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}({})", self.direction, self.amount)
    }
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,

    // Internal directions for tail to follow head
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
}

impl TryFrom<char> for Direction {
    type Error = anyhow::Error;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'U' => Ok(Self::Up),
            'L' => Ok(Self::Left),
            'D' => Ok(Self::Down),
            'R' => Ok(Self::Right),
            _ => Err(anyhow::anyhow!("Unrecognised direction '{c}'")),
        }
    }
}

#[derive(Debug)]
struct Move {
    direction: Direction,
    amount: usize,
}

impl FromStr for Move {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction_c, amount) = scan_fmt!(s, "{} {}", char, usize)?;
        Ok(Self {
            direction: direction_c.try_into()?,
            amount,
        })
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
struct Position {
    row: i32,
    column: i32,
}

impl Position {
    fn apply(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.row -= 1,
            Direction::Right => self.column += 1,
            Direction::Down => self.row += 1,
            Direction::Left => self.column -= 1,
            Direction::UpRight => {
                self.apply(Direction::Up);
                self.apply(Direction::Right);
            }
            Direction::UpLeft => {
                self.apply(Direction::Up);
                self.apply(Direction::Left);
            }
            Direction::DownRight => {
                self.apply(Direction::Down);
                self.apply(Direction::Right);
            }
            Direction::DownLeft => {
                self.apply(Direction::Down);
                self.apply(Direction::Left);
            }
        }
    }

    fn disatance(self, other_position: Self) -> i32 {
        (self.row - other_position.row)
            .abs()
            .max((self.column - other_position.column).abs())
    }

    fn direction_to(self, other_position: Self) -> Option<Direction> {
        if self.disatance(other_position) <= 1 {
            None
        } else if self.row == other_position.row {
            match self.column.cmp(&other_position.column) {
                Ordering::Less => Some(Direction::Right),
                Ordering::Equal => None,
                Ordering::Greater => Some(Direction::Left),
            }
        } else if self.row > other_position.row {
            match self.column.cmp(&other_position.column) {
                Ordering::Less => Some(Direction::UpRight),
                Ordering::Equal => Some(Direction::Up),
                Ordering::Greater => Some(Direction::UpLeft),
            }
        } else {
            match self.column.cmp(&other_position.column) {
                Ordering::Less => Some(Direction::DownRight),
                Ordering::Equal => Some(Direction::Down),
                Ordering::Greater => Some(Direction::DownLeft),
            }
        }
    }
}

fn rope_movements(moves: &[Move], number_of_knots: usize) -> usize {
    assert!(number_of_knots > 1);

    let mut visited_cells = BTreeSet::<Position>::new();
    let mut knots: Vec<Position> = (0..number_of_knots).map(|_| Position::default()).collect();

    visited_cells.insert(Position::default());

    for move_ in moves {
        for _ in 0..move_.amount {
            knots[0].apply(move_.direction);
            for knot_index in 1..number_of_knots {
                if let Some(direction) = knots[knot_index].direction_to(knots[knot_index - 1]) {
                    knots[knot_index].apply(direction);
                }

                visited_cells.insert(knots[number_of_knots - 1]);
            }
        }
    }
    visited_cells.len()
}

fn part01(moves: &[Move]) -> usize {
    rope_movements(moves, 2)
}

fn part02(moves: &[Move]) -> usize {
    rope_movements(moves, 10)
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
