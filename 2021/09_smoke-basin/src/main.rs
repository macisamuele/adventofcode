use helpers::input_lines;
use std::ops::{Deref, DerefMut};

const INPUT: &str = include_str!("../input.txt");

#[derive(Clone, Debug)]
struct Grid {
    inner: Vec<Vec<u8>>,
    rows: usize,
    columns: usize,
}

impl Deref for Grid {
    type Target = Vec<Vec<u8>>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Grid {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
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
    fn neighnours(&self, row: usize, column: usize) -> Vec<(usize, usize)> {
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

    fn is_local_minimum(&self, row: usize, column: usize) -> bool {
        let central_value = self[row][column];

        self.neighnours(row, column)
            .iter()
            .all(|(n_row, n_column)| self[*n_row][*n_column] > central_value)
    }

    fn fill_basin(&mut self, row: usize, column: usize) -> usize {
        if self[row][column] == 9 {
            return 0;
        }

        self[row][column] = 9; // Mark it as no longer reachable value

        1 + self
            .neighnours(row, column)
            .iter()
            .map(|(n_row, n_column)| self.fill_basin(*n_row, *n_column))
            .sum::<usize>()
    }
}

fn part01(grid: &Grid) -> usize {
    let (count, sum) = (0..grid.rows)
        .flat_map(|row| {
            (0..grid.columns).filter_map(move |column| {
                if grid.is_local_minimum(row, column) {
                    Some(grid[row][column])
                } else {
                    None
                }
            })
        })
        .fold((0, 0), |(count, sum), value| {
            (count + 1, sum + value as usize)
        });
    count + sum
}

fn part02(grid: &Grid) -> usize {
    let mut grid = grid.clone();
    let mut basin_sizes = vec![];

    for row in 0..grid.rows {
        for column in 0..grid.columns {
            if grid[row][column] < 9 {
                basin_sizes.push(grid.fill_basin(row, column));
            }
        }
    }
    basin_sizes.sort_unstable();
    basin_sizes.iter().rev().take(3).product()
}

fn main() -> anyhow::Result<()> {
    let grid = Grid::from(input_lines(INPUT)?);
    println!("Part 1: {}", part01(&grid));
    println!("Part 2: {}", part02(&grid));
    Ok(())
}
