use helpers::input_lines;
use std::cmp::Ordering;
use std::collections::VecDeque;

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug)]
struct CircularBuffer<T> {
    values: VecDeque<T>,
    capacity: usize,
}

impl<T> CircularBuffer<T> {
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
        self.values.push_back(value)
    }

    fn iter(&self) -> impl Iterator<Item = &T> {
        self.values.iter()
    }
}

fn two_sum<'a, I: Iterator<Item = &'a i64>>(values_iter: I, target: &i64) -> bool {
    let mut values: Vec<_> = values_iter.collect();
    values.sort();

    let mut left_index = 0;
    let mut right_index = values.len() - 1;

    while left_index != right_index {
        let left = values[left_index];
        let right = values[right_index];
        match (left + right).cmp(target) {
            Ordering::Equal => {
                return true;
            }
            Ordering::Less => {
                left_index += 1;
            }
            Ordering::Greater => {
                right_index -= 1;
            }
        }
    }

    false
}

fn contigous_numbers_with_sum(values: &[&i64], target: i64) -> Vec<i64> {
    for first_index in 0..values.len() - 1 {
        for last_index in (first_index + 1)..values.len() {
            if values[first_index..=last_index]
                .iter()
                .map(|value| **value)
                .sum::<i64>()
                == target
            {
                return values[first_index..=last_index]
                    .iter()
                    .map(|value| **value)
                    .collect();
            }
        }
    }
    vec![0; 2]
}

fn part01(values: &[i64]) -> i64 {
    const COUNT: usize = 25;
    let mut buffer = CircularBuffer::<i64>::new(COUNT);
    values.iter().take(COUNT).for_each(|value| {
        buffer.add(*value);
    });

    for value in values.iter().skip(COUNT) {
        if !two_sum(buffer.iter(), value) {
            return *value;
        }
        buffer.add(*value);
    }
    0
}

fn part02(values: &[i64]) -> i64 {
    const COUNT: usize = 25;
    let mut buffer = CircularBuffer::<i64>::new(COUNT);

    values.iter().take(COUNT).for_each(|value| {
        buffer.add(*value);
    });

    for (idx, value) in values.iter().skip(COUNT).enumerate() {
        if !two_sum(buffer.iter(), value) {
            let contigous_numbers = contigous_numbers_with_sum(
                values
                    .iter()
                    .take(idx + COUNT)
                    .collect::<Vec<_>>()
                    .as_slice(),
                *value,
            );
            return contigous_numbers.iter().min().unwrap()
                + contigous_numbers.iter().max().unwrap();
        }
        buffer.add(*value);
    }
    0
}

fn main() -> anyhow::Result<()> {
    let lines = input_lines(INPUT)?;
    let values: Vec<i64> = lines.iter().map(|line| line.parse().unwrap()).collect();

    println!("Part 1: {}", part01(&values));
    println!("Part 2: {}", part02(&values));
    Ok(())
}
