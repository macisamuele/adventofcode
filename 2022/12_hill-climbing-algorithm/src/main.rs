use helpers::input_lines;
use pathfinding::prelude::dijkstra;
use std::convert::TryFrom;

const INPUT: &str = include_str!("../input.txt");

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
struct Position {
    row: usize,
    column: usize,
}
#[derive(Debug)]
struct Input {
    start: Position,
    end: Position,
    n_rows: usize,
    n_columns: usize,
    map: Vec<Vec<u8>>,
}

impl TryFrom<&Vec<String>> for Input {
    type Error = anyhow::Error;

    fn try_from(lines: &Vec<String>) -> Result<Self, Self::Error> {
        if lines.is_empty() {
            Err(anyhow::anyhow!("Expected to have at least one row"))
        } else if lines
            .iter()
            .skip(1)
            .any(|line| lines[0].len() != line.len())
        {
            Err(anyhow::anyhow!(
                "Expected to have at all the rows with the same number of columns"
            ))
        } else {
            let mut map: Vec<Vec<u8>> = lines.iter().map(|line| line.as_bytes().to_vec()).collect();

            let start = map
                .iter()
                .enumerate()
                .flat_map(|(row_id, row)| {
                    row.iter()
                        .enumerate()
                        .map(move |(column_id, cell)| (row_id, column_id, cell))
                })
                .find_map(|(row_id, column_id, cell)| {
                    if *cell == b'S' {
                        Some(Position {
                            row: row_id,
                            column: column_id,
                        })
                    } else {
                        None
                    }
                })
                .ok_or_else(|| anyhow::anyhow!("Start not found"))?;

            let end = map
                .iter()
                .enumerate()
                .flat_map(|(row_id, row)| {
                    row.iter()
                        .enumerate()
                        .map(move |(column_id, cell)| (row_id, column_id, cell))
                })
                .find_map(|(row_id, column_id, cell)| {
                    if *cell == b'E' {
                        Some(Position {
                            row: row_id,
                            column: column_id,
                        })
                    } else {
                        None
                    }
                })
                .ok_or_else(|| anyhow::anyhow!("Start not found"))?;

            map[start.row][start.column] = b'a';
            map[end.row][end.column] = b'z';

            Ok(Self {
                start,
                end,

                n_rows: lines.len(),
                n_columns: lines[0].len(),
                map,
            })
        }
    }
}

impl std::fmt::Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.map {
            for cell in row {
                write!(f, "{}", char::from(*cell))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn shortest_path_length(input: &Input, start: &Position, end: &Position) -> Option<usize> {
    dijkstra(
        /* start */ start,
        /* successors */
        |position| {
            let mut successors = vec![];
            if position.row > 0
                && input.map[position.row - 1][position.column]
                    <= input.map[position.row][position.column] + 1
            {
                // Up is possible
                successors.push((
                    Position {
                        row: position.row - 1,
                        column: position.column,
                    },
                    1,
                ));
            }
            if position.row < input.n_rows - 1
                && input.map[position.row + 1][position.column]
                    <= input.map[position.row][position.column] + 1
            {
                // Down is possible
                successors.push((
                    Position {
                        row: position.row + 1,
                        column: position.column,
                    },
                    1,
                ));
            }

            if position.column > 0
                && input.map[position.row][position.column - 1]
                    <= input.map[position.row][position.column] + 1
            {
                // Left is possible
                successors.push((
                    Position {
                        row: position.row,
                        column: position.column - 1,
                    },
                    1,
                ));
            }
            if position.column < input.n_columns - 1
                && input.map[position.row][position.column + 1]
                    <= input.map[position.row][position.column] + 1
            {
                // Right is possible
                successors.push((
                    Position {
                        row: position.row,
                        column: position.column + 1,
                    },
                    1,
                ));
            }

            // neighbours
            successors
        },
        /* success */ |position| end == position,
    )
    .map(|(_, cost)| cost)
}

fn part01(input: &Input) -> usize {
    shortest_path_length(input, &input.start, &input.end).unwrap_or(0)
}

fn part02(input: &Input) -> usize {
    input
        .map
        .iter()
        .enumerate()
        .flat_map(|(row_id, row)| {
            row.iter()
                .enumerate()
                .map(move |(column_id, cell)| (row_id, column_id, cell))
        })
        .filter_map(|(row_id, column_id, cell)| {
            if *cell == b'a' {
                Some(Position {
                    row: row_id,
                    column: column_id,
                })
            } else {
                None
            }
        })
        .filter_map(|start_position| shortest_path_length(input, &start_position, &input.end))
        .min()
        .unwrap_or(0)
}

fn main() -> anyhow::Result<()> {
    let input = Input::try_from(&input_lines(INPUT)?)?;

    println!("Part 1: {}", part01(&input));
    println!("Part 2: {}", part02(&input));

    Ok(())
}
