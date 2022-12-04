use helpers::input_lines;

const INPUT: &str = include_str!("../input.txt");

fn part01(elf_to_calories: &[usize]) -> usize {
    *elf_to_calories.iter().max().unwrap_or(&0)
}

fn part02(elf_to_calories: &[usize]) -> usize {
    let mut elf_to_calories = elf_to_calories.to_vec();
    elf_to_calories.sort_unstable_by(|a, b| b.cmp(a));
    elf_to_calories.iter().take(3).sum()
}

fn main() -> anyhow::Result<()> {
    let elf_to_calories: Vec<usize> = {
        let mut result = vec![];
        let mut current_elf_calories: usize = 0;
        for line in input_lines(INPUT)? {
            if let Ok(calories) = line.parse::<usize>() {
                current_elf_calories += calories;
            } else if line.is_empty() {
                result.push(current_elf_calories);
                current_elf_calories = 0;
            }
        }
        result.push(current_elf_calories);
        result
    };

    println!("Part 1: {}", part01(&elf_to_calories));
    println!("Part 2: {}", part02(&elf_to_calories));
    Ok(())
}
