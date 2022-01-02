use helpers::input_lines;
use scan_fmt::scan_fmt;
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};

const INPUT: &str = include_str!("../input.txt");
const TRACK_LENGTH: u8 = 10;

#[inline]
fn cast_to_u8(value: u16) -> u8 {
    value
        .try_into()
        .expect("Expected to be able to convert value")
}

trait Dice: std::fmt::Debug + Default {
    fn roll(&mut self) -> u8;
    fn roll_count(&self) -> u16;

    fn roll_3_times(&mut self) -> u16 {
        u16::from(self.roll()) + u16::from(self.roll()) + u16::from(self.roll())
    }

    fn extraction_3_rolls_statistics() -> HashMap<u16, usize>;
}

#[derive(Debug, Default)]
struct DeterministicDice<const N: u8> {
    value: u8,
    roll_count: u16,
}

impl<const N: u8> Dice for DeterministicDice<N> {
    fn roll(&mut self) -> u8 {
        self.roll_count += 1;
        let result = self.value + 1;
        self.value = (self.value + 1) % N;
        result
    }

    fn roll_count(&self) -> u16 {
        self.roll_count
    }

    fn extraction_3_rolls_statistics() -> HashMap<u16, usize> {
        let mut dice = Self::default();
        let mut extractions_histogram: HashMap<(u8, u8, u8), usize> = HashMap::new();

        loop {
            let key = (dice.roll(), dice.roll(), dice.roll());
            if extractions_histogram.contains_key(&key) {
                break;
            }

            *extractions_histogram.entry(key).or_default() += 1;
        }

        extractions_histogram
            .into_iter()
            .map(|((d1, d2, d3), count)| (u16::from(d1) + u16::from(d2) + u16::from(d3), count))
            .fold(HashMap::new(), |mut map, (sum, count)| {
                *map.entry(sum).or_default() += count;
                map
            })
    }
}

#[derive(Debug, Default)]
struct RandomDice<const N: u8> {
    roll_count: u16,
}

impl<const N: u8> Dice for RandomDice<N> {
    fn roll(&mut self) -> u8 {
        self.roll_count += 1;
        rand::random::<u8>() % N + 1
    }

    fn roll_count(&self) -> u16 {
        self.roll_count
    }

    fn extraction_3_rolls_statistics() -> HashMap<u16, usize> {
        (1..=N)
            .flat_map(|d1| {
                (1..=N).flat_map(move |d2| {
                    (1..=N).map(move |d3| u16::from(d1) + u16::from(d2) + u16::from(d3))
                })
            })
            .fold(HashMap::new(), |mut map, value| {
                *map.entry(value).or_default() += 1;
                map
            })
    }
}

#[derive(Debug)]
enum Player {
    Player1,
    Player2,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
// Game is compressed as much as possible such that it would not cause
// memory problems as we will store many
struct Game {
    is_player1_round: bool,
    // Positions are between 1 and 10, hence 8 bits are sufficient
    player1_position: u8,
    player2_position: u8,
    // Scores are up to 1000 for part 1 and 21 for part2,
    // hence 11 bits are sufficient with some margin
    // as there are no 11 bits integers we're using the closer
    // one (16 bits)
    player1_score: u16,
    player2_score: u16,
}

impl TryFrom<Vec<String>> for Game {
    type Error = anyhow::Error;

    fn try_from(lines: Vec<String>) -> Result<Self, Self::Error> {
        anyhow::ensure!(lines.len() == 2);
        let player1_position = scan_fmt!(&lines[0], "Player 1 starting position: {}", u8)?;
        let player2_position = scan_fmt!(&lines[1], "Player 2 starting position: {}", u8)?;
        Ok(Self::new(player1_position, player2_position))
    }
}

impl Game {
    fn new(player1_position: u8, player2_position: u8) -> Self {
        Self {
            player1_position,
            player2_position,
            player1_score: 0,
            player2_score: 0,
            is_player1_round: true,
        }
    }

    #[allow(clippy::trivially_copy_pass_by_ref)]
    fn winner(&self, game_victory_score: u16) -> Option<Player> {
        if self.player1_score >= game_victory_score {
            Some(Player::Player1)
        } else if self.player2_score >= game_victory_score {
            Some(Player::Player2)
        } else {
            None
        }
    }

    fn apply_move(&mut self, amount: u16) {
        let (position, score) = if self.is_player1_round {
            (&mut self.player1_position, &mut self.player1_score)
        } else {
            (&mut self.player2_position, &mut self.player2_score)
        };
        *position = cast_to_u8((u16::from(*position) + amount - 1) % u16::from(TRACK_LENGTH) + 1);
        *score += u16::from(*position);

        self.is_player1_round = !self.is_player1_round;
    }

    fn play<D: Dice>(&mut self, game_victory_score: u16) -> usize {
        let mut dice = D::default();

        while self.winner(game_victory_score).is_none() {
            self.apply_move(dice.roll_3_times());
        }

        usize::from(self.player1_score.min(self.player2_score)) * usize::from(dice.roll_count())
    }

    fn play_all_possible_games<D: Dice>(&mut self, game_victory_score: u16) -> (usize, usize) {
        fn register_future_games(
            future_game_move_amout_and_count: &mut Vec<(Game, u16, usize)>,
            extraction_3_rolls_statistics: &HashMap<u16, usize>,
            game: Game,
            counter: usize,
        ) {
            future_game_move_amout_and_count.extend(
                extraction_3_rolls_statistics
                    .iter()
                    .map(|(move_amout, count)| (game, *move_amout, counter * *count)),
            );
        }

        let mut player1_wins = 0;
        let mut player2_wins = 0;

        // Storing it in a variable such that we don't re-evaluate the statistics all the time
        let extraction_3_rolls_statistics = D::extraction_3_rolls_statistics();

        // It will contains all the games that we should play
        // Each entry is a game and the number of times it occurred
        // NOTE: the game itself could be present multiple times in the list
        let mut future_game_move_amout_and_count: Vec<(Self, u16, usize)> = vec![];
        register_future_games(
            &mut future_game_move_amout_and_count,
            &extraction_3_rolls_statistics,
            *self,
            1,
        );

        while !future_game_move_amout_and_count.is_empty() {
            let (mut current_game, move_amout, counter) = future_game_move_amout_and_count
                .pop()
                .expect("At least a game is present in future_games");

            current_game.apply_move(move_amout);

            match current_game.winner(game_victory_score) {
                Some(Player::Player1) => player1_wins += counter,
                Some(Player::Player2) => player2_wins += counter,
                None => {
                    register_future_games(
                        &mut future_game_move_amout_and_count,
                        &extraction_3_rolls_statistics,
                        current_game,
                        counter,
                    );
                }
            }
        }
        (player1_wins, player2_wins)
    }
}

fn part01(mut game: Game) -> usize {
    game.play::<DeterministicDice<100>>(1000)
}

fn part02(mut game: Game) -> usize {
    let (player1_wins, player2_wins) = game.play_all_possible_games::<RandomDice<3>>(21);
    player1_wins.max(player2_wins)
}

fn main() -> anyhow::Result<()> {
    let game = Game::try_from(input_lines(INPUT)?)?;

    println!("Part 1: {}", part01(game));
    println!("Part 2: {}", part02(game));

    Ok(())
}
