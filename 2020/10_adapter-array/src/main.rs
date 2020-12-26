use helpers::input_lines;
use std::collections::HashMap;

const INPUT: &str = include_str!("../input.txt");

fn part01(sorted_adapers: &[usize]) -> usize {
    let mut diff_1 = 0;
    let mut diff_3 = 0;

    let mut current_value = &0;

    for value in sorted_adapers {
        match value - current_value {
            1 => {
                diff_1 += 1;
                current_value = value;
            }
            3 => {
                diff_3 += 1;
                current_value = value;
            }
            _ => {}
        }
    }

    diff_1 * (diff_3 + 1)
}

fn part02(sorted_adapers: &[usize]) -> usize {
    struct CachedCountCombinations<'a> {
        input: &'a [usize],
        memoized_results: HashMap<usize, usize>,
    }
    impl<'a> CachedCountCombinations<'a> {
        fn new(input: &'a [usize]) -> Self {
            Self {
                input,
                memoized_results: HashMap::with_capacity(input.len()),
            }
        }

        fn call(&mut self, index: usize) -> usize {
            if let Some(result) = self.memoized_results.get(&index) {
                *result
            } else if index >= self.input.len() {
                0
            } else {
                let result = if index == self.input.len() - 1 {
                    1
                } else {
                    let mut result = 0;
                    for next_index in index + 1..=index + 3 {
                        if next_index < self.input.len() {
                            match self.input[next_index] - self.input[index] {
                                1 | 2 | 3 => {
                                    result += self.call(next_index);
                                }
                                _ => {}
                            }
                        }
                    }
                    result
                };
                self.memoized_results.insert(index, result);
                result
            }
        }
    }

    let mut count_combinations = CachedCountCombinations::new(sorted_adapers);

    for idx in (0..sorted_adapers.len()).rev() {
        count_combinations.call(idx);
    }

    count_combinations
        .input
        .iter()
        .enumerate()
        .take(3)
        .map(|(index, value)| {
            if value <= &3 {
                count_combinations.memoized_results[&index]
            } else {
                0
            }
        })
        .sum()
}

fn main() -> anyhow::Result<()> {
    let lines = input_lines(INPUT)?;
    let mut sorted_adapers: Vec<_> = lines.iter().map(|line| line.parse().unwrap()).collect();
    sorted_adapers.sort_unstable();

    println!("Part 1: {}", part01(&sorted_adapers));
    println!("Part 2: {}", part02(&sorted_adapers));
    Ok(())
}
