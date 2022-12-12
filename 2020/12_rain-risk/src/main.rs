use helpers::input_lines;
use scan_fmt::scan_fmt;
use std::str::FromStr;

const INPUT: &str = include_str!("../input.txt");

#[derive(Clone, Copy, Debug)]
enum Move {
    East(usize),
    Forward(usize),
    North(usize),
    RotateLeft(usize),
    RotateRight(usize),
    South(usize),
    West(usize),
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first_character, number) = scan_fmt!(s, "{[NSEWFRL]}{}", char, usize).unwrap();
        match first_character {
            'E' => Ok(Self::East(number)),
            'F' => Ok(Self::Forward(number)),
            'L' => {
                assert!(number == 0 || number == 90 || number == 180 || number == 270);
                Ok(Self::RotateLeft(number))
            }
            'N' => Ok(Self::North(number)),
            'R' => Ok(Self::RotateRight(number)),
            'S' => Ok(Self::South(number)),
            'W' => Ok(Self::West(number)),
            _ => unreachable!("Unknown rule: {}", first_character),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum ShipDirection {
    East,
    North,
    South,
    West,
}

fn degree_to_quarters(degrees: i64) -> i64 {
    // degrees > 0 => degrees counter-clockwise rotation
    // degrees < 0 => -degrees counter-clockwise rotation
    let mut quarters = -degrees / 90;
    while quarters < 0 {
        quarters += 4;
    }
    quarters
}

impl ShipDirection {
    fn rotate(&mut self, degrees: i64) {
        fn quarter(value: ShipDirection) -> i64 {
            match value {
                ShipDirection::East => 0,
                ShipDirection::South => 1,
                ShipDirection::West => 2,
                ShipDirection::North => 3,
            }
        }

        *self = match (quarter(*self) + degree_to_quarters(degrees)).checked_rem(4) {
            Some(0) => Self::East,
            Some(1) => Self::South,
            Some(2) => Self::West,
            Some(3) => Self::North,
            unknown_value => unreachable!("Not possible value: {:?}", unknown_value),
        }
    }
}

#[derive(Debug)]
struct Point {
    east: i64,
    north: i64,
}

impl Point {
    fn rotate(&mut self, degrees: i64) {
        let rotation_in_quarters = degree_to_quarters(degrees);
        match rotation_in_quarters {
            0 => {
                // (x, y) => (x, y)
            }
            1 => {
                // (x, y) => (y, -x)
                std::mem::swap(&mut self.east, &mut self.north);
                self.east *= -1;
            }
            2 => {
                // (x, y) => (-x, -y)
                self.east *= -1;
                self.north *= -1;
            }
            3 => {
                // (x, y) => (-y, x)
                std::mem::swap(&mut self.east, &mut self.north);
                self.north *= -1;
            }
            unknown_value => unreachable!("Not possible value: {:?}", unknown_value),
        }
    }

    fn manhattan_distance(&self) -> u64 {
        self.north.unsigned_abs() + self.east.unsigned_abs()
    }
}

#[derive(Debug)]
struct Ship {
    direction: ShipDirection,
    // moves: &'a [Move],
    position: Point,
    waypoint_position: Option<Point>,
}

impl Ship {
    fn new() -> Self {
        Self {
            direction: ShipDirection::East,
            position: Point { east: 0, north: 0 },
            waypoint_position: None,
        }
    }

    fn new_with_waypoint(waypoint_position: Point) -> Self {
        Self {
            direction: ShipDirection::East,
            position: Point { east: 0, north: 0 },
            waypoint_position: Some(waypoint_position),
        }
    }

    fn perform_move_without_waypoint(&mut self, move_: &Move) {
        match move_ {
            Move::East(value) => {
                self.position.east += *value as i64;
            }
            Move::Forward(value) => self.perform_move(&match self.direction {
                ShipDirection::South => Move::South(*value),
                ShipDirection::North => Move::North(*value),
                ShipDirection::West => Move::West(*value),
                ShipDirection::East => Move::East(*value),
            }),
            Move::North(value) => {
                self.position.north += *value as i64;
            }
            Move::RotateLeft(value) => {
                self.direction.rotate(*value as i64);
            }
            Move::RotateRight(value) => {
                self.direction.rotate(-(*value as i64));
            }
            Move::South(value) => {
                self.position.north -= *value as i64;
            }
            Move::West(value) => {
                self.position.east -= *value as i64;
            }
        }
    }

    fn perform_move_with_waypoint(&mut self, move_: &Move) {
        let waypoint_position: &mut Point = self.waypoint_position.as_mut().unwrap();
        match move_ {
            Move::East(value) => {
                waypoint_position.east += *value as i64;
            }
            Move::Forward(value) => {
                self.position.east += (*value as i64) * waypoint_position.east;
                self.position.north += (*value as i64) * waypoint_position.north;
            }
            Move::North(value) => {
                waypoint_position.north += *value as i64;
            }
            Move::RotateLeft(value) => {
                waypoint_position.rotate(-(*value as i64));
            }
            Move::RotateRight(value) => {
                waypoint_position.rotate(*value as i64);
            }
            Move::South(value) => {
                waypoint_position.north -= *value as i64;
            }
            Move::West(value) => {
                waypoint_position.east -= *value as i64;
            }
        }
    }

    fn perform_move(&mut self, move_: &Move) {
        if self.waypoint_position.is_none() {
            self.perform_move_without_waypoint(move_);
        } else {
            self.perform_move_with_waypoint(move_);
        }
    }

    fn manhattan_distance(&self) -> u64 {
        self.position.manhattan_distance()
    }
}

fn part01(moves: &[Move]) -> u64 {
    let mut ship = Ship::new();
    for move_ in moves {
        ship.perform_move(move_);
    }
    ship.manhattan_distance()
}

fn part02(moves: &[Move]) -> u64 {
    let mut ship = Ship::new_with_waypoint(Point { east: 10, north: 1 });
    for move_ in moves {
        ship.perform_move(move_);
    }

    ship.manhattan_distance()
}

fn main() -> anyhow::Result<()> {
    let lines = input_lines(INPUT)?;
    let moves: Vec<_> = lines
        .iter()
        .map(|line| line.parse::<Move>().unwrap())
        .collect();
    println!("Part 1: {}", part01(&moves));
    println!("Part 2: {}", part02(&moves));
    Ok(())
}
