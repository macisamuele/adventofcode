use helpers::input_lines;
use scan_fmt::scan_fmt;
use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;
use std::ops::RangeInclusive;

const INPUT: &str = include_str!("../input.txt");

#[derive(Clone, Copy, Debug)]
enum Fold {
    Horizontally(isize),
    Vertically(isize),
}

#[derive(Clone, Debug, Default)]
struct SparseGrid(HashMap<isize, HashSet<isize>>);

impl SparseGrid {
    fn rows_range(&self) -> RangeInclusive<isize> {
        let min_row = *self
            .0
            .keys()
            .min()
            .expect("Expected to have at least one row");
        let max_row = *self
            .0
            .keys()
            .max()
            .expect("Expected to have at least one row");
        min_row..=max_row
    }

    fn columns_range(&self) -> RangeInclusive<isize> {
        let min_columns = *self
            .0
            .values()
            .flat_map(HashSet::iter)
            .min()
            .expect("Expected to have at least one row");
        let max_columns = *self
            .0
            .values()
            .flat_map(HashSet::iter)
            .max()
            .expect("Expected to have at least one row");
        min_columns..=max_columns
    }

    fn cells(&self) -> impl Iterator<Item = (isize, isize)> + '_ {
        self.0
            .iter()
            .flat_map(|(row, value)| value.iter().map(move |column| (*row, *column)))
    }

    fn register(&mut self, row: isize, column: isize) {
        self.0
            .entry(row)
            .or_insert_with(HashSet::new)
            .insert(column);
    }

    fn apply_folding_rules(&self, rules: &[Fold]) -> SparseGrid {
        self.cells()
            .filter_map(|(mut row, mut column)| {
                for rule in rules {
                    match rule {
                        Fold::Vertically(value) if value == &row => {
                            // The cell is on the folding line, so it should not be present
                            return None;
                        }
                        Fold::Vertically(value) if &row > value => {
                            row = 2 * value - row;
                        }
                        Fold::Horizontally(value) if value == &column => {
                            // The cell is on the folding line, so it should not be present
                            return None;
                        }
                        Fold::Horizontally(value) if &column > value => {
                            column = 2 * value - column;
                        }
                        _ => {}
                    }
                }
                Some((row, column))
            })
            .fold(SparseGrid::default(), |mut spare_grid, (row, column)| {
                spare_grid.register(row, column);
                spare_grid
            })
    }
}

#[derive(Debug)]
struct Origami {
    sparse_grid: SparseGrid,
    rules: Vec<Fold>,
}

impl Origami {
    fn apply_first_move(&self) -> SparseGrid {
        self.sparse_grid.apply_folding_rules(&[self
            .rules
            .first()
            .copied()
            .expect("At least one rule is defined")])
    }

    fn apply_all_moves(&self) -> SparseGrid {
        self.sparse_grid.apply_folding_rules(&self.rules)
    }
}

impl TryFrom<Vec<String>> for Origami {
    type Error = anyhow::Error;

    fn try_from(lines: Vec<String>) -> Result<Self, Self::Error> {
        let mut sparse_grid = SparseGrid::default();
        let mut rules = Vec::new();

        let mut read_rules = false;
        for line in lines {
            if line.is_empty() {
                read_rules = true;
                continue;
            };

            if read_rules {
                let (axis, value) = scan_fmt!(&line, "fold along {}={}", char, isize)?;
                anyhow::ensure!(axis == 'x' || axis == 'y');
                rules.push(match axis {
                    'x' => Fold::Horizontally(value),
                    'y' => Fold::Vertically(value),
                    _ => unreachable!(),
                });
            } else {
                let (column, row) = scan_fmt!(&line, "{},{}", isize, isize)?;
                sparse_grid.register(row, column);
            }
        }
        Ok(Self { sparse_grid, rules })
    }
}

fn part01(origami: &Origami) -> usize {
    origami.apply_first_move().cells().count()
}

fn part02(origami: &Origami) -> String {
    format!("\n{}", origami.apply_all_moves())
}

impl std::fmt::Display for SparseGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row_index in self.rows_range() {
            let maybe_row_values = self.0.get(&row_index);

            for column_index in self.columns_range() {
                if maybe_row_values.map_or(false, |row_values| row_values.contains(&column_index)) {
                    write!(f, "#")?;
                } else {
                    write!(f, " ")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    let origami = Origami::try_from(input_lines(INPUT)?)?;

    println!("Part 1: {}", part01(&origami));
    println!("Part 2: {}", part02(&origami));

    Ok(())
}
