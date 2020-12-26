use std::ops::Index;

use helpers::input_lines;

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, Eq, PartialEq)]
enum Cell {
    Open,
    Tree,
}

#[derive(Debug)]
struct World {
    height: usize,
    map: Vec<Vec<Cell>>,
    width: usize,
}

impl World {
    fn new(lines: &[String]) -> Self {
        let map: Vec<Vec<Cell>> = lines
            .iter()
            .map(|line| {
                line.chars()
                    .map(|c| if c == '.' { Cell::Open } else { Cell::Tree })
                    .collect()
            })
            .collect();
        let height = map.len();
        let width = map[0].len();

        Self { height, map, width }
    }

    fn count_trees(&self, slope: &(usize, usize)) -> usize {
        let mut index = (0, 0);
        let mut trees_count = 0;

        while index.0 != self.height - 1 {
            index.0 += slope.0;
            index.1 += slope.1;
            if self[index] == Cell::Tree {
                trees_count += 1;
            }
        }
        trees_count
    }
}

impl Index<(usize, usize)> for World {
    type Output = Cell;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (row, column) = index;
        &self.map[row][column % self.width]
    }
}

fn part01(world: &World) -> usize {
    world.count_trees(&(1, 3))
}

fn part02(world: &World) -> usize {
    let slopes = [(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];
    slopes
        .iter()
        .map(|slope| world.count_trees(slope))
        .product()
}

fn main() -> anyhow::Result<()> {
    let lines = input_lines(INPUT)?;
    let world = World::new(&lines);
    println!("Part 1: {}", part01(&world));
    println!("Part 2: {}", part02(&world));
    Ok(())
}
