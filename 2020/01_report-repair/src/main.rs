use helpers::input_lines;
use std::cmp::Ordering;
use std::collections::HashSet;

const INPUT: &str = include_str!("../input.txt");

fn part01(numbers: &[i32]) -> i32 {
    let mut left_index = 0;
    let mut right_index = numbers.len() - 1;

    loop {
        let left = numbers[left_index];
        let right = numbers[right_index];
        match (left + right).cmp(&2020) {
            Ordering::Equal => {
                return numbers[left_index] * numbers[right_index];
            }
            Ordering::Less => {
                left_index += 1;
            }
            Ordering::Greater => {
                right_index -= 1;
            }
        }
    }
}

fn part02(numbers: &[i32]) -> i32 {
    let all_numbers: HashSet<_> = numbers.iter().collect();

    for left_index in 0..(numbers.len()) {
        for right_index in left_index..(numbers.len()) {
            let left = numbers[left_index];
            let right = numbers[right_index];
            let other = 2020 - left - right;
            if all_numbers.contains(&other) && left + right + other == 2020 {
                return left * right * other;
            }
        }
    }
    0
}

fn main() -> anyhow::Result<()> {
    let mut numbers: Vec<_> = input_lines(INPUT)?
        .iter()
        .filter_map(|value| value.parse::<i32>().ok())
        .collect();
    numbers.sort_unstable();

    println!("Part 1: {}", part01(&numbers));
    println!("Part 2: {}", part02(&numbers));
    Ok(())
}
