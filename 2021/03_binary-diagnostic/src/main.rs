use helpers::input_lines;

const INPUT: &str = include_str!("../input.txt");

const NUMBER_OF_BITS: usize = 12;
#[derive(Debug, Clone, Copy, Default)]
struct Rate {
    bit_sum: [usize; NUMBER_OF_BITS],
    number_of_elements: usize,
}

impl Rate {
    fn add_value(&mut self, value: usize) {
        for bit_power in 0..=(NUMBER_OF_BITS - 1) {
            if value & 1 << bit_power != 0 {
                self.bit_sum[bit_power] += 1
            }
        }
        self.number_of_elements += 1;
    }

    fn gamma_rate(&self) -> usize {
        let mut result = 0;
        let min_elements_to_be_one = self.number_of_elements >> 1;

        for bit_power in 0..=(NUMBER_OF_BITS - 1) {
            if self.bit_sum[bit_power] >= min_elements_to_be_one {
                result |= 1 << bit_power;
            }
        }
        result
    }

    fn epsilon_rate(&self) -> usize {
        (1 << NUMBER_OF_BITS) - 1 - self.gamma_rate()
    }
}

fn part01(inputs: &[usize]) -> usize {
    let rate = inputs.iter().fold(Rate::default(), |mut rate, value| {
        rate.add_value(*value);
        rate
    });
    rate.gamma_rate() * rate.epsilon_rate()
}

fn calculate_o2_value(inputs: &[usize], bit_index: usize) -> usize {
    if inputs.len() == 1 {
        inputs[0]
    } else {
        let (inputs_with_0_bit, inputs_with_1_bit) = inputs.iter().fold(
            (vec![], vec![]),
            |(mut inputs_with_0_bit, mut inputs_with_1_bit), value| {
                if (*value) & 1 << (NUMBER_OF_BITS - 1 - bit_index) == 0 {
                    inputs_with_0_bit.push(*value);
                } else {
                    inputs_with_1_bit.push(*value);
                }
                (inputs_with_0_bit, inputs_with_1_bit)
            },
        );

        if inputs_with_0_bit.len() > inputs_with_1_bit.len() {
            calculate_o2_value(&inputs_with_0_bit, bit_index + 1)
        } else {
            calculate_o2_value(&inputs_with_1_bit, bit_index + 1)
        }
    }
}

fn calculate_co2_value(inputs: &[usize], bit_index: usize) -> usize {
    if inputs.len() == 1 {
        inputs[0]
    } else {
        let (inputs_with_0_bit, inputs_with_1_bit) = inputs.iter().fold(
            (vec![], vec![]),
            |(mut inputs_with_0_bit, mut inputs_with_1_bit), value| {
                if (*value) & 1 << (NUMBER_OF_BITS - 1 - bit_index) == 0 {
                    inputs_with_0_bit.push(*value);
                } else {
                    inputs_with_1_bit.push(*value);
                }
                (inputs_with_0_bit, inputs_with_1_bit)
            },
        );

        if inputs_with_0_bit.len() > inputs_with_1_bit.len() {
            calculate_co2_value(&inputs_with_1_bit, bit_index + 1)
        } else {
            calculate_co2_value(&inputs_with_0_bit, bit_index + 1)
        }
    }
}

fn part02(inputs: &[usize]) -> usize {
    calculate_o2_value(inputs, 0) * calculate_co2_value(inputs, 0)
}

fn main() -> anyhow::Result<()> {
    let inputs = input_lines(INPUT)?
        .iter()
        .map(|line| usize::from_str_radix(line, 2))
        .collect::<Result<Vec<_>, _>>()?;

    println!("Part 1: {}", part01(&inputs));
    println!("Part 2: {}", part02(&inputs));
    Ok(())
}
