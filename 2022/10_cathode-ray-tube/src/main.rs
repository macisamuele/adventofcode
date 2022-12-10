use helpers::input_lines;
use std::collections::BTreeSet;
use std::str::FromStr;

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug)]
enum Instruction {
    Noop,
    Addx(i64),
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "noop" {
            Ok(Self::Noop)
        } else if s.starts_with("addx ") {
            Ok(Self::Addx(s.replace("addx ", "").parse()?))
        } else {
            Err(anyhow::anyhow!("Unrecognised instruction: {s}"))
        }
    }
}

// #[derive(Debug)]
// enum Visibility {
//     Lit,
//     LitNeighbour,
// }

#[derive(Debug)]
struct Grid {
    n_rows: usize,
    n_columns: usize,
    lit_cells: BTreeSet<(usize, usize)>,
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row_idx in 0..self.n_rows {
            for column_idx in 0..self.n_columns {
                if self.lit_cells.contains(&(row_idx, column_idx)) {
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

impl Grid {
    fn new(n_rows: usize, n_columns: usize) -> Self {
        Self {
            n_rows,
            n_columns,
            lit_cells: BTreeSet::new(),
        }
    }
}
fn part01(instructions: &[Instruction]) -> i64 {
    static IMPORTANT_TIMES: &[i64] = &[20, 60, 100, 140, 180, 220];
    instructions
        .iter()
        .flat_map(|instruction| match instruction {
            Instruction::Noop => vec![0],
            Instruction::Addx(v) => vec![0, *v],
        })
        .enumerate()
        .fold((1, 0), |(register_value, score), (clock_tick, to_add)| {
            (
                register_value + to_add,
                if IMPORTANT_TIMES.contains(&((clock_tick as i64) + 1)) {
                    score + ((clock_tick as i64) + 1) * register_value
                } else {
                    score
                },
            )
        })
        .1
}

fn part02(instructions: &[Instruction]) -> Grid {
    instructions
        .iter()
        .flat_map(|instruction| match instruction {
            Instruction::Noop => vec![0],
            Instruction::Addx(v) => vec![0, *v],
        })
        .enumerate()
        .fold(
            (1, Grid::new(6, 40)),
            |(position, mut grid), (clock_tick, to_add)| {
                let row = clock_tick / grid.n_columns;
                let column = clock_tick % grid.n_columns;
                if (position - column as i64).abs() <= 1 {
                    grid.lit_cells.insert((row, column));
                }
                (position + to_add, grid)
            },
        )
        .1
}

fn main() -> anyhow::Result<()> {
    let instructions: Vec<Instruction> = input_lines(INPUT)?
        .iter()
        .map(|line| line.parse())
        .collect::<Result<_, _>>()?;

    println!("Part 1: {}", part01(&instructions));
    println!("Part 2:\n{}", part02(&instructions));
    Ok(())
}
