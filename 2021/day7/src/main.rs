fn main() {
    let input = include_str!("../input.txt");
    let mut positions: Vec<i32> = input
        .trim()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();
    positions.sort();
    let size = positions.len();
    let result1 = if size % 2 == 0 {
        std::cmp::min(
            calc_cost(&positions, positions[size / 2 - 1]),
            calc_cost(&positions, positions[size / 2]),
        )
    } else {
        calc_cost(&positions, positions[(size - 1) / 2])
    };
    println!("Result: {}", result1);

    let sum: i32 = positions.iter().sum();
    let mean: f32 = sum as f32 / size as f32;
    println!("Result: {}", calc_cost2(&positions, mean.round() as i32));

    part2(&positions);
}

fn calc_cost(positions: &Vec<i32>, target: i32) -> i32 {
    positions.iter().map(|x| (target - x).abs()).sum()
}

fn calc_cost2(positions: &Vec<i32>, target: i32) -> i32 {
    positions
        .iter()
        .map(|x| {
            let diff = (target - x).abs();
            ((diff * (diff + 1)) as f32 / 2.0) as i32
        })
        .sum()
}

fn part2(positions: &Vec<i32>) {
    let result = positions.iter().map(|x| [calc_cost2(positions, *x)]).min();
    println!("{:?}", result);
}
