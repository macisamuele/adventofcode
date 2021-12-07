use helpers::input_lines;

const INPUT: &str = include_str!("../input.txt");

fn part01(values: &[usize]) -> usize {
    let mid_index = values.len() / 2;
    let median = values[mid_index];

    values
        .iter()
        .map(|value| {
            if value > &median {
                value - median
            } else {
                median - value
            }
        })
        .sum()
}

fn part02(values: &[usize]) -> usize {
    let (sum, count) = values
        .iter()
        .fold((0, 0), |(sum, count), value| (sum + value, count + 1));
    let ceil_average = sum / count;
    (ceil_average..=ceil_average + 1)
        .map(|target| {
            values
                .iter()
                .map(|value| {
                    let difference = if value > &target {
                        value - target
                    } else {
                        target - value
                    };
                    difference * (difference + 1) / 2
                })
                .sum::<usize>()
        })
        .min()
        .expect("As values are present, a result shoud exists")
}

fn main() -> anyhow::Result<()> {
    let lines = input_lines(INPUT)?;
    anyhow::ensure!(lines.len() == 1);
    let values = {
        let mut tmp: Vec<usize> = lines[0]
            .split(',')
            .map(str::parse)
            .collect::<Result<_, _>>()?;
        tmp.sort_unstable();
        tmp
    };
    anyhow::ensure!(!values.is_empty());

    println!("Part 1: {}", part01(&values));
    println!("Part 2: {}", part02(&values));

    Ok(())
}
