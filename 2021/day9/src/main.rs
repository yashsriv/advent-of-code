use std::collections::{HashSet, VecDeque};

fn main() {
    let input = include_str!("../input.txt");
    let map: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|ch| ch.to_digit(10).unwrap()).collect())
        .collect();

    let mut risk_level_sum = 0;
    let mut basin_sizes = vec![];

    for row in 0..map.len() {
        for col in 0..map[row].len() {
            let current_height = map[row][col];
            let mut is_low_point = true;

            if row > 0 && map[row - 1][col] <= current_height {
                is_low_point = false;
            }

            if row != map.len() - 1 && map[row + 1][col] <= current_height {
                is_low_point = false;
            }

            if col > 0 && map[row][col - 1] <= current_height {
                is_low_point = false;
            }

            if col != map[row].len() - 1 && map[row][col + 1] <= current_height {
                is_low_point = false;
            }

            if is_low_point {
                let risk_level = 1 + current_height;
                risk_level_sum += risk_level;

                // Calculate basin size over here
                let mut basin_size = 0;
                let mut queue = VecDeque::new();
                let mut visited = HashSet::new();
                queue.push_back((row, col));

                while !queue.is_empty() {
                    let (r, c) = queue.pop_front().unwrap();
                    if map[r][c] == 9 || visited.contains(&(r, c)) {
                        continue;
                    }
                    visited.insert((r, c));
                    basin_size += 1;

                    if r > 0 && map[r - 1][c] >= map[r][c] {
                        queue.push_back((r - 1, c));
                    }
                    if c > 0 && map[r][c - 1] >= map[r][c] {
                        queue.push_back((r, c - 1));
                    }
                    if c != map[r].len() - 1 && map[r][c + 1] >= map[r][c] {
                        queue.push_back((r, c + 1));
                    }
                    if r != map.len() - 1 && map[r + 1][c] >= map[r][c] {
                        queue.push_back((r + 1, c));
                    }
                }

                basin_sizes.push(basin_size);
            }
        }
    }

    println!("Risk Level Sum: {}", risk_level_sum);

    basin_sizes.sort();
    let max = basin_sizes.pop().unwrap();
    let max_2 = basin_sizes.pop().unwrap();
    let max_3 = basin_sizes.pop().unwrap();

    println!("Basin Product: {}", max * max_2 * max_3);
}
