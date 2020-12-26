use helpers::input_lines;
use std::collections::{HashSet, VecDeque};

const INPUT: &str = include_str!("../input.txt");

#[derive(Clone, Debug, PartialEq)]
enum Player {
    Player1,
    Player2,
}

trait Game {
    fn deck(&self, player: Player) -> &VecDeque<usize>;

    fn play(&mut self) -> (Player, usize);

    fn score(&self, player: Player) -> usize {
        self.deck(player)
            .iter()
            .rev()
            .enumerate()
            .map(|(index, value)| value * (index + 1))
            .sum()
    }

    fn winner(&self) -> Option<Player> {
        if self.deck(Player::Player1).is_empty() {
            Some(Player::Player2)
        } else if self.deck(Player::Player2).is_empty() {
            Some(Player::Player1)
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct CrabCombat {
    player1: VecDeque<usize>,
    player2: VecDeque<usize>,
}

impl From<&[String]> for CrabCombat {
    fn from(lines: &[String]) -> Self {
        let mut lines_iter = lines.iter();

        let player1 = lines_iter
            .by_ref()
            .take_while(|line| !line.is_empty())
            .skip(1)
            .filter_map(|line| line.parse::<usize>().ok())
            .collect();

        let player2 = lines_iter
            .by_ref()
            .take_while(|line| !line.is_empty())
            .skip(1)
            .filter_map(|line| line.parse::<usize>().ok())
            .collect();

        Self::new(player1, player2)
    }
}

impl Game for CrabCombat {
    fn deck(&self, player: Player) -> &VecDeque<usize> {
        match player {
            Player::Player1 => &self.player1,
            Player::Player2 => &self.player2,
        }
    }

    fn play(&mut self) -> (Player, usize) {
        while self.winner().is_none() {
            let card1 = self.player1.pop_front().unwrap();
            let card2 = self.player2.pop_front().unwrap();

            if card1 > card2 {
                self.player1.push_back(card1);
                self.player1.push_back(card2);
            } else {
                self.player2.push_back(card2);
                self.player2.push_back(card1);
            }
        }

        let winner = self.winner().unwrap();
        (winner.clone(), self.score(winner))
    }
}

impl CrabCombat {
    fn new(deck1: VecDeque<usize>, deck2: VecDeque<usize>) -> Self {
        Self {
            player1: deck1,
            player2: deck2,
        }
    }
}

#[derive(Debug)]
struct RecursiveCombat {
    player1: VecDeque<usize>,
    player2: VecDeque<usize>,
    rounds_decks: HashSet<(VecDeque<usize>, VecDeque<usize>)>,
}

impl From<&[String]> for RecursiveCombat {
    fn from(lines: &[String]) -> Self {
        let mut lines_iter = lines.iter();

        let player1 = lines_iter
            .by_ref()
            .take_while(|line| !line.is_empty())
            .skip(1)
            .filter_map(|line| line.parse::<usize>().ok())
            .collect();

        let player2 = lines_iter
            .by_ref()
            .take_while(|line| !line.is_empty())
            .skip(1)
            .filter_map(|line| line.parse::<usize>().ok())
            .collect();

        Self::new_from_deque(player1, player2)
    }
}

impl Game for RecursiveCombat {
    fn deck(&self, player: Player) -> &VecDeque<usize> {
        match player {
            Player::Player1 => &self.player1,
            Player::Player2 => &self.player2,
        }
    }

    fn play(&mut self) -> (Player, usize) {
        while self.winner().is_none() {
            if self.rounds_decks.insert((
                self.player1.iter().copied().collect(),
                self.player2.iter().copied().collect(),
            )) {
                let card1 = self.player1.pop_front().unwrap();
                let card2 = self.player2.pop_front().unwrap();

                let round_winner = if card1 <= self.player1.len() && card2 <= self.player2.len() {
                    let (sub_game_winner, _) = Self::new(
                        self.player1.iter().take(card1).copied(),
                        self.player2.iter().take(card2).copied(),
                    )
                    .play();
                    sub_game_winner
                } else if card1 > card2 {
                    Player::Player1
                } else {
                    Player::Player2
                };

                match round_winner {
                    Player::Player1 => {
                        self.player1.push_back(card1);
                        self.player1.push_back(card2);
                    }
                    Player::Player2 => {
                        self.player2.push_back(card2);
                        self.player2.push_back(card1);
                    }
                }
            } else {
                return (Player::Player1, self.score(Player::Player1));
            }
        }

        let winner = self.winner().unwrap();
        (winner.clone(), self.score(winner))
    }
}

impl RecursiveCombat {
    fn new<I1: Iterator<Item = usize>, I2: Iterator<Item = usize>>(deck1: I1, deck2: I2) -> Self {
        Self::new_from_deque(deck1.collect(), deck2.collect())
    }
    fn new_from_deque(deck1: VecDeque<usize>, deck2: VecDeque<usize>) -> Self {
        Self {
            player1: deck1,
            player2: deck2,
            rounds_decks: HashSet::new(),
        }
    }
}

fn part01(lines: &[String]) -> usize {
    let (_, score) = CrabCombat::from(lines).play();
    score
}

fn part02(lines: &[String]) -> usize {
    let (_, score) = RecursiveCombat::from(lines).play();
    score
}

fn main() -> anyhow::Result<()> {
    let lines = input_lines(INPUT)?;
    println!("Part 1: {}", part01(&lines));
    println!("Part 2: {}", part02(&lines));

    Ok(())
}
