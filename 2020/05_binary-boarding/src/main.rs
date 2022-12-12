use helpers::input_lines;

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug)]
struct BoardingPass {
    row: u8,
    column: u8,
}

fn to_number(code: &str, character_zero: char, character_one: char) -> u8 {
    code.chars()
        .map(|c| {
            if c == character_zero {
                0
            } else if c == character_one {
                1
            } else {
                panic!(
                    "Only {} and {} are accepted, received {}",
                    character_zero, character_one, c
                );
            }
        })
        .fold(0, |acc, x| (acc << 1) + x)
}

impl BoardingPass {
    fn new(code: &str) -> Self {
        assert_eq!(code.len(), 10);
        Self {
            row: to_number(code.get(..7).unwrap(), 'F', 'B'),
            column: to_number(code.get(7..).unwrap(), 'L', 'R'),
        }
    }

    fn seat_id(&self) -> usize {
        (self.row as usize) * 8 + (self.column as usize)
    }
}

fn part01(boarding_passes: &[BoardingPass]) -> usize {
    boarding_passes
        .iter()
        .map(BoardingPass::seat_id)
        .max()
        .unwrap_or(0)
}

fn part02(boarding_passes: &[BoardingPass]) -> usize {
    let mut sorted_sids: Vec<_> = boarding_passes.iter().map(BoardingPass::seat_id).collect();
    sorted_sids.sort_unstable();

    for (index, seat_id) in sorted_sids.iter().enumerate() {
        if let Some(next_seat_id) = sorted_sids.get(index + 1) {
            if *seat_id + 2 == *next_seat_id {
                return *seat_id + 1;
            }
        }
    }
    0
}

fn main() -> anyhow::Result<()> {
    let lines = input_lines(INPUT)?;
    let boarding_passes = lines
        .iter()
        .map(|code| BoardingPass::new(code))
        .collect::<Vec<_>>();

    println!("Part 1: {}", part01(&boarding_passes));
    println!("Part 2: {}", part02(&boarding_passes));
    Ok(())
}
