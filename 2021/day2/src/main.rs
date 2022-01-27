use std::str::FromStr;

enum Direction {
    Forward,
    Up,
    Down,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "forward" => Ok(Direction::Forward),
            "down" => Ok(Direction::Down),
            "up" => Ok(Direction::Up),
            _ => Err(()),
        }
    }
}

struct Command {
    direction: Direction,
    value: i64,
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, value) = s.split_once(' ').unwrap();
        Ok(Command {
            direction: direction.parse().unwrap(),
            value: value.parse().unwrap(),
        })
    }
}

fn main() {
    let input = include_str!("../input.txt");

    let commands: Vec<Command> = input
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

    part1(&commands);
    part2(&commands);
}

fn part1(commands: &Vec<Command>) {
    let (horizontal, depth) = commands.iter().fold((0, 0), |acc, item| {
        let (horizontal, depth) = acc;
        match item.direction {
            Direction::Forward => (horizontal + item.value, depth),
            Direction::Down => (horizontal, depth + item.value),
            Direction::Up => (horizontal, depth - item.value),
        }
    });
    println!("Product = {}", horizontal * depth);
}

fn part2(commands: &Vec<Command>) {
    let (horizontal, depth, _) = commands.iter().fold((0, 0, 0), |acc, item| {
        let (horizontal, depth, aim) = acc;
        match item.direction {
            Direction::Forward => (horizontal + item.value, depth + aim * item.value, aim),
            Direction::Down => (horizontal, depth, aim + item.value),
            Direction::Up => (horizontal, depth, aim - item.value),
        }
    });
    println!("Product = {}", horizontal * depth);
}
