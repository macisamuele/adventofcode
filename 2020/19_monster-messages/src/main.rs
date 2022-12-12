use helpers::input_lines;
use std::collections::HashMap;
use std::str::FromStr;

const INPUT: &str = include_str!("../input.txt");

#[derive(Clone, Debug)]
enum Rule {
    Concat(Vec<usize>),
    Literal(String),
    Multiple(Vec<Rule>),
    Single(usize),
}

impl std::fmt::Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Rule::Concat(rule_ids) => {
                for (index, rule_id) in rule_ids.iter().enumerate() {
                    if index == 0 {
                        write!(f, "{}", rule_id)?;
                    } else {
                        write!(f, " {}", rule_id)?;
                    }
                }
            }
            Rule::Literal(value) => {
                write!(f, "\"{}\"", value)?;
            }
            Rule::Multiple(rules) => {
                for (index, rule) in rules.iter().enumerate() {
                    if index == 0 {
                        write!(f, "{}", rule)?;
                    } else {
                        write!(f, " | {}", rule)?;
                    }
                }
            }
            Rule::Single(rule_id) => {
                write!(f, "{}", rule_id)?;
            }
        }
        Ok(())
    }
}

impl FromStr for Rule {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.strip_prefix('"')
            .and_then(|l| l.strip_suffix('"'))
            .map_or_else(
                || {
                    let parts: Vec<_> = s.split('|').collect();
                    if parts.len() == 1 {
                        let sub_parts: Vec<usize> = s
                            .split(' ')
                            .filter_map(|value| value.parse::<usize>().ok())
                            .collect();
                        if sub_parts.len() == 1 {
                            Ok(Self::Single(sub_parts[0]))
                        } else {
                            Ok(Self::Concat(sub_parts))
                        }
                    } else {
                        Ok(Self::Multiple(
                            parts
                                .iter()
                                .filter_map(|part| part.parse::<Rule>().ok())
                                .collect(),
                        ))
                    }
                },
                |literal| Ok(Self::Literal(literal.to_string())),
            )
    }
}

impl Rule {
    fn matched_characters(
        &self,
        rule_id_to_rule: &HashMap<usize, Rule>,
        message: &str,
    ) -> Option<usize> {
        match &self {
            Rule::Concat(rule_ids) => {
                let mut matched_characters = 0;
                for rule_id in rule_ids {
                    if let Some(m) = rule_id_to_rule[rule_id]
                        .matched_characters(rule_id_to_rule, &message[matched_characters..])
                    {
                        matched_characters += m;
                    } else {
                        return None;
                    }
                }
                Some(matched_characters)
            }
            Rule::Literal(value) => {
                let matched_characters = value
                    .as_bytes()
                    .iter()
                    .zip(message.as_bytes().iter())
                    .take_while(|(b1, b2)| b1 == b2)
                    .count();

                if matched_characters == value.len() {
                    Some(matched_characters)
                } else {
                    None
                }
            }
            Rule::Multiple(rules) => rules
                .iter()
                .filter_map(|rule| rule.matched_characters(rule_id_to_rule, message))
                .max(),
            Rule::Single(rule_id) => {
                rule_id_to_rule[rule_id].matched_characters(rule_id_to_rule, message)
            }
        }
    }

    fn rule_id_and_rule(line: &str) -> (usize, Self) {
        let (column_index, _) = line
            .as_bytes()
            .iter()
            .enumerate()
            .find(|(_, character)| *character == &b':')
            .unwrap();
        (
            line[0..column_index].parse::<usize>().unwrap(),
            line[column_index + 2..].parse::<Rule>().unwrap(),
        )
    }

    fn simplified(&self, rule_id_to_rule: &HashMap<usize, Rule>) -> Option<Rule> {
        match self {
            Rule::Concat(rule_ids) => {
                let are_all_literal = rule_ids
                    .iter()
                    .all(|rule_id| matches!(rule_id_to_rule.get(rule_id), Some(Rule::Literal(_))));
                if are_all_literal {
                    Some(Rule::Literal(
                        rule_ids
                            .iter()
                            .filter_map(|rule_id| {
                                if let Some(Rule::Literal(value)) = rule_id_to_rule.get(rule_id) {
                                    Some(value.as_str())
                                } else {
                                    None
                                }
                            })
                            .collect(),
                    ))
                } else {
                    None
                }
            }
            Rule::Literal(_) => None,
            Rule::Multiple(rules) => {
                let simplified_rules: Vec<_> = rules
                    .iter()
                    .map(|rule| rule.simplified(rule_id_to_rule))
                    .collect();
                let at_least_one_simplification = simplified_rules.iter().any(Option::is_some);
                if at_least_one_simplification {
                    Some(Rule::Multiple(
                        simplified_rules
                            .into_iter()
                            .zip(rules.iter())
                            .map(|(maybe_simplified_rule, rule)| {
                                maybe_simplified_rule
                                    .map_or_else(|| rule.clone(), |simplified_rule| simplified_rule)
                            })
                            .collect(),
                    ))
                } else {
                    None
                }
            }
            Rule::Single(rule_id) => rule_id_to_rule.get(rule_id).map(|rule: &Rule| {
                let res: Rule = rule.clone();
                res
            }),
        }
    }
}

#[derive(Debug)]
struct Input {
    messages: Vec<String>,
    rules: HashMap<usize, Rule>,
}

impl std::fmt::Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (index, rule) in &self.rules {
            writeln!(f, "{}: {}", index, rule)?;
        }
        writeln!(f)?;
        for message in &self.messages {
            writeln!(f, "{}", message)?;
        }
        Ok(())
    }
}

impl Clone for Input {
    fn clone(&self) -> Self {
        Self {
            messages: self.messages.clone(),
            rules: self
                .rules
                .iter()
                .map(|(index, rule)| (*index, rule.clone()))
                .collect(),
        }
    }
}

impl From<&Vec<String>> for Input {
    fn from(lines: &Vec<String>) -> Self {
        let mut lines_iter = lines.iter();
        let mut rule_id_to_rule: HashMap<_, _> = lines_iter
            .by_ref()
            .take_while(|line| !line.is_empty())
            .map(|line| Rule::rule_id_and_rule(line))
            .collect();

        let messages = lines_iter.by_ref().cloned().collect();

        // Try to simplify the rules
        loop {
            let rules_to_update: HashMap<usize, Rule> = rule_id_to_rule
                .iter()
                .filter_map(|(index, rule)| {
                    rule.simplified(&rule_id_to_rule)
                        .map(|simplified_rule| (*index, simplified_rule))
                })
                .collect();
            if rules_to_update.is_empty() {
                break;
            }
            for (index, simplified_rule) in rules_to_update {
                rule_id_to_rule.insert(index, simplified_rule);
            }
        }

        Self {
            rules: rule_id_to_rule,
            messages,
        }
    }
}

impl Input {
    fn is_valid(&self, index: usize, message: &str) -> bool {
        self.rules[&index]
            .matched_characters(&self.rules, message)
            .map_or(false, |matched_characters| {
                matched_characters == message.len()
            })
    }
}

fn part01(input: &Input) -> usize {
    input
        .messages
        .iter()
        .filter(|message| input.is_valid(0, message))
        .count()
}

fn part02(input: &Input) -> usize {
    let mut cloned_input: Input = input.clone();

    cloned_input.rules.extend(
        ["8: 42 | 42 8", "11: 42 31 | 42 11 31"]
            .iter()
            .map(|rule_str| Rule::rule_id_and_rule(rule_str)),
    );

    cloned_input
        .messages
        .iter()
        .filter(|message| cloned_input.is_valid(0, message))
        .count()
}

fn main() -> anyhow::Result<()> {
    let lines = input_lines(INPUT)?;
    let input = Input::from(&lines);

    println!("Part 1: {}", part01(&input));
    println!("Part 2: {}", part02(&input));

    Ok(())
}
