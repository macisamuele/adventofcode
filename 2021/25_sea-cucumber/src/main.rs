use helpers::input_lines;

const INPUT: &str = include_str!("../input.txt");

fn part01(_lines: &[String]) -> usize {
    0
}

fn part02(_lines: &[String]) -> usize {
    0
}

fn main() -> anyhow::Result<()> {
    let lines = input_lines(INPUT)?;

    println!("Part 1: {}", part01(&lines));
    println!("Part 2: {}", part02(&lines));
    Ok(())
}
