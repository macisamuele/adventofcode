use helpers::input_lines;
use scan_fmt::scan_fmt;
use std::collections::HashMap;

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug)]
struct Passport {
    fields: HashMap<String, String>,
}

impl Passport {
    const REQUIRED_FIELDS: [&'static str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    fn is_field_valid(&self, field_name: &str) -> Option<()> {
        fn is_in_range(value: i32, min: i32, max: i32) -> Option<()> {
            if value >= min && value <= max {
                Some(())
            } else {
                None
            }
        }
        fn is_integer_in_range(value: &str, min: i32, max: i32) -> Option<()> {
            is_in_range(value.parse::<i32>().ok()?, min, max)
        }

        let field_value = self.fields.get(field_name)?;
        match field_name {
            "byr" => is_integer_in_range(field_value, 1920, 2002),
            "iyr" => is_integer_in_range(field_value, 2010, 2020),
            "eyr" => is_integer_in_range(field_value, 2020, 2030),
            "hgt" => {
                if field_value.ends_with("cm") {
                    is_in_range(scan_fmt!(field_value, "{}cm", i32).ok()?, 150, 193)
                } else if field_value.ends_with("in") {
                    is_in_range(scan_fmt!(field_value, "{}in", i32).ok()?, 59, 76)
                } else {
                    None
                }
            }
            "hcl" => {
                if field_value.len() == 7
                    && field_value.starts_with('#')
                    && field_value
                        .chars()
                        .skip(1)
                        .all(|c| matches!(c, '0'..='9' | 'a'..='f'))
                {
                    Some(())
                } else {
                    None
                }
            }
            "ecl" => match field_value.as_str() {
                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => Some(()),
                _ => None,
            },
            "pid" => {
                if field_value.len() == 9 && field_value.chars().all(|c| matches!(c, '0'..='9')) {
                    Some(())
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn is_valid(&self, only_check_presence: bool) -> bool {
        if only_check_presence {
            Self::REQUIRED_FIELDS
                .iter()
                .all(|field| self.fields.contains_key(*field))
        } else {
            Self::REQUIRED_FIELDS
                .iter()
                .all(|field| self.is_field_valid(field).is_some())
        }
    }
}

struct PassportReader<'a> {
    text_iter: std::slice::Iter<'a, String>,
}

impl Iterator for PassportReader<'_> {
    type Item = Passport;

    fn next(&mut self) -> Option<Self::Item> {
        let mut passport_fields = HashMap::new();
        loop {
            let next_line = self.text_iter.next().map_or_else(|| "", String::as_str);
            if next_line.is_empty() {
                break;
            }
            for part in next_line.split(' ') {
                let (k, v) = scan_fmt!(part, "{}:{}", String, String).unwrap();
                passport_fields.insert(k, v);
            }
        }

        if passport_fields.is_empty() {
            None
        } else {
            Some(Passport {
                fields: passport_fields,
            })
        }
    }
}

fn part01(passports: &[Passport]) -> usize {
    passports.iter().filter(|p| p.is_valid(true)).count()
}

fn part02(passports: &[Passport]) -> usize {
    passports.iter().filter(|p| p.is_valid(false)).count()
}

fn main() -> anyhow::Result<()> {
    let lines = input_lines(INPUT)?;
    let passports: Vec<_> = PassportReader {
        text_iter: lines.iter(),
    }
    .collect();

    println!("Part 1: {}", part01(&passports));
    println!("Part 2: {}", part02(&passports));
    Ok(())
}
