use helpers::input_lines;

const INPUT: &str = include_str!("../input.txt");

fn value_in_every_loop(subject: usize) -> impl Iterator<Item = usize> {
    std::iter::successors(Some(1), move |value| Some((value * subject) % 20_201_227))
}

fn part01(lines: &[String]) -> usize {
    let pub_key_1: usize = lines[0].parse().unwrap();
    let pub_key_2: usize = lines[1].parse().unwrap();

    let loop_size = value_in_every_loop(7)
        .take_while(|value| value != &pub_key_1)
        .count();

    value_in_every_loop(pub_key_2).nth(loop_size).unwrap()
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
