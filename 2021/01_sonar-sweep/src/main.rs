use helpers::input_lines;

const INPUT: &str = include_str!("../input.txt");

fn part01(input: &[usize]) -> usize {
    input
        .windows(2)
        .map(|elements| {
            if elements.last().unwrap() > elements.first().unwrap() {
                1
            } else {
                0
            }
        })
        .sum()
}

fn part02(input: &[usize]) -> usize {
    input
        .windows(4)
        .map(|elements| {
            if elements.last().unwrap() > elements.first().unwrap() {
                1
            } else {
                0
            }
        })
        .sum()
}

fn main() -> anyhow::Result<()> {
    let input: Vec<usize> = input_lines(INPUT)?
        .iter()
        .filter_map(|line| line.parse().ok())
        .collect();

    println!("Part 1: {}", part01(&input));
    println!("Part 2: {}", part02(&input));
    Ok(())
}
