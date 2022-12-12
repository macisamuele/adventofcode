use helpers::input_lines;
use std::collections::HashSet;
use std::convert::{TryFrom, TryInto};
use std::str::FromStr;

const ALGORITHM_LENGTH: usize = 512;
const INPUT: &str = include_str!("../input.txt");

#[inline]
fn cast_to_isize(value: usize) -> isize {
    value
        .try_into()
        .expect("Expected to be able to convert value")
}

#[inline]
fn is_light_color(pixel_color: u8) -> Result<bool, anyhow::Error> {
    match pixel_color {
        b'#' => Ok(true),
        b'.' => Ok(false),
        _ => Err(anyhow::anyhow!(
            "Unrecognized pixel color: {pixel_color}",
            pixel_color = pixel_color
        )),
    }
}

#[derive(Debug)]
struct ImageEnhancementAlgorithm {
    light_positions: HashSet<usize>,
}

impl FromStr for ImageEnhancementAlgorithm {
    type Err = anyhow::Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let pixel_colors = line.as_bytes();
        anyhow::ensure!(pixel_colors.len() == ALGORITHM_LENGTH);

        let light_positions: HashSet<usize> = pixel_colors
            .iter()
            .enumerate()
            .filter_map(|(index, pixel_color)| match is_light_color(*pixel_color) {
                Ok(true) => Some(Ok(index)),
                Ok(false) => None,
                Err(err) => Some(Err(err)),
            })
            .collect::<Result<_, _>>()?;

        Ok(Self { light_positions })
    }
}

#[derive(Debug)]
struct Square {
    value: usize,
}

impl From<[[bool; 3]; 3]> for Square {
    fn from(values: [[bool; 3]; 3]) -> Self {
        let value = (u16::from(values[0][0]) << 8)
            | (u16::from(values[0][1]) << 7)
            | (u16::from(values[0][2]) << 6)
            | (u16::from(values[1][0]) << 5)
            | (u16::from(values[1][1]) << 4)
            | (u16::from(values[1][2]) << 3)
            | (u16::from(values[2][0]) << 2)
            | (u16::from(values[2][1]) << 1)
            | u16::from(values[2][2]);

        Self {
            value: value as usize,
        }
    }
}

#[derive(Clone, Debug)]
struct Ranges {
    min_row: isize,
    max_row: isize,
    min_column: isize,
    max_column: isize,
}

impl Default for Ranges {
    fn default() -> Self {
        Self {
            min_row: isize::MAX,
            max_row: isize::MIN,
            min_column: isize::MAX,
            max_column: isize::MIN,
        }
    }
}

#[derive(Clone, Debug)]
struct Image {
    light_positions: HashSet<(isize, isize)>,
    ranges: Ranges,
    is_light_outside_ranges: bool,
}

impl TryFrom<&[String]> for Image {
    type Error = anyhow::Error;

    fn try_from(lines: &[String]) -> Result<Self, Self::Error> {
        anyhow::ensure!(lines.len() > 0);
        let light_positions = lines
            .iter()
            .enumerate()
            .flat_map(|(line_number, line)| {
                line.bytes()
                    .enumerate()
                    .filter_map(move |(column_number, pixel_color)| {
                        match is_light_color(pixel_color) {
                            Ok(true) => Some(Ok((
                                cast_to_isize(line_number),
                                cast_to_isize(column_number),
                            ))),
                            Ok(false) => None,
                            Err(err) => Some(Err(err)),
                        }
                    })
            })
            .collect::<Result<_, _>>()?;

        let mut res = Self {
            light_positions,
            ranges: Ranges::default(),
            is_light_outside_ranges: false,
        };
        res.update_row_column_ranges();
        Ok(res)
    }
}

#[derive(Debug)]
struct Input {
    image_enhancement_algorithm: ImageEnhancementAlgorithm,
    base_image: Image,
}

impl TryFrom<Vec<String>> for Input {
    type Error = anyhow::Error;

    fn try_from(lines: Vec<String>) -> Result<Self, Self::Error> {
        anyhow::ensure!(lines.len() > 3);
        Ok(Self {
            image_enhancement_algorithm: lines[0].parse()?,
            base_image: Image::try_from(&lines[2..])?,
        })
    }
}

impl ImageEnhancementAlgorithm {
    fn is_light(&self, position: usize) -> bool {
        debug_assert!(position < 512);
        self.light_positions.contains(&position)
    }
}

impl Square {
    fn to_value(&self) -> usize {
        self.value as usize
    }
}

impl Image {
    fn update_row_column_ranges(&mut self) {
        self.ranges = self.light_positions.iter().fold(
            Ranges::default(),
            |mut ranges, (row_index, column_index)| {
                ranges.min_row = ranges.min_row.min(*row_index);
                ranges.max_row = ranges.max_row.max(*row_index);
                ranges.min_column = ranges.min_column.min(*column_index);
                ranges.max_column = ranges.max_column.max(*column_index);
                ranges
            },
        );
    }

    fn is_light(&self, row: isize, column: isize) -> bool {
        let ranges = &self.ranges;

        if (ranges.min_row..=ranges.max_row).contains(&row)
            && (ranges.min_column..=ranges.max_column).contains(&column)
        {
            self.light_positions.contains(&(row, column))
        } else {
            self.is_light_outside_ranges
        }
    }

    fn get_square(&self, row: isize, column: isize) -> Square {
        Square::from([
            [
                self.is_light(row - 1, column - 1),
                self.is_light(row - 1, column),
                self.is_light(row - 1, column + 1),
            ],
            [
                self.is_light(row, column - 1),
                self.is_light(row, column),
                self.is_light(row, column + 1),
            ],
            [
                self.is_light(row + 1, column - 1),
                self.is_light(row + 1, column),
                self.is_light(row + 1, column + 1),
            ],
        ])
    }

    fn apply_image_enhancement_algorithm(
        &mut self,
        algorithm: &ImageEnhancementAlgorithm,
        count: usize,
    ) {
        for _ in 0..count {
            let ranges = &self.ranges;
            let light_positions = ((ranges.min_row - 1)..=(ranges.max_row + 1))
                .flat_map(|row_index| {
                    ((ranges.min_column - 1)..=(ranges.max_column + 1))
                        .filter_map(|column_index| {
                            if algorithm
                                .is_light(self.get_square(row_index, column_index).to_value())
                            {
                                Some((row_index, column_index))
                            } else {
                                None
                            }
                        })
                        .collect::<HashSet<_>>()
                })
                .collect();
            self.is_light_outside_ranges = if self.is_light_outside_ranges {
                algorithm.is_light(ALGORITHM_LENGTH - 1)
            } else {
                algorithm.is_light(0)
            };
            self.light_positions = light_positions;
            self.update_row_column_ranges();
        }
    }

    fn light_pixels_count(&self) -> usize {
        self.light_positions.len()
    }
}

fn part01(input: &Input) -> usize {
    let mut processed_image = input.base_image.clone();
    processed_image.apply_image_enhancement_algorithm(&input.image_enhancement_algorithm, 2);
    processed_image.light_pixels_count()
}

fn part02(input: &Input) -> usize {
    let mut processed_image = input.base_image.clone();
    processed_image.apply_image_enhancement_algorithm(&input.image_enhancement_algorithm, 50);
    processed_image.light_pixels_count()
}

impl std::fmt::Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ranges = &self.ranges;
        for row_index in ranges.min_row..=ranges.max_row {
            for column_index in ranges.min_column..=ranges.max_column {
                write!(
                    f,
                    "{}",
                    if self.is_light(row_index, column_index) {
                        '#'
                    } else {
                        '.'
                    }
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    let input = Input::try_from(input_lines(INPUT)?)?;

    println!("Part 1: 5249 {}", part01(&input));
    println!("Part 2: 15714 {}", part02(&input));

    Ok(())
}
