use helpers::input_lines;
use std::collections::{HashMap, VecDeque};

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug)]
struct CircularBuffer<T> {
    values: VecDeque<T>,
    capacity: usize,
}

impl<T: Eq> CircularBuffer<T> {
    fn new(capacity: usize) -> Self {
        Self {
            values: VecDeque::new(),
            capacity,
        }
    }

    fn add(&mut self, value: T) {
        if self.values.len() >= self.capacity {
            self.values.pop_front();
        }
        self.values.push_back(value);
    }

    fn last(&self) -> Option<&T> {
        self.values.back()
    }

    fn first(&self) -> Option<&T> {
        self.values.front()
    }

    fn len(&self) -> usize {
        self.values.len()
    }
}

fn game(input: &[usize], turns: usize) -> usize {
    if turns < input.len() {
        input[turns]
    } else {
        let mut values: Vec<usize> = vec![0; turns];
        let mut value_to_turns: HashMap<usize, CircularBuffer<usize>> = HashMap::new();

        macro_rules! play {
            ($turn:expr, $value: expr) => {
                values[$turn] = $value;
                value_to_turns
                    .entry($value)
                    .or_insert_with(|| CircularBuffer::new(2))
                    .add($turn);
            };
        }

        for (turn, value) in input.iter().enumerate() {
            play!(turn, *value);
        }

        for turn in input.len()..turns {
            let last = values[(turn - 1) as usize];
            match value_to_turns.get(&last) {
                Some(turns) if turns.len() > 1 => {
                    let last_turn = *turns.last().unwrap();
                    let first_turn = *turns.first().unwrap();
                    play!(turn, last_turn - first_turn);
                }
                _ => {
                    play!(turn, 0);
                }
            }
        }
        *values.last().unwrap()
    }
}

fn part01(input: &[usize]) -> usize {
    game(input, 2_020)
}

fn part02(input: &[usize]) -> usize {
    game(input, 30_000_000)
}

fn main() -> anyhow::Result<()> {
    let lines = input_lines(INPUT)?;
    let input: Vec<usize> = lines[0]
        .split(',')
        .map(str::parse)
        .collect::<Result<_, _>>()?;

    println!("Part 1: {}", part01(&input));
    println!("Part 2: {}", part02(&input));
    Ok(())
}
