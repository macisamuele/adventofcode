use helpers::input_lines;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

const INPUT: &str = include_str!("../input.txt");

type Point = (usize, usize);

#[derive(Clone, Debug)]
struct Grid {
    inner: Vec<Vec<u8>>,
    rows: usize,
    columns: usize,
}

impl From<Vec<String>> for Grid {
    fn from(value: Vec<String>) -> Self {
        let inner: Vec<Vec<u8>> = value
            .iter()
            .map(|line| {
                line.chars()
                    .map(|character| (character as u8) - b'0')
                    .collect()
            })
            .collect();
        let rows = inner.len();
        let columns = inner.get(0).map_or(0, Vec::len);
        Self {
            inner,
            rows,
            columns,
        }
    }
}

impl Grid {
    fn neighnours(&self, row: usize, column: usize) -> Vec<Point> {
        vec![
            (row.checked_sub(1), Some(column)),
            (row.checked_add(1), Some(column)),
            (Some(row), column.checked_sub(1)),
            (Some(row), column.checked_add(1)),
        ]
        .iter()
        .filter_map(|(row, column)| match (row, column) {
            (Some(row), Some(column)) if row < &self.rows && column < &self.columns => {
                Some((*row, *column))
            }
            _ => None,
        })
        .collect()
    }

    #[allow(clippy::unused_self)]
    fn top_left(&self) -> Point {
        (0, 0)
    }

    fn bottom_right(&self) -> Point {
        (self.rows - 1, self.columns - 1)
    }

    fn is_in_grid(&self, point: Point) -> bool {
        point.0 < self.rows && point.1 < self.columns
    }

    fn min_risk_path_cost(&self, src: Point, dst: Point) -> usize {
        debug_assert!(self.is_in_grid(src));
        debug_assert!(self.is_in_grid(dst));

        // AKA Dijkstra's shortest path algorithm wher path distance is given by risk to enter!

        // distances[(node.row, node.column)] = current shortest distance from `src` to `dst`
        let mut distances: HashMap<Point, usize> = (0..self.rows)
            .flat_map(|row| (0..self.columns).map(move |column| ((row, column), usize::MAX)))
            .collect();
        distances.insert(src, 0);

        let mut heap: BinaryHeap<(Reverse<usize>, Point)> = BinaryHeap::new();
        heap.push((Reverse(0), src));

        // Examine the frontier with lower cost nodes first (min-heap)
        while let Some((Reverse(cost), position)) = heap.pop() {
            if position == dst {
                return cost;
            }

            // Important as we may have already found a better way
            if cost > distances[&position] {
                continue;
            }

            // For each node we can reach, see if we can find a way with
            // a lower cost going through this node
            for neighbour in self.neighnours(position.0, position.1) {
                let neighbour_cost = cost + (self.inner[neighbour.0][neighbour.1] as usize);
                if neighbour_cost < distances[&neighbour] {
                    heap.push((Reverse(neighbour_cost), neighbour));
                    // Relaxation, we have now found a better way
                    distances.insert(neighbour, neighbour_cost);
                }
            }
        }

        panic!(
            "Failed to find a path from {src:?} to {dst:?}",
            src = src,
            dst = dst
        );
    }

    fn expand(&self, scale: usize) -> Self {
        let rows = self.rows * scale;
        let columns = self.columns * scale;

        Self {
            inner: (0..rows)
                .map(|row| {
                    (0..columns)
                        .map(|column| {
                            let reference = self.inner[row % self.rows][column % self.columns];
                            let add = u8::try_from(row / self.rows + column / self.columns)
                                .expect("Expected number in 0-255");
                            (reference + add - 1) % 9 + 1
                        })
                        .collect()
                })
                .collect(),
            rows,
            columns,
        }
    }
}

fn part01(grid: &Grid) -> usize {
    grid.min_risk_path_cost(grid.top_left(), grid.bottom_right())
}

fn part02(grid: &Grid) -> usize {
    let expanded_grid = grid.expand(5);
    expanded_grid.min_risk_path_cost(expanded_grid.top_left(), expanded_grid.bottom_right())
}

fn main() -> anyhow::Result<()> {
    let grid = Grid::from(input_lines(INPUT)?);

    println!("Part 1: {}", part01(&grid));
    println!("Part 2: {}", part02(&grid));

    Ok(())
}
