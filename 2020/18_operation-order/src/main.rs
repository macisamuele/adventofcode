use helpers::input_lines;
use std::collections::LinkedList;
use std::iter::Peekable;
use std::str::FromStr;

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug)]
enum AlgebricOperation {
    Add,
    Mul,
}

#[derive(Clone, Debug, PartialEq)]
enum Token {
    Literal(usize),
    Add,
    Mul,
    SubOperation(Operation),
}

#[derive(Clone, Debug, PartialEq)]
struct Operation {
    tokens: Vec<Token>,
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(&mut value.chars().peekable()))
    }
}

impl<I: Iterator<Item = char>> From<&mut Peekable<I>> for Operation {
    fn from(value: &mut Peekable<I>) -> Self {
        struct ExpressionInteration<'a, T: Iterator<Item = char>> {
            input_characters: &'a mut Peekable<T>,
        }

        impl<T: Iterator<Item = char>> Iterator for ExpressionInteration<'_, T> {
            type Item = Token;

            fn next(&mut self) -> Option<Self::Item> {
                while self.input_characters.peek() == Some(&' ') {
                    self.input_characters.next();
                }

                let mut literal_str = String::new();
                while let Some(next_value) = self.input_characters.peek() {
                    if ('0'..='9').contains(next_value) {
                        literal_str.push(*next_value);
                        self.input_characters.next();
                    } else {
                        break;
                    }
                }

                Some(if literal_str.is_empty() {
                    let next_value = self.input_characters.next()?;

                    match next_value {
                        '+' => Token::Add,
                        '*' => Token::Mul,
                        '(' => Token::SubOperation(Operation::from({
                            let tmp: &mut Peekable<T> = self.input_characters;
                            tmp
                        })),
                        ')' => return None,
                        _ => unreachable!(),
                    }
                } else {
                    Token::Literal(literal_str.parse().ok()?)
                })
            }
        }
        Self {
            tokens: ExpressionInteration {
                input_characters: value,
            }
            .collect(),
        }
    }
}

fn evaluate_operation_part01(operation_str: &str) -> usize {
    fn eval_rec(iter: &mut dyn Iterator<Item = &Token>) -> usize {
        let mut current_result = 0;
        let mut algebric_operation = AlgebricOperation::Add;

        for token in iter {
            match token {
                Token::Literal(n) => match algebric_operation {
                    AlgebricOperation::Add => {
                        current_result += n;
                    }
                    AlgebricOperation::Mul => {
                        current_result *= n;
                    }
                },
                Token::Add => {
                    algebric_operation = AlgebricOperation::Add;
                }
                Token::Mul => {
                    algebric_operation = AlgebricOperation::Mul;
                }
                Token::SubOperation(sub_operation) => {
                    let n = eval_rec(&mut sub_operation.tokens.iter());
                    match algebric_operation {
                        AlgebricOperation::Add => {
                            current_result += n;
                        }
                        AlgebricOperation::Mul => {
                            current_result *= n;
                        }
                    }
                }
            }
        }
        current_result
    }

    let operation = operation_str.parse::<Operation>().unwrap();
    eval_rec(&mut operation.tokens.iter())
}

fn part01(lines: &[String]) -> usize {
    lines
        .iter()
        .map(|line| evaluate_operation_part01(line))
        .sum()
}

fn evaluate_operation_part02(operation_str: &str) -> usize {
    fn eval_rec(operation: &mut Operation) -> usize {
        let mut stack = LinkedList::<Token>::new();

        for token in &mut operation.tokens {
            // Resolve sub operations
            if let Token::SubOperation(sub_operation) = token {
                *token = Token::Literal(eval_rec(sub_operation));
            }
        }

        for token in &operation.tokens {
            // Resolve additions (have precedence)
            match token {
                Token::Literal(n) => {
                    if let Some(last_token) = stack.back() {
                        if last_token == &Token::Add {
                            stack.pop_back(); // Operator
                            if let Some(Token::Literal(previous_n)) = stack.pop_back() {
                                stack.push_back(Token::Literal(previous_n + n));
                            } else {
                                unreachable!();
                            }
                        } else {
                            stack.push_back(token.clone());
                        }
                    } else {
                        stack.push_back(token.clone());
                    }
                }
                Token::Add | Token::Mul => stack.push_back(token.clone()),
                Token::SubOperation(_) => {
                    unreachable!("We should not have {:?} in here", token);
                }
            };
        }

        let mut current_result = 1;
        for token in stack {
            // Resolve multiplications
            match token {
                Token::Literal(n) => {
                    current_result *= n;
                }
                Token::Mul => {}
                _ => {
                    unreachable!("We should never have {:?} in here", token);
                }
            }
        }

        current_result
    }

    eval_rec(&mut operation_str.parse::<Operation>().unwrap())
}

fn part02(lines: &[String]) -> usize {
    lines
        .iter()
        .map(|line| evaluate_operation_part02(line))
        .sum()
}

fn main() -> anyhow::Result<()> {
    let lines = input_lines(INPUT)?;
    println!("Part 1: {}", part01(&lines));
    println!("Part 2: {}", part02(&lines));

    Ok(())
}
