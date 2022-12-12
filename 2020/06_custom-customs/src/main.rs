use helpers::input_lines;

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug)]
struct IndividualAnswers {
    positive: String,
}

#[derive(Debug)]
struct IndividualAnswersGroup {
    individual_answers: Vec<IndividualAnswers>,
}

impl IndividualAnswersGroup {
    fn positively_answered_count_per_question(&self) -> [usize; 26] {
        let mut positively_answered = [0; 26];
        self.individual_answers.iter().for_each(|answers| {
            answers.positive.chars().for_each(|positive_answer| {
                positively_answered[(positive_answer as usize) - ('a' as usize)] += 1;
            });
        });
        positively_answered
    }

    fn positively_answered_by_any_count(&self) -> usize {
        self.positively_answered_count_per_question()
            .iter()
            .filter(|value| **value != 0)
            .count()
    }

    fn positively_answered_by_all_count(&self) -> usize {
        self.positively_answered_count_per_question()
            .iter()
            .filter(|value| **value == self.individual_answers.len())
            .count()
    }
}

struct IndividualAnswersGroupReader<'a> {
    text_iter: std::slice::Iter<'a, String>,
}

impl Iterator for IndividualAnswersGroupReader<'_> {
    type Item = IndividualAnswersGroup;

    fn next(&mut self) -> Option<Self::Item> {
        let mut individual_answers = Vec::new();
        loop {
            let next_line = self.text_iter.next().map_or_else(|| "", String::as_str);
            if next_line.is_empty() {
                break;
            }
            individual_answers.push(IndividualAnswers {
                positive: next_line.to_string(),
            });
        }

        if individual_answers.is_empty() {
            None
        } else {
            Some(IndividualAnswersGroup { individual_answers })
        }
    }
}

fn part01(individual_answers_groups: &[IndividualAnswersGroup]) -> usize {
    individual_answers_groups
        .iter()
        .map(IndividualAnswersGroup::positively_answered_by_any_count)
        .sum()
}

fn part02(individual_answers_groups: &[IndividualAnswersGroup]) -> usize {
    individual_answers_groups
        .iter()
        .map(IndividualAnswersGroup::positively_answered_by_all_count)
        .sum()
}

fn main() -> anyhow::Result<()> {
    let lines = input_lines(INPUT)?;
    let individual_answers_groups: Vec<_> = IndividualAnswersGroupReader {
        text_iter: lines.iter(),
    }
    .collect();

    println!("Part 1: {}", part01(&individual_answers_groups));
    println!("Part 2: {}", part02(&individual_answers_groups));
    Ok(())
}
