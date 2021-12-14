use helpers::input_lines;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::mem::swap;

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug)]
struct Input {
    template: Vec<char>,
    insertion_rules: HashMap<(char, char), char>,
}

impl TryFrom<Vec<String>> for Input {
    type Error = anyhow::Error;

    fn try_from(lines: Vec<String>) -> Result<Self, Self::Error> {
        anyhow::ensure!(lines.len() > 2);
        let template = lines[0].chars().collect();
        let insertion_rules = lines[2..]
            .iter()
            .map(|line| {
                if let Some((source, destination)) = line.split_once(" -> ") {
                    let bytes_source: Vec<char> = source.chars().collect();
                    anyhow::ensure!(
                        bytes_source.len() == 2,
                        "source ({source}) is expected to have only 2 elements",
                        source = source
                    );
                    let bytes_destination: Vec<char> = destination.chars().collect();
                    anyhow::ensure!(
                        destination.len() == 1,
                        "destination ({destination}) is expected to have only 2 elements",
                        destination = destination
                    );
                    Ok(((bytes_source[0], bytes_source[1]), bytes_destination[0]))
                } else {
                    Err(anyhow::anyhow!(
                        "'{line}' does not respect the expected structure 'source -> destination'",
                        line = line
                    ))
                }
            })
            .collect::<Result<HashMap<(char, char), char>, _>>()?;
        Ok(Self {
            template,
            insertion_rules,
        })
    }
}

impl Input {
    fn run_iterations(&self, iteration_count: usize) -> HashMap<(char, char), usize> {
        // Running for real all the iterations would be "easy" from an implementiation
        // prespective but orrible from a memory prespective as the memory needed to
        // keep the updating base (elements) would double at every iteration leading
        // to O(KB) in Part 1 and to O(TB) in Part 2
        // In order to make it scalable we are keeping track of the count of the different
        // pairs (we cannot have more than 26*26=676 pairs)

        let mut counters = self.template.windows(2).fold(
            HashMap::default(),
            |mut counters: HashMap<_, _>, pair| {
                *counters.entry((pair[0], pair[1])).or_default() += 1;
                counters
            },
        );
        let mut tmp_counters = HashMap::default();

        for _ in 0..iteration_count {
            tmp_counters.clear();
            for (pair, count) in &counters {
                if let Some(insertion) = self.insertion_rules.get(pair) {
                    *tmp_counters.entry((pair.0, *insertion)).or_default() += count;
                    *tmp_counters.entry((*insertion, pair.1)).or_default() += count;
                }
            }
            // Using swap instead of copying the elements to avoid copying
            // all the elements all the time but rather we flip the "pointers"
            swap(&mut counters, &mut tmp_counters);
        }

        counters
    }

    fn occurrences_after_iterations(&self, iteration_count: usize) -> HashMap<char, usize> {
        let mut result = HashMap::new();
        result.insert(
            *self
                .template
                .last()
                .expect("Initial template has more than 1 element"),
            1,
        );
        for (pair, count) in self.run_iterations(iteration_count) {
            *result.entry(pair.0).or_default() += count;
        }
        result
    }
}

fn part01(input: &Input) -> usize {
    let final_occurrences = input.occurrences_after_iterations(10);

    let min = final_occurrences.values().min().unwrap_or(&0);
    let max = final_occurrences.values().max().unwrap_or(&usize::MAX);

    max - min
}

fn part02(input: &Input) -> usize {
    let final_occurrences = input.occurrences_after_iterations(40);

    let min = final_occurrences.values().min().unwrap_or(&0);
    let max = final_occurrences.values().max().unwrap_or(&usize::MAX);

    max - min
}

fn main() -> anyhow::Result<()> {
    let input = Input::try_from(input_lines(INPUT)?)?;

    println!("Part 1: {}", part01(&input));
    println!("Part 2: {}", part02(&input));
    Ok(())
}
