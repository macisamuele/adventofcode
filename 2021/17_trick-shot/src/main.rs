use helpers::input_lines;
use scan_fmt::scan_fmt;
use std::ops::RangeInclusive;
use std::str::FromStr;

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug)]
struct Area {
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
}

impl FromStr for Area {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (min_x, max_x, min_y, max_y) = scan_fmt!(
            s,
            "target area: x={}..{}, y={}..{}",
            isize,
            isize,
            isize,
            isize
        )?;
        anyhow::ensure!(min_x <= max_x);
        anyhow::ensure!(min_y <= max_y);

        Ok(Self {
            min_x,
            max_x,
            min_y,
            max_y,
        })
    }
}

impl Area {
    fn possible_horizontal_initial_speed(&self) -> RangeInclusive<isize> {
        match (self.min_x, self.max_x) {
            (min_x, max_x) if min_x.is_negative() && max_x.is_negative() => min_x..=0,
            (min_x, max_x) if min_x.is_positive() && max_x.is_positive() => 0..=max_x,
            (min_x, max_x) => min_x..=max_x,
        }
    }

    fn possible_vertical_initial_speed(&self) -> RangeInclusive<isize> {
        // This is an hack as I haven't thought about how to
        // be smart and restrict the range
        const MAX_VERTICAL_SPEED: isize = 1000;

        match (self.min_y, self.max_y) {
            (min_y, max_y) if min_y.is_negative() && max_y.is_negative() => {
                min_y..=MAX_VERTICAL_SPEED
            }
            (min_y, max_y) if min_y.is_positive() && max_y.is_positive() => {
                0..=max_y.max(MAX_VERTICAL_SPEED)
            }
            (min_y, max_y) => min_y..=max_y,
        }
    }

    fn max_vertical_positition_toward_target_area(
        &self,
        mut speed_x: isize,
        mut speed_y: isize,
    ) -> Option<isize> {
        let (mut x, mut y) = (0, 0);
        let mut max_y = isize::MIN;

        loop {
            x += speed_x;
            y += speed_y;

            max_y = max_y.max(y);

            if speed_x.is_positive() {
                speed_x -= 1;
            } else if speed_x.is_negative() {
                speed_x += 1;
            }
            speed_y -= 1;

            if x >= self.min_x && x <= self.max_x && y >= self.min_y && y <= self.max_y {
                return Some(max_y);
            }

            if x > self.max_x || y < self.min_y {
                return None;
            }
        }
    }

    fn can_reach_target_area(&self, speed_x: isize, speed_y: isize) -> bool {
        self.max_vertical_positition_toward_target_area(speed_x, speed_y)
            .is_some()
    }
}

fn part01(area: &Area) -> isize {
    area.possible_horizontal_initial_speed()
        .flat_map(|speed_x| {
            area.possible_vertical_initial_speed()
                .filter_map(move |speed_y| {
                    area.max_vertical_positition_toward_target_area(speed_x, speed_y)
                })
        })
        .max()
        .expect("At least one combination should be able to reach the destination")
}

fn part02(area: &Area) -> usize {
    area.possible_horizontal_initial_speed()
        .flat_map(|speed_x| {
            area.possible_vertical_initial_speed()
                .filter(move |speed_y| area.can_reach_target_area(speed_x, *speed_y))
        })
        .count()
}

fn main() -> anyhow::Result<()> {
    let lines = input_lines(INPUT)?;
    anyhow::ensure!(lines.len() == 1);
    let area: Area = lines[0].parse()?;

    println!("Part 1: {}", part01(&area));
    println!("Part 2: {}", part02(&area));

    Ok(())
}
