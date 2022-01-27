use std::str::FromStr;

const SIZE: usize = 12;

struct BitVec {
    vec: [bool; SIZE],
}

impl FromStr for BitVec {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vec: Vec<bool> = s
            .trim()
            .chars()
            .map(|x| match x {
                '0' => false,
                '1' => true,
                _ => unreachable!("should not reach here"),
            })
            .collect();
        Ok(BitVec {
            vec: vec.try_into().unwrap(),
        })
    }
}

fn main() {
    let input = include_str!("../input.txt");

    let values: Vec<BitVec> = input
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() {
                return None;
            }

            match line.parse() {
                Ok(command) => Some(command),
                _ => None,
            }
        })
        .collect();

    part1(&values);
    part2(&values);
}

fn bitvec_to_decimal(vec: [bool; SIZE]) -> u64 {
    vec.iter().fold(0, |acc, val| {
        acc * 2
            + match val {
                true => 1,
                false => 0,
            }
    })
}

fn part1(values: &Vec<BitVec>) {
    let counts = values.iter().fold([(0, 0); SIZE], |mut acc, value| {
        for index in 0..SIZE {
            let (zeroes, ones) = acc[index];
            if value.vec[index] {
                acc[index] = (zeroes, ones + 1);
            } else {
                acc[index] = (zeroes + 1, ones);
            }
        }
        return acc;
    });

    let min_vec: [bool; SIZE] = counts.map(|(zeroes, ones)| ones < zeroes);
    let max_vec: [bool; SIZE] = counts.map(|(zeroes, ones)| ones > zeroes);

    let gamma = bitvec_to_decimal(max_vec);
    let epsilon = bitvec_to_decimal(min_vec);

    println!("Power Consumption = {}", gamma * epsilon);
}

fn part2(values: &Vec<BitVec>) {
    let o2 = oxygen_generator_rating(values.iter().map(|v| v.vec.clone()).collect());
    let co2 = co2_scrubber_rating(values.iter().map(|v| v.vec.clone()).collect());
    println!("Life Support = {}", o2 * co2);
}

fn bits_count(values: &Vec<[bool; SIZE]>) -> [(usize, usize); SIZE] {
    values.iter().fold([(0, 0); SIZE], |mut acc, value| {
        for index in 0..SIZE {
            let (zeroes, ones) = acc[index];
            if value[index] {
                acc[index] = (zeroes, ones + 1);
            } else {
                acc[index] = (zeroes + 1, ones);
            }
        }
        acc
    })
}

fn oxygen_generator_rating(values: Vec<[bool; SIZE]>) -> u64 {
    rating_calculator(values, 0, |(zeroes, ones)| ones >= zeroes)
}

fn co2_scrubber_rating(values: Vec<[bool; SIZE]>) -> u64 {
    rating_calculator(values, 0, |(zeroes, ones)| ones < zeroes)
}

fn rating_calculator<F>(values: Vec<[bool; SIZE]>, index: usize, bit_criteria: F) -> u64
where
    F: Fn((usize, usize)) -> bool,
{
    let counts = bits_count(&values);
    let bit_for_index = bit_criteria(counts[index]);
    let filtered_values: Vec<[bool; SIZE]> = values
        .into_iter()
        .filter(|value| value[index] == bit_for_index)
        .collect();
    if filtered_values.len() == 1 {
        bitvec_to_decimal(filtered_values[0])
    } else {
        rating_calculator(filtered_values, index + 1, bit_criteria)
    }
}
