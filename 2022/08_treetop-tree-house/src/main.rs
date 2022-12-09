use helpers::input_lines;

const INPUT: &str = include_str!("../input.txt");

fn part01(grid: &[Vec<u8>]) -> usize {
    let n_rows = grid.len();
    let n_cols = grid[0].len();
    (0..n_rows)
        .map(|row_id: usize| {
            if row_id == 0 || row_id == n_rows - 1 {
                n_cols
            } else {
                (0..n_cols)
                    .filter(|col_id: &usize| {
                        let col_id: usize = *col_id;
                        (
                            // border
                            col_id == 0 || col_id == n_cols - 1
                        ) || (
                            // visible from left
                            (0..col_id).rev().all(|other_col_id| {
                                grid[row_id][col_id] > grid[row_id][other_col_id]
                            })
                        ) || (
                            // visible from right
                            (col_id + 1..n_cols).all(|other_col_id| {
                                grid[row_id][col_id] > grid[row_id][other_col_id]
                            })
                        ) || (
                            // visible from top
                            (0..row_id).rev().all(|other_row_id| {
                                grid[row_id][col_id] > grid[other_row_id][col_id]
                            })
                        ) || (
                            // visible from bottom
                            (row_id + 1..n_rows).all(|other_row_id| {
                                grid[row_id][col_id] > grid[other_row_id][col_id]
                            })
                        )
                    })
                    .count()
            }
        })
        .sum()
}

fn part02(grid: &[Vec<u8>]) -> usize {
    let n_rows = grid.len();
    let n_cols = grid[0].len();

    let viewing_score = |row_id: usize, col_id: usize| {
        if row_id == 0 || col_id == 0 || row_id == n_rows - 1 || col_id == n_cols - 1 {
            0
        } else {
            let mut visibility_left = (0..col_id)
                .rev()
                .take_while(|other_col_id| grid[row_id][col_id] > grid[row_id][*other_col_id])
                .count();
            if visibility_left < col_id {
                // We did not reach the border
                if grid[row_id][col_id - visibility_left - 1] >= grid[row_id][col_id] {
                    visibility_left += 1;
                }
            }

            let mut visibility_right = (col_id + 1..n_cols)
                .take_while(|other_col_id| grid[row_id][col_id] > grid[row_id][*other_col_id])
                .count();
            if col_id + visibility_right < n_cols - 1 {
                // We did not reach the border
                if grid[row_id][col_id + visibility_right + 1] >= grid[row_id][col_id] {
                    visibility_right += 1;
                }
            }

            let mut visibility_up = (0..row_id)
                .rev()
                .take_while(|other_row_id| grid[row_id][col_id] > grid[*other_row_id][col_id])
                .count();
            if visibility_up < row_id {
                // We did not reach the border
                if grid[row_id - visibility_up - 1][col_id] >= grid[row_id][col_id] {
                    visibility_up += 1;
                }
            }

            let mut visibility_down = (row_id + 1..n_rows)
                .take_while(|other_row_id| grid[row_id][col_id] > grid[*other_row_id][col_id])
                .count();
            if row_id + visibility_down < n_rows - 1 {
                // We did not reach the border
                if grid[row_id + visibility_down + 1][col_id] >= grid[row_id][col_id] {
                    visibility_down += 1;
                }
            }

            visibility_left * visibility_right * visibility_up * visibility_down
        }
    };

    (1..(n_rows - 1))
        .flat_map(|row_id| (1..(n_cols - 1)).map(move |col_id| (row_id, col_id)))
        .map(
            // Generated row_id and col_id excluding the borders as the score will be
            // 0 by definition ("at least one of its viewing distances will be zero")
            |(row_id, col_id)| viewing_score(row_id, col_id),
        )
        .max()
        .unwrap_or(0)
}

fn main() -> anyhow::Result<()> {
    let lines = input_lines(INPUT)?;
    let grid: Vec<Vec<u8>> = lines
        .iter()
        .map(|line| line.as_bytes().iter().map(|b| b - b'0').collect())
        .collect();
    assert!(!grid.is_empty() && grid.iter().all(|row| row.len() == grid[0].len()));

    println!("Part 1: {}", part01(&grid));
    println!("Part 2: {}", part02(&grid));
    Ok(())
}
