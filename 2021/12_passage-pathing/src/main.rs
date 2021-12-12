use helpers::input_lines;
use scan_fmt::scan_fmt;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::str::FromStr;

const INPUT: &str = include_str!("../input.txt");

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
enum Cell {
    Start,
    End,
    BigCave(String),
    SmallCave(String),
}

impl FromStr for Cell {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "start" => Ok(Self::Start),
            "end" => Ok(Self::End),
            value if value.chars().all(char::is_uppercase) => Ok(Self::BigCave(value.to_string())),
            value if value.chars().all(char::is_lowercase) => {
                Ok(Self::SmallCave(value.to_string()))
            }
            value => Err(anyhow::anyhow!("Unrecognised cell: {value}", value = value)),
        }
    }
}

type CellId = usize;

#[derive(Debug)]
struct Graph {
    cells: Vec<Cell>,
    links: HashMap<CellId, Vec<CellId>>,
}

impl Graph {
    fn cell_id(&mut self, cell: Cell) -> CellId {
        let maybe_index = self
            .cells
            .iter()
            .enumerate()
            .find_map(|(index, know_cell)| {
                if &cell == know_cell {
                    Some(index)
                } else {
                    None
                }
            });

        if let Some(index) = maybe_index {
            index
        } else {
            self.cells.push(cell);
            self.cells.len() - 1
        }
    }

    fn register_link(&mut self, cell1: Cell, cell2: Cell) {
        let cell_id1 = self.cell_id(cell1);
        let cell_id2 = self.cell_id(cell2);
        self.links.entry(cell_id1).or_default().push(cell_id2);
        self.links.entry(cell_id2).or_default().push(cell_id1);
    }

    fn cell(&self, cell_id: CellId) -> &Cell {
        &self.cells[cell_id]
    }

    fn find_all_paths(&self, allow_double_small_cavern_visit: bool) -> Vec<Vec<CellId>> {
        fn recurse(
            graph: &Graph,
            known_paths: &mut Vec<Vec<CellId>>,
            current_path: &mut Vec<CellId>,
            allow_double_small_cavern_visit: bool,
        ) {
            let last_cell_id = current_path
                .last()
                .expect("Current path is expected to have at least one cell");

            if last_cell_id == &1 {
                known_paths.push(current_path.clone());
            } else {
                for neighbour_cell_id in &graph.links[last_cell_id] {
                    let (is_multiple_small_cavern_visit, should_add) =
                        if current_path.contains(neighbour_cell_id) {
                            match graph.cell(*neighbour_cell_id) {
                                Cell::Start => (false, false),
                                Cell::SmallCave(_) => {
                                    let is_small_cavern_already_visited = current_path
                                        .iter()
                                        .filter(|cell_id| cell_id == &neighbour_cell_id)
                                        .count()
                                        > 0;

                                    if is_small_cavern_already_visited {
                                        if allow_double_small_cavern_visit {
                                            (true, true)
                                        } else {
                                            (true, false)
                                        }
                                    } else {
                                        (false, true)
                                    }
                                }
                                Cell::End | Cell::BigCave(_) => (false, true),
                            }
                        } else {
                            (false, true)
                        };

                    if should_add {
                        current_path.push(*neighbour_cell_id);
                        recurse(
                            graph,
                            known_paths,
                            current_path,
                            allow_double_small_cavern_visit && !is_multiple_small_cavern_visit,
                        );
                        current_path.pop();
                    }
                }
            }
        }

        let mut known_paths = vec![];

        // Pre-allocate the vector containing the current path to a length
        // which would reduce allocation along the way.
        // Using 2 times the number of cells is just an empirical measure
        let mut current_path = Vec::with_capacity(self.cells.len() * 2);
        current_path.push(0);
        recurse(
            self,
            &mut known_paths,
            &mut current_path,
            allow_double_small_cavern_visit,
        );

        known_paths
    }
}

impl Default for Graph {
    fn default() -> Self {
        Self {
            cells: vec![Cell::Start, Cell::End],
            links: HashMap::new(),
        }
    }
}

impl TryFrom<Vec<String>> for Graph {
    type Error = anyhow::Error;

    fn try_from(lines: Vec<String>) -> Result<Self, Self::Error> {
        let mut graph = Graph::default();

        for line in &lines {
            let (cell1_str, cell2_str) = scan_fmt!(line, "{}-{}", String, String)?;
            graph.register_link(cell1_str.parse()?, cell2_str.parse()?);
        }

        Ok(graph)
    }
}

fn part01(graph: &Graph) -> usize {
    graph.find_all_paths(false).len()
}

fn part02(graph: &Graph) -> usize {
    graph.find_all_paths(true).len()
}

fn main() -> anyhow::Result<()> {
    let graph: Graph = Graph::try_from(input_lines(INPUT)?)?;

    println!("Part 1: {}", part01(&graph));
    println!("Part 2: {}", part02(&graph));

    Ok(())
}
