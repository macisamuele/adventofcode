use helpers::input_lines;
use scan_fmt::scan_fmt;
use std::collections::HashMap;
use std::str::FromStr;

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Line {
    point1: Point,
    point2: Point,
}

impl FromStr for Line {
    type Err = anyhow::Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (x1, y1, x2, y2) = scan_fmt!(line, "{},{} -> {},{}", usize, usize, usize, usize)?;
        Ok(Self {
            point1: Point { x: x1, y: y1 },
            point2: Point { x: x2, y: y2 },
        })
    }
}

impl Line {
    fn to_horizontal_points(&self) -> impl Iterator<Item = Point> {
        let y: usize = self.point1.y;
        let (min_x, max_x): (usize, usize) = if self.point1.y == self.point2.y {
            if self.point1.x < self.point2.x {
                (self.point1.x, self.point2.x + 1)
            } else {
                (self.point2.x, self.point1.x + 1)
            }
        } else {
            (0, 0)
        };

        (min_x..max_x).map(move |x| Point { x, y })
    }

    fn to_vertical_points(&self) -> impl Iterator<Item = Point> {
        let x: usize = self.point1.x;
        let (min_y, max_y): (usize, usize) = if self.point1.x == self.point2.x {
            if self.point1.y < self.point2.y {
                (self.point1.y, self.point2.y + 1)
            } else {
                (self.point2.y, self.point1.y + 1)
            }
        } else {
            (0, 0)
        };

        (min_y..max_y).map(move |y| Point { x, y })
    }

    #[allow(clippy::similar_names)]
    fn to_diagonal_points(&self) -> impl Iterator<Item = Point> {
        let (min_x, max_x): (usize, usize) = if self.point1.x < self.point2.x {
            (self.point1.x, self.point2.x + 1)
        } else {
            (self.point2.x, self.point1.x + 1)
        };
        let (min_y, max_y): (usize, usize) = if self.point1.y < self.point2.y {
            (self.point1.y, self.point2.y + 1)
        } else {
            (self.point2.y, self.point1.y + 1)
        };

        let (is_x_negative_increment, is_y_negative_increment, points_in_line) =
            if max_x - min_x == max_y - min_y {
                (
                    self.point1.x > self.point2.x,
                    self.point1.y > self.point2.y,
                    max_x - min_x,
                )
            } else {
                (false, false, 0)
            };

        let point1_x = self.point1.x;
        let point1_y = self.point1.y;
        (0..points_in_line).map(move |point_number| Point {
            x: if is_x_negative_increment {
                point1_x - point_number
            } else {
                point1_x + point_number
            },
            y: if is_y_negative_increment {
                point1_y - point_number
            } else {
                point1_y + point_number
            },
        })
    }
}

fn register_point(sparse_grid: &mut HashMap<Point, usize>, key: Point) {
    let current_value = sparse_grid.get(&key).map_or(0, |value| *value);
    sparse_grid.insert(key, current_value + 1);
}

fn part01(lines: &[Line]) -> usize {
    let mut sparse_grid = HashMap::new();
    for line in lines {
        for point in line.to_horizontal_points().chain(line.to_vertical_points()) {
            register_point(&mut sparse_grid, point);
        }
    }
    sparse_grid.values().filter(|count| **count >= 2).count()
}

fn part02(lines: &[Line]) -> usize {
    let mut sparse_grid = HashMap::new();
    for line in lines {
        for point in line
            .to_horizontal_points()
            .chain(line.to_vertical_points())
            .chain(line.to_diagonal_points())
        {
            register_point(&mut sparse_grid, point);
        }
    }
    sparse_grid.values().filter(|count| **count >= 2).count()
}

fn main() -> anyhow::Result<()> {
    let lines: Vec<Line> = input_lines(INPUT)?
        .iter()
        .map(|line| line.parse())
        .collect::<Result<_, _>>()?;

    println!("Part 1: {}", part01(&lines));
    println!("Part 2: {}", part02(&lines));
    Ok(())
}
