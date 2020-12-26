use helpers::input_lines;
use scan_fmt::scan_fmt;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::ops::RangeInclusive;

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug)]
struct Ticket {
    field_values: Vec<usize>,
}

impl Ticket {
    fn invalid_field_values(
        &self,
        field_to_ranges: &HashMap<&str, Vec<RangeInclusive<usize>>>,
    ) -> Vec<usize> {
        self.field_values
            .iter()
            .filter_map(|field| {
                if field_to_ranges
                    .values()
                    .any(|ranges| ranges.iter().any(|range| range.contains(field)))
                {
                    None
                } else {
                    Some(*field)
                }
            })
            .collect()
    }

    fn is_valid(&self, field_to_ranges: &HashMap<&str, Vec<RangeInclusive<usize>>>) -> bool {
        self.field_values.iter().all(|field| {
            field_to_ranges
                .values()
                .any(|ranges| ranges.iter().any(|range| range.contains(field)))
        })
    }

    fn possible_columns_for_fields<'a>(
        &self,
        field_to_ranges: &HashMap<&'a str, Vec<RangeInclusive<usize>>>,
    ) -> Option<Vec<HashSet<&'a str>>> {
        if self.is_valid(field_to_ranges) {
            // Ensure that we operate only on valid rows
            let mut field_to_possible_columns: Vec<HashSet<&'a str>> =
                vec![HashSet::new(); self.field_values.len()];
            for (index, field_value) in self.field_values.iter().enumerate() {
                for (field_name, ranges) in field_to_ranges.iter() {
                    if ranges.iter().any(|range| range.contains(field_value)) {
                        field_to_possible_columns[index].insert(field_name);
                    }
                }
            }
            Some(field_to_possible_columns)
        } else {
            // The row is not valid
            None
        }
    }
}

impl From<&String> for Ticket {
    fn from(line: &String) -> Ticket {
        Ticket {
            field_values: line
                .split(',')
                .filter_map(|part| part.parse().ok())
                .collect(),
        }
    }
}

#[derive(Debug)]
struct Input<'a> {
    field_to_ranges: HashMap<&'a str, Vec<RangeInclusive<usize>>>,
    nearby_tickets: Vec<Ticket>,
    ticket: Ticket,
}

impl<'a> From<&'a Vec<String>> for Input<'a> {
    fn from(lines: &'a Vec<String>) -> Self {
        let mut lines_iter = lines.iter();

        let field_to_ranges = lines_iter
            .by_ref()
            .take_while(|line| !line.is_empty())
            .map(|line| {
                let line_parts: Vec<_> = line.split(": ").collect();
                let field = line_parts[0];
                let ranges = line_parts[1]
                    .split(" or ")
                    .map(|range| {
                        let (start, end) = scan_fmt!(range, "{}-{}", usize, usize).unwrap();
                        RangeInclusive::new(start, end)
                    })
                    .collect();
                (field, ranges)
            })
            .collect();

        let ticket = lines_iter
            .by_ref()
            .take_while(|line| !line.is_empty())
            .skip(1) // contains "your ticket:""
            .map(Ticket::from)
            .collect::<Vec<_>>()
            .into_iter()
            .next()
            .unwrap();

        let nearby_tickets = lines_iter
            .by_ref()
            .take_while(|line| !line.is_empty())
            .skip(1) // contains "nearby tickets:"
            .map(Ticket::from)
            .collect();

        Self {
            field_to_ranges,
            nearby_tickets,
            ticket,
        }
    }
}

fn part01(input: &Input) -> usize {
    input
        .nearby_tickets
        .iter()
        .map(|ticket| {
            ticket
                .invalid_field_values(&input.field_to_ranges)
                .iter()
                .sum::<usize>()
        })
        .sum()
}

fn part02(input: &Input) -> usize {
    fn update_possible_fields_per_column<'a>(
        ticket: &Ticket,
        input: &Input,
        possible_fields_per_column: &mut BTreeMap<usize, HashSet<&'a str>>,
        certanly_allocated_field_names: &mut HashSet<&'a str>,
    ) {
        if let Some(possible_columns_for_fields) =
            ticket.possible_columns_for_fields(&input.field_to_ranges)
        {
            for (general_possible_fields, possible_field_for_ticket) in possible_fields_per_column
                .values_mut()
                .zip(possible_columns_for_fields.iter())
            {
                general_possible_fields.retain(|value| possible_field_for_ticket.contains(value));
                if general_possible_fields.len() == 1 {
                    certanly_allocated_field_names
                        .insert(*general_possible_fields.iter().next().unwrap());
                }
            }

            loop {
                let mut should_break = true;
                for general_possible_fields in possible_fields_per_column.values_mut() {
                    if general_possible_fields.len() > 1 {
                        general_possible_fields
                            .retain(|value| !certanly_allocated_field_names.contains(value));
                        if general_possible_fields.len() == 1 {
                            // As removing the other certain fields we have only one
                            // left, then it is certain as well
                            certanly_allocated_field_names
                                .insert(general_possible_fields.iter().next().unwrap());
                            // Ensure that we try to cleanup the general_possible_fields again
                            should_break = false;
                        }
                    }
                }
                if should_break {
                    break;
                }
            }
        }
    }

    let mut possible_fields_per_column = (0..input.ticket.field_values.len())
        .map(|index| (index, input.field_to_ranges.keys().cloned().collect()))
        .collect::<BTreeMap<_, HashSet<_>>>();

    let mut certanly_allocated_field_names = HashSet::new();

    for nearby_ticket_ in &input.nearby_tickets {
        let nearby_ticket: &Ticket = nearby_ticket_;
        update_possible_fields_per_column(
            nearby_ticket,
            input,
            &mut possible_fields_per_column,
            &mut certanly_allocated_field_names,
        );
    }

    // Ensure that we have found info for all the fields
    assert_eq!(
        certanly_allocated_field_names.len(),
        input.ticket.field_values.len()
    );

    possible_fields_per_column
        .iter()
        .map(|(index, field_names)| {
            let field_name = field_names.iter().next().unwrap();
            if field_name.starts_with("departure") {
                input.ticket.field_values[*index]
            } else {
                // Neutral value for a multiplication
                1
            }
        })
        .product()
}

fn main() -> anyhow::Result<()> {
    let lines = input_lines(INPUT)?;
    let input = Input::from(&lines);
    println!("Part 1: {}", part01(&input));
    println!("Part 2: {}", part02(&input));
    Ok(())
}
