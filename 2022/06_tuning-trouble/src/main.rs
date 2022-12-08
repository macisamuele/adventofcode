use helpers::input_lines;

const INPUT: &str = include_str!("../input.txt");

fn find_start(input: &str, sequence_length: usize) -> usize {
    input
        .as_bytes()
        .windows(sequence_length)
        .enumerate()
        .find(|(_, chars)| {
            (0..(sequence_length - 1))
                .flat_map(|index1| {
                    (index1 + 1..sequence_length).map(move |index2| (index1, index2))
                })
                .all(|(index1, index2)| chars[index1] != chars[index2])
        })
        .map_or(0, |(idx, _)| idx)
        + sequence_length
}

fn part01(input: &str) -> usize {
    find_start(input, 4)
}

fn part02(input: &str) -> usize {
    find_start(input, 14)
}

fn main() -> anyhow::Result<()> {
    let input = &input_lines(INPUT)?[0];

    println!("Part 1: {}", part01(input));
    println!("Part 2: {}", part02(input));
    Ok(())
}
