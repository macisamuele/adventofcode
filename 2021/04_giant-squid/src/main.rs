use helpers::input_lines;
use std::convert::TryFrom;

const INPUT: &str = include_str!("../input.txt");

type BingoNumber = u8; // Using u8 as numbers in the input are in the range 0-99

#[derive(Debug)]
struct Grid {
    values: [[BingoNumber; 5]; 5],
}

impl TryFrom<&[String]> for Grid {
    type Error = anyhow::Error;

    fn try_from(lines: &[String]) -> Result<Self, Self::Error> {
        anyhow::ensure!(lines.len() == 5);

        let mut values = [[0; 5]; 5];

        for (line_no, line) in lines.iter().enumerate() {
            let parsed_values: Vec<BingoNumber> = line
                .split(' ')
                .filter(|part| !part.is_empty())
                .map(str::parse)
                .collect::<Result<_, _>>()?;
            anyhow::ensure!(parsed_values.len() == 5);

            values[line_no][..5].copy_from_slice(&parsed_values[..5]);
        }

        Ok(Self { values })
    }
}

#[derive(Debug)]
struct Game {
    extractions: Vec<BingoNumber>,
    grids: Vec<Grid>,
}

impl TryFrom<Vec<String>> for Game {
    type Error = anyhow::Error;

    fn try_from(lines: Vec<String>) -> Result<Self, Self::Error> {
        anyhow::ensure!(
            (lines.len() - 1) % 6 == 0,
            "Invalid input: number of lines do not match the expectations"
        );

        let extractions = lines[0]
            .split(',')
            .map(str::parse)
            .collect::<Result<_, _>>()?;

        let grids = lines[1..]
            .chunks(6)
            .map(|lines_chunk| Grid::try_from(&lines_chunk[1..]))
            .collect::<Result<_, _>>()?;

        Ok(Self { extractions, grids })
    }
}

#[derive(Debug)]
struct GridMetadata<'g> {
    grid: &'g Grid,
    matched_cells: [[bool; 5]; 5],
    matched_in_rows: [u8; 5],
    matched_in_columns: [u8; 5],
    last_extracted: Option<BingoNumber>,
}

impl<'g> From<&'g Grid> for GridMetadata<'g> {
    fn from(grid: &'g Grid) -> Self {
        Self {
            grid,
            matched_cells: [[false; 5]; 5],
            matched_in_rows: [0; 5],
            matched_in_columns: [0; 5],
            last_extracted: None,
        }
    }
}

impl GridMetadata<'_> {
    fn register_extraction(&mut self, value: BingoNumber) {
        self.last_extracted = Some(value);
        for (row_index, row) in self.grid.values.iter().enumerate() {
            for (column_index, cell_value) in row.iter().enumerate() {
                if *cell_value == value {
                    self.matched_in_rows[row_index] += 1;
                    self.matched_in_columns[column_index] += 1;
                    self.matched_cells[row_index][column_index] = true;
                    return;
                }
            }
        }
    }

    fn winning_score(&self) -> Option<usize> {
        if self.matched_in_rows.iter().any(|v| *v == 5)
            || self.matched_in_columns.iter().any(|v| *v == 5)
        {
            let sum_not_marked: usize = self
                .matched_cells
                .iter()
                .enumerate()
                .flat_map(|(row_index, row)| {
                    row.iter()
                        .enumerate()
                        .map(move |(column_index, cell_value)| {
                            if *cell_value {
                                0
                            } else {
                                self.grid.values[row_index][column_index] as usize
                            }
                        })
                })
                .sum();
            let last_extracted = self
                .last_extracted
                .expect("An extracted value should be present to have a winning board")
                as usize;

            Some(sum_not_marked * last_extracted)
        } else {
            None
        }
    }
}

fn part01(game: &Game) -> usize {
    let mut grids_metadata: Vec<_> = game.grids.iter().map(GridMetadata::from).collect();
    for extracted_value in &game.extractions {
        for grid_metadata in &mut grids_metadata {
            grid_metadata.register_extraction(*extracted_value);
            if let Some(winning_score) = grid_metadata.winning_score() {
                return winning_score;
            }
        }
    }
    0
}

fn part02(game: &Game) -> usize {
    let mut grids_metadata: Vec<_> = game.grids.iter().map(GridMetadata::from).collect();

    let mut extractions = game.extractions.iter();

    let mut last_won_score = None;
    loop {
        let extracted_value = extractions.next();
        if extracted_value.is_none() || grids_metadata.is_empty() {
            return last_won_score.expect("Expected the definition of a winning score");
        }
        let mut indexes_to_remove = vec![];
        for (index, grid_metadata) in grids_metadata.iter_mut().enumerate() {
            grid_metadata.register_extraction(
                *extracted_value.expect("Expected to have an extracted value"),
            );
            if let Some(winning_score) = grid_metadata.winning_score() {
                last_won_score = Some(winning_score);
                indexes_to_remove.push(index);
            }
        }
        for index_to_remove in indexes_to_remove.iter().rev() {
            grids_metadata.remove(*index_to_remove);
        }
    }
}

fn main() -> anyhow::Result<()> {
    let lines = input_lines(INPUT)?;
    let game: Game = Game::try_from(lines)?;
    println!("Part 1: {}", part01(&game));
    println!("Part 2: {}", part02(&game));
    Ok(())
}
