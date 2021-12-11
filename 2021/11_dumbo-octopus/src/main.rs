use helpers::input_lines;
use std::convert::TryFrom;
use std::fmt;
use std::ops::{Deref, DerefMut};

const INPUT: &str = include_str!("../input.txt");
const ROWS: usize = 10;
const COLUMNS: usize = 10;

#[derive(Clone)]
struct Grid {
    inner: [[u8; COLUMNS]; ROWS],
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.inner {
            for cell in row {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f)?;
        for row in self.inner {
            for cell in row {
                write!(f, "{:2},", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Deref for Grid {
    type Target = [[u8; COLUMNS]; ROWS];

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Grid {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl TryFrom<Vec<String>> for Grid {
    type Error = anyhow::Error;

    fn try_from(lines: Vec<String>) -> Result<Self, Self::Error> {
        anyhow::ensure!(lines.len() == ROWS);
        let mut matrix = [[0; COLUMNS]; ROWS];
        for (row_number, line) in lines.iter().enumerate() {
            for (column_number, byte) in line.bytes().enumerate() {
                anyhow::ensure!(byte >= b'0');
                anyhow::ensure!(byte <= b'9');
                let cell: u8 = byte - b'0';
                matrix[row_number][column_number] = cell;
            }
        }
        Ok(Self { inner: matrix })
    }
}

impl Grid {
    fn neighnours(row: usize, column: usize) -> Vec<(usize, usize)> {
        [
            (row.checked_sub(1), column.checked_sub(1)),
            (row.checked_sub(1), Some(column)),
            (row.checked_sub(1), column.checked_add(1)),
            (Some(row), column.checked_sub(1)),
            (Some(row), column.checked_add(1)),
            (row.checked_add(1), column.checked_sub(1)),
            (row.checked_add(1), Some(column)),
            (row.checked_add(1), column.checked_add(1)),
        ]
        .iter()
        .filter_map(|(row, column)| match (row, column) {
            (Some(row), Some(column)) if row < &ROWS && column < &COLUMNS => Some((*row, *column)),
            _ => None,
        })
        .collect()
    }

    fn all_flashed(&self) -> bool {
        (0..ROWS as usize).all(|row| (0..COLUMNS as usize).all(|column| self[row][column] == 0))
    }

    fn next_step(&mut self) -> usize {
        fn flash_recurse(grid: &mut Grid, row: usize, column: usize) -> usize {
            if grid[row][column] > 9 {
                let mut res = 1;
                grid[row][column] = 0;
                for (n_row, n_column) in Grid::neighnours(row, column) {
                    if grid[n_row][n_column] > 0 {
                        grid[n_row][n_column] += 1;
                        res += flash_recurse(grid, n_row, n_column);
                    }
                }
                res
            } else {
                0
            }
        }

        for row in 0..ROWS {
            for column in 0..COLUMNS {
                self[row][column] += 1;
            }
        }

        (0..ROWS as usize)
            .map(|row| {
                (0..COLUMNS as usize)
                    .map(|column| flash_recurse(self, row, column))
                    .sum::<usize>()
            })
            .sum()
    }
}

fn part01(grid: &Grid) -> usize {
    let mut grid: Grid = grid.clone();
    (0..100).map(|_| grid.next_step()).sum()
}

fn part02(grid: &Grid) -> usize {
    let mut grid: Grid = grid.clone();
    (1..usize::MAX)
        .find(|_| {
            grid.next_step();
            grid.all_flashed()
        })
        .expect("All the octopus should sync")
}

fn main() -> anyhow::Result<()> {
    let grid = Grid::try_from(input_lines(INPUT)?)?;
    println!("Part 1: {}", part01(&grid));
    println!("Part 2: {}", part02(&grid));
    Ok(())
}
