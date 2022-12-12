use helpers::input_lines;
use std::collections::HashMap;

const INPUT: &str = include_str!("../input.txt");

fn are_coprime(mut value1: i64, mut value2: i64) -> bool {
    // number1 and number2 are coprime if GCD is 1
    // We evaluate GCD via Euclid's Algorithm
    while value1 > 0 && value2 > 0 {
        if value1 > value2 {
            value1 %= value2;
        } else {
            value2 %= value1;
        }
    }
    (value1 + value2) == 1
}

fn paiwise_coprime(values: &[i64]) -> bool {
    values.iter().enumerate().all(|(index, value1)| {
        values[index + 1..]
            .iter()
            .all(|value2| are_coprime(*value1, *value2))
    })
}

fn part01(arrival_time: usize, line_numbers: &[Option<usize>]) -> usize {
    let earliest_departure_to_line_number: HashMap<usize, &usize> = line_numbers
        .iter()
        .filter_map(|maybe_line_number| {
            maybe_line_number.as_ref().map(|line_number| {
                match arrival_time.checked_rem(*line_number) {
                    Some(0) => (arrival_time, line_number),
                    Some(value) => (arrival_time - value + line_number, line_number),
                    None => unreachable!("Not possible value"),
                }
            })
        })
        .collect();
    earliest_departure_to_line_number
        .keys()
        .min()
        .map_or(0, |earliest_departure| {
            let line_number = *earliest_departure_to_line_number[earliest_departure];
            (earliest_departure - arrival_time) * line_number
        })
}

fn positive_module(mut a: i64, mut b: i64) -> i64 {
    // Evaluate (a % b) and ensures that the result is positive
    if b < 0 {
        a *= -1;
        b *= -1;
    }
    if a < 0 {
        a % b + b
    } else {
        a % b
    }
}

fn module_inverse(a: i64, b: i64) -> i64 {
    // Evaluate a^(-1) ≡ 1 (mod b)
    (1..=b).find(|value| (value * a) % b == 1).unwrap_or(1)
}

fn part02(line_numbers: &[Option<usize>]) -> i64 {
    // Given "7,13,x,x,59,x,31,19" input string
    // line_numbers will look like
    // [Some(7), Some(13), None, None, Some(59), None, Some(31), Some(19)]
    //
    // We will need to find t such that
    //       7 * K0 = t
    //      13 * K1 = t + 1
    //      59 * K4 = t + 4
    //      31 * K6 = t + 6
    //      19 * K7 = t + 7
    // Where: Ki are positive integers
    //
    // If we generalise we will need to solve
    //      L0 * K0 = t + O0
    //      L1 * K1 = t + O1
    //      L2 * K2 = t + O2
    //      ...
    //      Li * Ki = t + Oi
    // Where
    //      Li = <line-number i>
    //      Ki = are unknown but by construct are positive integers
    //      Oi = are offsets to the value
    //
    // Solving the equations is equivalent to
    //      t ≡ -O0 (mod L0)
    //      t ≡ -O1 (mod L1)
    //      t ≡ -O2 (mod L2)
    //      ...
    //      t ≡ -Oi (mod Li)
    // This formulation ressamble the one of the Chinese Reminder Theorem.
    // https://en.wikipedia.org/wiki/Chinese_remainder_theorem
    // It works only if Li are pairwise coprime
    //
    // The implementation works on the following bases
    //  1. product = L0 * L1 * L2 * ... * Li
    //  2. partial_i = product / Li
    //  3. inverse_i, such that (partial_i * inverse_i) ≡ 1 (mod Li)
    //  4. result = (
    //         inverse_0 * partial_0 * (-O0) +
    //         inverse_1 * partial_1 * (-O1) +
    //         inverse_2 * partial_2 * (-O2) +
    //         ...
    //         inverse_i * partial_i * (-Oi)
    //     ) % product
    let li_to_oi: HashMap<_, _> = line_numbers
        .iter()
        .enumerate()
        .filter_map(|(index, maybe_value)| maybe_value.map(|value| (value as i64, index as i64)))
        .collect();

    assert!(paiwise_coprime(
        &li_to_oi.keys().copied().collect::<Vec<_>>()
    ));

    let product = li_to_oi.keys().product();
    let partials: HashMap<_, _> = li_to_oi.keys().map(|l_i| (*l_i, product / l_i)).collect();
    let inverses: HashMap<_, _> = partials
        .iter()
        .map(|(l_i, partial_i)| (*l_i, module_inverse(*partial_i, *l_i)))
        .collect();

    li_to_oi.iter().fold(0, |result, (li, oi)| {
        positive_module(result + inverses[li] * partials[li] * (-oi), product)
    })
}

fn main() -> anyhow::Result<()> {
    let lines = input_lines(INPUT)?;
    let arrival_time: usize = lines[0].parse().unwrap();
    let line_numbers: Vec<Option<usize>> =
        lines[1].split(',').map(|line| line.parse().ok()).collect();

    println!("Part 1: {}", part01(arrival_time, &line_numbers));
    println!("Part 2: {}", part02(&line_numbers));

    Ok(())
}
