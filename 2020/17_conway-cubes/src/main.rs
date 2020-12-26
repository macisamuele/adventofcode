use helpers::input_lines;
use std::collections::HashSet;

const INPUT: &str = include_str!("../input.txt");

#[derive(Clone, Eq, Hash, PartialEq)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
    w: i64,
}

impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[x={}, y={}, z={}, w={}]",
            self.x, self.y, self.z, self.w
        )
    }
}

impl Point {
    #[allow(clippy::needless_lifetimes)]
    fn neighbours<'a>(&'a self) -> impl Iterator<Item = Point> + 'a {
        (-1..=1).flat_map(move |dx| {
            (-1..=1).flat_map(move |dy| {
                (-1..=1).flat_map(move |dz| {
                    (-1..=1).filter_map(move |dw| {
                        if (dx, dy, dz, dw) == (0, 0, 0, 0) {
                            None
                        } else {
                            Some(Point {
                                x: self.x + dx,
                                y: self.y + dy,
                                z: self.z + dz,
                                w: self.w + dw,
                            })
                        }
                    })
                })
            })
        })
    }
}
#[derive(Default)]
struct World {
    active_points: HashSet<Point>,
}

impl Clone for World {
    fn clone(&self) -> Self {
        Self {
            active_points: self.active_points.iter().cloned().collect(),
        }
    }
}

impl std::fmt::Debug for World {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (
            Point {
                x: min_x,
                y: min_y,
                z: min_z,
                w: min_w,
            },
            Point {
                x: max_x,
                y: max_y,
                z: max_z,
                w: max_w,
            },
        ) = self.min_max_points();

        for w in min_w..=max_w {
            for z in min_z..=max_z {
                writeln!(f, "z={}, w={}", z, w)?;
                for x in min_x..=max_x {
                    for y in min_y..=max_y {
                        if self.active_points.contains(&Point { x, y, z, w }) {
                            write!(f, "#")?;
                        } else {
                            write!(f, ".")?;
                        }
                    }
                    writeln!(f)?;
                }
            }
        }
        Ok(())
    }
}

impl World {
    fn count_active_points(&self) -> usize {
        self.active_points.len()
    }

    fn min_max_points(&self) -> (Point, Point) {
        let first_point = self.active_points.iter().next().unwrap();
        self.active_points.iter().fold(
            (first_point.clone(), first_point.clone()),
            |(mut min, mut max), point| {
                if point.x < min.x {
                    min.x = point.x;
                }
                if point.y < min.y {
                    min.y = point.y;
                }
                if point.z < min.z {
                    min.z = point.z;
                }
                if point.w < min.w {
                    min.w = point.w;
                }
                if point.x > max.x {
                    max.x = point.x;
                }
                if point.y > max.y {
                    max.y = point.y;
                }
                if point.z > max.z {
                    max.z = point.z;
                }
                if point.w > max.w {
                    max.w = point.w;
                }
                (min, max)
            },
        )
    }

    fn add_from_layer(&mut self, layer: &[String], level: i64) {
        // layer represent the 2d map on `level`
        // Coordinates are (<line>, <column>, <level>)
        self.active_points
            .extend(layer.iter().enumerate().flat_map(|(x, line)| {
                line.chars().enumerate().filter_map(move |(y, character)| {
                    if character == '#' {
                        Some(Point {
                            x: x as i64,
                            y: y as i64,
                            z: level,
                            w: 0,
                        })
                    } else {
                        None
                    }
                })
            }))
    }

    fn is_active(&self, point: &Point) -> bool {
        self.active_points.contains(point)
    }

    fn run_cycle(&mut self, dimensions: usize) {
        let (min_point, max_point) = self.min_max_points();

        let mut points_to_add = HashSet::new();
        let mut points_to_remove = HashSet::new();

        macro_rules! dimension_values {
            ($min_value:expr, $max_value:expr, $min_dimension:expr) => {
                if dimensions > $min_dimension {
                    $min_value - 1..=$max_value + 1
                } else {
                    $min_value..=$max_value
                }
            };
        }

        // Extend the cube of 1 in every direction <- points outside of the
        // currently known active places might be active in the next round
        for x in dimension_values!(min_point.x, max_point.x, 0) {
            for y in dimension_values!(min_point.y, max_point.y, 1) {
                for z in dimension_values!(min_point.z, max_point.z, 2) {
                    for w in dimension_values!(min_point.w, max_point.w, 3) {
                        let point = Point { x, y, z, w };

                        let active_neighbours = point
                            .neighbours()
                            .filter(|neighbour_point| self.is_active(neighbour_point))
                            .count();
                        let will_be_active = active_neighbours == 3
                            || (active_neighbours == 2 && self.is_active(&point));

                        match (will_be_active, self.is_active(&point)) {
                            (false, true) => {
                                points_to_remove.insert(point);
                            }
                            (true, false) => {
                                points_to_add.insert(point);
                            }
                            (false, false) | (true, true) => {}
                        }
                    }
                }
            }
        }
        self.active_points
            .retain(|point| !points_to_remove.contains(point));
        self.active_points.extend(points_to_add);
    }
}

fn part01(world: &World) -> usize {
    let mut cloned_world = world.clone();
    for _ in 0..6 {
        cloned_world.run_cycle(3);
    }
    cloned_world.count_active_points()
}

fn part02(world: &World) -> usize {
    let mut cloned_world = world.clone();
    for _ in 0..6 {
        cloned_world.run_cycle(4);
    }
    cloned_world.count_active_points()
}

fn main() -> anyhow::Result<()> {
    let lines = input_lines(INPUT)?;
    let mut world = World::default();
    world.add_from_layer(&lines, 0);

    println!("Part 1: {}", part01(&world));
    println!("Part 2: {}", part02(&world));
    Ok(())
}
