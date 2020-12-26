use scan_fmt::scan_fmt;

use helpers::input_lines;

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug)]
struct PasswordPolicyPart1 {
    min: usize,
    max: usize,
    character: char,
}

impl PasswordPolicyPart1 {
    fn is_password_good(&self, password: &str) -> bool {
        let occurrences = password.chars().filter(|c| &self.character == c).count();
        self.min <= occurrences && occurrences <= self.max
    }
}

fn part01(lines: &[String]) -> usize {
    lines
        .iter()
        .filter(|line| {
            let (min, max, character, password) =
                scan_fmt!(line, "{}-{} {}: {}", usize, usize, char, String).unwrap();
            PasswordPolicyPart1 {
                min,
                max,
                character,
            }
            .is_password_good(&password)
        })
        .count()
}

#[derive(Debug)]
struct PasswordPolicyPart2 {
    indexes: [usize; 2],
    character: char,
}

impl PasswordPolicyPart2 {
    fn is_password_good(&self, password: &str) -> bool {
        self.indexes
            .iter()
            .filter(|index| password.chars().nth(*index - 1) == Some(self.character))
            .count()
            == 1
    }
}

fn part02(lines: &[String]) -> usize {
    lines
        .iter()
        .filter(|line| {
            let (first_index, second_index, character, password) =
                scan_fmt!(line, "{}-{} {}: {}", usize, usize, char, String).unwrap();
            PasswordPolicyPart2 {
                indexes: [first_index, second_index],
                character,
            }
            .is_password_good(&password)
        })
        .count()
}

fn main() -> anyhow::Result<()> {
    let lines = input_lines(INPUT)?;
    println!("Part 1: {}", part01(&lines));
    println!("Part 2: {}", part02(&lines));
    Ok(())
}
