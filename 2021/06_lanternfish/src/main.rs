use helpers::input_lines;
use std::str::FromStr;
const INPUT: &str = include_str!("../input.txt");

fn part01(mut fishes: LanternFishes) -> usize {
    for _ in 0..80 {
        fishes.next_generation();
    }
    fishes.fish_count()
}

fn part02(mut fishes: LanternFishes) -> usize {
    for _ in 0..256 {
        fishes.next_generation();
    }
    fishes.fish_count()
}

#[derive(Clone, Copy, Debug)]
struct LanternFishes {
    time_before_reproduction: [usize; 9],
}

impl FromStr for LanternFishes {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<usize> = s.split(',').map(str::parse).collect::<Result<_, _>>()?;
        let time_before_reproduction =
            parts
                .iter()
                .fold([0; 9], |mut time_before_reproduction, value| {
                    time_before_reproduction[*value] += 1;
                    time_before_reproduction
                });
        Ok(Self {
            time_before_reproduction,
        })
    }
}

impl LanternFishes {
    fn next_generation(&mut self) {
        let lantern_fishes_in_reproduction = self.time_before_reproduction[0];

        for index in 0..8 {
            self.time_before_reproduction[index] = self.time_before_reproduction[index + 1];
        }

        self.time_before_reproduction[6] += lantern_fishes_in_reproduction;
        self.time_before_reproduction[8] = lantern_fishes_in_reproduction;
    }
    fn fish_count(&self) -> usize {
        self.time_before_reproduction.iter().sum()
    }
}
fn main() -> anyhow::Result<()> {
    let lines = input_lines(INPUT)?;
    anyhow::ensure!(lines.len() == 1);

    let fishes: LanternFishes = lines[0].parse()?;

    println!("Part 1: {}", part01(fishes));
    println!("Part 2: {}", part02(fishes));
    Ok(())
}
