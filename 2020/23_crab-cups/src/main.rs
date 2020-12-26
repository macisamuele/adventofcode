use helpers::input_lines;

const INPUT: &str = include_str!("../input.txt");

struct GameTable {
    current_value: usize,
    index_to_next: Vec<usize>,
}

impl GameTable {
    fn new(values: &[usize], count: usize) -> Self {
        let mut index_to_next = vec![0; count + 1];
        let mut max_value = values[0];
        for (index, value) in values.iter().take(values.len() - 1).enumerate() {
            index_to_next[*value] = values[index + 1];
            if *value > max_value {
                max_value = *value;
            }
        }

        #[allow(clippy::needless_range_loop)]
        for value in (max_value + 1)..=count {
            index_to_next[value] = value + 1;
        }

        if count > max_value {
            index_to_next[values[values.len() - 1]] = max_value + 1;
            index_to_next[count] = values[0];
        } else {
            index_to_next[values[values.len() - 1]] = values[0];
        }
        index_to_next.shrink_to_fit();

        let mut sorted_values = values.to_owned();
        sorted_values.sort_unstable();

        Self {
            index_to_next,
            current_value: values[0],
        }
    }

    fn iter(&self, first_value: usize) -> impl Iterator<Item = usize> + '_ {
        std::iter::successors(Some(first_value), move |index| {
            let next_value = self.index_to_next[*index];
            if next_value == first_value {
                None
            } else {
                Some(next_value)
            }
        })
    }

    fn predecessor_value_iter(&self, first_value: usize) -> impl Iterator<Item = usize> + '_ {
        std::iter::successors(Some(first_value), move |value| {
            if value > &1 {
                Some(value - 1)
            } else {
                Some(self.index_to_next.len() - 1)
            }
        })
    }

    fn round(&mut self) {
        let current_value = self.current_value;
        let next_value1 = self.index_to_next[self.current_value];
        let next_value2 = self.index_to_next[next_value1];
        let next_value3 = self.index_to_next[next_value2];

        let destination = self
            .predecessor_value_iter(current_value)
            .skip(1) // Skip the first value because it would be the current_value
            .find(|value| {
                // We don't check for current value because it would mean an entire cycle
                // and we are sure that we have found such value before hand
                value != &next_value1 && value != &next_value2 && value != &next_value3
            })
            .unwrap();

        self.index_to_next[current_value] = self.index_to_next[next_value3];
        self.index_to_next[next_value3] = self.index_to_next[destination];
        self.index_to_next[destination] = next_value1;

        self.current_value = self.index_to_next[self.current_value];
    }

    fn cups_in_string(&self, first_value: usize) -> String {
        self.iter(first_value)
            .map(|value| value.to_string())
            .collect()
    }

    fn play(&mut self, n_rounds: usize) {
        for _ in 0..n_rounds {
            self.round();
        }
    }
}

impl std::fmt::Display for GameTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for value in self.iter(self.current_value) {
            if value == self.current_value {
                write!(f, "({})", value)?;
            } else {
                write!(f, " {} ", value)?;
            }
        }
        Ok(())
    }
}

fn part01(line: &str) -> String {
    let mut game = GameTable::new(
        line.as_bytes()
            .iter()
            .map(|b| (b - b'0') as usize)
            .collect::<Vec<_>>()
            .as_slice(),
        9,
    );

    game.play(100);

    game.cups_in_string(1)[1..].to_string()
}

fn part02(line: &str) -> usize {
    let mut game = GameTable::new(
        line.as_bytes()
            .iter()
            .map(|b| (b - b'0') as usize)
            .collect::<Vec<_>>()
            .as_slice(),
        1_000_000,
    );

    game.play(10_000_000);

    game.iter(1).skip(1).take(2).product()
}

fn main() -> anyhow::Result<()> {
    let lines = input_lines(INPUT)?;
    println!("Part 1: {}", part01(&lines[0]));
    println!("Part 2: {}", part02(&lines[0]));

    Ok(())
}
