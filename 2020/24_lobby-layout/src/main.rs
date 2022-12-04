use std::collections::HashSet;
use strum::{EnumIter, IntoEnumIterator};

use std::str::FromStr;

use helpers::input_lines;

const INPUT: &str = include_str!("../input.txt");

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

impl std::ops::Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub for Point {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Point {
    fn neighbours(&self) -> impl Iterator<Item = Point> + '_ {
        Direction::iter().map(move |direction| *self + direction.offset())
    }
}

#[derive(Debug, EnumIter)]
enum Direction {
    E,
    NE,
    NW,
    SE,
    SW,
    W,
}

impl Direction {
    fn offset(&self) -> Point {
        match self {
            Direction::E => Point { x: 2, y: 0 },
            Direction::NE => Point { x: 1, y: 1 },
            Direction::NW => Point { x: -1, y: 1 },
            Direction::SE => Point { x: 1, y: -1 },
            Direction::SW => Point { x: -1, y: -1 },
            Direction::W => Point { x: -2, y: 0 },
        }
    }
}

#[derive(Debug)]
struct Path(Vec<Direction>);

impl FromStr for Path {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut directions_str = s;
        let mut directions = Vec::new();
        while !directions_str.is_empty() {
            if let Some(residual_string) = directions_str.strip_prefix('e') {
                directions.push(Direction::E);
                directions_str = residual_string;
            } else if let Some(residual_string) = directions_str.strip_prefix("ne") {
                directions.push(Direction::NE);
                directions_str = residual_string;
            } else if let Some(residual_string) = directions_str.strip_prefix("nw") {
                directions.push(Direction::NW);
                directions_str = residual_string;
            } else if let Some(residual_string) = directions_str.strip_prefix("se") {
                directions.push(Direction::SE);
                directions_str = residual_string;
            } else if let Some(residual_string) = directions_str.strip_prefix("sw") {
                directions.push(Direction::SW);
                directions_str = residual_string;
            } else if let Some(residual_string) = directions_str.strip_prefix('w') {
                directions.push(Direction::W);
                directions_str = residual_string;
            } else {
                unreachable!()
            }
        }

        Ok(Self(directions))
    }
}

impl Path {
    fn evaluate_point(&self) -> Point {
        let mut point = Point { x: 0, y: 0 };
        for direction in &self.0 {
            point += direction.offset();
        }
        point
    }
}

#[derive(Debug)]
struct Floor {
    black_tiles: HashSet<Point>,
}

impl Floor {
    fn points_with_cnt_black_neighbours<I: Iterator<Item = Point>>(
        &self,
        points: I,
        valid_black_neighbours_count: &[usize],
    ) -> HashSet<Point> {
        points
            .filter(|black_tile| {
                valid_black_neighbours_count.contains(
                    &black_tile
                        .neighbours()
                        .filter(|point| self.black_tiles.contains(point))
                        .count(),
                )
            })
            .collect()
    }

    fn round(&mut self) {
        let black_tiles_with_0_or_more_than_2_neighbours = self
            .points_with_cnt_black_neighbours(self.black_tiles.iter().copied(), &[0, 3, 4, 5, 6]);

        let white_tiles_with_2_neighbours = self.points_with_cnt_black_neighbours(
            self.black_tiles
                .iter()
                .flat_map(|point| point.neighbours().filter(|p| !self.black_tiles.contains(p))),
            &[2],
        );

        self.black_tiles.retain(|point| {
            // Remove the black tiles with 0 or more than 2 black neighbours
            !black_tiles_with_0_or_more_than_2_neighbours.contains(point)
        });

        self.black_tiles
            .extend(white_tiles_with_2_neighbours.iter());
    }
}

fn part01(directions: &[Path]) -> usize {
    let mut floor = Floor {
        black_tiles: HashSet::new(),
    };

    for direction in directions {
        let point = direction.evaluate_point();
        if !floor.black_tiles.insert(point) {
            // Not inserting means that was already there
            // So the tile was black and now we're flipping it again
            floor.black_tiles.take(&point);
        }
    }

    floor.black_tiles.len()
}

fn part02(directions: &[Path]) -> usize {
    let mut floor = Floor {
        black_tiles: HashSet::new(),
    };

    for direction in directions {
        let point = direction.evaluate_point();
        if !floor.black_tiles.insert(point) {
            // Not inserting means that was already there
            // So the tile was black and now we're flipping it again
            floor.black_tiles.take(&point);
        }
    }

    for _ in 0..100 {
        floor.round();
    }

    floor.black_tiles.len()
}

fn main() -> anyhow::Result<()> {
    let directions: Vec<Path> = input_lines(INPUT)?
        .iter()
        .filter_map(|line| line.parse().ok())
        .collect();

    println!("Part 1: {}", part01(&directions));
    println!("Part 2: {}", part02(&directions));

    let mut floor = Floor {
        black_tiles: HashSet::new(),
    };
    floor.black_tiles.insert(Point { x: 0, y: 0 });
    floor.black_tiles.insert(Point { x: 2, y: 0 });
    floor.black_tiles.insert(Point { x: -2, y: 0 });
    floor.black_tiles.insert(Point { x: 1, y: 1 });

    Ok(())
}
