use std::collections::HashSet;

#[derive(Clone, Debug)]
enum Rule {
    Acc(isize),
    Jmp(isize),
    NoOp(isize),
}

impl Rule {
    fn flip(&mut self) {
        match self {
            Rule::Acc(_) => {}
            Rule::Jmp(value) => *self = Rule::NoOp(*value),
            Rule::NoOp(value) => *self = Rule::Jmp(*value),
        }
    }
}

#[derive(Debug)]
struct MachineState<'a> {
    rules: &'a [Rule],
    value: isize,
    index: usize,
}

impl MachineState<'_> {
    fn is_completed(&self) -> bool {
        self.index == self.rules.len()
    }

    fn do_move(&mut self) {
        match self.rules[self.index] {
            Rule::Acc(acc_value) => {
                self.value += acc_value;
                self.index += 1;
            }
            Rule::Jmp(value) => {
                if value > 0 {
                    self.index += value as usize;
                } else {
                    self.index -= -value as usize;
                }
            }
            Rule::NoOp(_) => {
                self.index += 1;
            }
        }
    }

    fn do_all_moves(rules: &[Rule]) -> (bool, isize) {
        let mut state = MachineState {
            rules,
            value: 0,
            index: 0,
        };
        let mut visited_indexes = HashSet::new();
        while !state.is_completed() && !visited_indexes.contains(&state.index) {
            visited_indexes.insert(state.index);
            state.do_move();
        }
        (state.is_completed(), state.value)
    }
}
use helpers::input_lines;

const INPUT: &str = include_str!("../input.txt");

fn part01(rules: &[Rule]) -> isize {
    MachineState::do_all_moves(rules).1
}

fn part02(rules: &[Rule]) -> isize {
    let mut cloned_rules: Vec<_> = rules.to_vec();
    for index in 0..rules.len() {
        cloned_rules[index].flip();
        let (is_completed, accumulator_value) = MachineState::do_all_moves(&cloned_rules);
        if is_completed {
            return accumulator_value;
        }
        cloned_rules[index].flip();
    }
    0
}

fn main() -> anyhow::Result<()> {
    let lines = input_lines(INPUT)?;
    let rules: Vec<Rule> = lines
        .iter()
        .map(|line| {
            if let Some(value) = line.strip_prefix("acc ") {
                Rule::Acc(value.parse().unwrap())
            } else if let Some(value) = line.strip_prefix("jmp ") {
                Rule::Jmp(value.parse().unwrap())
            } else if let Some(value) = line.strip_prefix("nop ") {
                Rule::NoOp(value.parse().unwrap())
            } else {
                unreachable!()
            }
        })
        .collect();

    println!("Part 1: {}", part01(&rules));
    println!("Part 2: {}", part02(&rules));
    Ok(())
}
