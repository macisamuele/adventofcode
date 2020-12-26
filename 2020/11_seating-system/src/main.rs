use helpers::input_lines;

const INPUT: &str = include_str!("../input.txt");

const DIRECTIONS: [Direction; 8] = [
    Direction::NW,
    Direction::N,
    Direction::NE,
    Direction::W,
    Direction::E,
    Direction::SW,
    Direction::S,
    Direction::SE,
];

#[derive(Debug)]
enum Direction {
    NW,
    N,
    NE,
    W,
    E,
    SW,
    S,
    SE,
}

fn usize_add_isize(value: usize, to_add: &isize) -> Option<usize> {
    if to_add >= &0 {
        Some(value + (*to_add as usize))
    } else {
        let to_add_abs = (-(*to_add)) as usize;
        value.checked_sub(to_add_abs)
    }
}

impl Direction {
    fn delta_row(&self) -> isize {
        match self {
            Self::NW | Self::N | Self::NE => -1,
            Self::SW | Self::S | Self::SE => 1,
            Self::W | Self::E => 0,
        }
    }
    fn delta_column(&self) -> isize {
        match self {
            Self::NW | Self::W | Self::SW => -1,
            Self::NE | Self::E | Self::SE => 1,
            Self::N | Self::S => 0,
        }
    }

    fn move_(&self, (row, column): (usize, usize)) -> Option<(usize, usize)> {
        match (
            usize_add_isize(row, &self.delta_row()),
            usize_add_isize(column, &self.delta_column()),
        ) {
            (Some(r), Some(c)) => Some((r, c)),
            _ => None,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Place {
    Floor,
    Empty,
    Occupied,
}

impl From<char> for Place {
    fn from(c: char) -> Self {
        match c {
            'L' => Self::Empty,
            '#' => Self::Occupied,
            _ => Self::Floor,
        }
    }
}

#[derive(Debug, Clone)]
struct Map {
    grid: Vec<Vec<Place>>,
    n_rows: usize,
    n_columns: usize,
}

impl From<&Vec<String>> for Map {
    fn from(lines: &Vec<String>) -> Self {
        assert!(!lines.is_empty());
        Self {
            grid: lines
                .iter()
                .map(|line| line.chars().map(Place::from).collect())
                .collect(),
            n_rows: lines.len(),
            n_columns: lines[0].len(),
        }
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.grid {
            writeln!(
                f,
                "{}",
                row.iter()
                    .map(|col| match col {
                        Place::Empty => 'L',
                        Place::Occupied => '#',
                        Place::Floor => '.',
                    })
                    .collect::<String>()
            )?;
        }
        Ok(())
    }
}

impl Map {
    fn move_toward(
        &self,
        (row, column): (usize, usize),
        direction: &Direction,
    ) -> Option<(usize, usize)> {
        match direction.move_((row, column)) {
            Some((r, c)) if r < self.n_rows && c < self.n_columns => Some((r, c)),
            _ => None,
        }
    }

    fn count_adjacent_occupied(&self, (row, column): (usize, usize)) -> usize {
        DIRECTIONS
            .iter()
            .map(
                |direction| match self.move_toward((row, column), direction) {
                    Some((r, c)) if self.grid[r][c] == Place::Occupied => 1,
                    _ => 0,
                },
            )
            .sum()
    }

    fn get_first_neighbour(
        &self,
        (mut row, mut column): (usize, usize),
        direction: &Direction,
    ) -> Option<(usize, usize)> {
        while let Some((r, c)) = self.move_toward((row, column), direction) {
            match self.grid[r][c] {
                Place::Occupied | Place::Empty => {
                    return Some((r, c));
                }
                _ => {}
            }
            row = r;
            column = c;
        }
        None
    }

    fn count_visible_occupied(&self, (row, column): (usize, usize)) -> usize {
        DIRECTIONS
            .iter()
            .map(
                |direction| match self.get_first_neighbour((row, column), direction) {
                    Some((r, c)) if self.grid[r][c] == Place::Occupied => 1,
                    _ => 0,
                },
            )
            .sum()
    }

    fn run_move_part1(&mut self) -> bool {
        let mut one_place_is_updated = false;
        let mut new_grid = self
            .grid
            .iter()
            .enumerate()
            .map(|(row_no, row)| {
                row.iter()
                    .enumerate()
                    .map(|(column_no, cell)| {
                        if cell == &Place::Empty
                            && self.count_adjacent_occupied((row_no, column_no)) == 0
                        {
                            one_place_is_updated = true;
                            Place::Occupied
                        } else if cell == &Place::Occupied
                            && self.count_adjacent_occupied((row_no, column_no)) >= 4
                        {
                            one_place_is_updated = true;
                            Place::Empty
                        } else {
                            *cell
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        if one_place_is_updated {
            std::mem::swap(&mut self.grid, &mut new_grid);
            true
        } else {
            false
        }
    }

    fn run_move_part2(&mut self) -> bool {
        let mut one_place_is_updated = false;
        let mut new_grid = self
            .grid
            .iter()
            .enumerate()
            .map(|(row_no, row)| {
                row.iter()
                    .enumerate()
                    .map(|(column_no, cell)| {
                        if cell == &Place::Empty
                            && self.count_visible_occupied((row_no, column_no)) == 0
                        {
                            one_place_is_updated = true;
                            Place::Occupied
                        } else if cell == &Place::Occupied
                            && self.count_visible_occupied((row_no, column_no)) >= 5
                        {
                            one_place_is_updated = true;
                            Place::Empty
                        } else {
                            *cell
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        if one_place_is_updated {
            std::mem::swap(&mut self.grid, &mut new_grid);
            true
        } else {
            false
        }
    }

    fn count_occupied_seats(&self) -> usize {
        self.grid
            .iter()
            .map(|row| {
                row.iter()
                    .filter(|place| *place == &Place::Occupied)
                    .count()
            })
            .sum()
    }
}

fn part01(map: &Map) -> usize {
    let mut cloned_map = map.clone();
    while cloned_map.run_move_part1() {}
    cloned_map.count_occupied_seats()
}

fn part02(map: &Map) -> usize {
    let mut cloned_map = map.clone();
    while cloned_map.run_move_part2() {}
    cloned_map.count_occupied_seats()
}

fn main() -> anyhow::Result<()> {
    let lines = input_lines(INPUT)?;
    let map = Map::from(&lines);
    println!("Part 1: {}", part01(&map));
    println!("Part 2: {}", part02(&map));

    Ok(())
}
