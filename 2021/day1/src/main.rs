fn main() {
    let input = include_str!("../input.txt");

    let values: Vec<i64> = input
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() {
                return None;
            }

            Some(line.parse().unwrap())
        })
        .collect();

    part1(&values);
    part2(&values);
}

fn part1(values: &Vec<i64>) {
    let count = values.windows(2).filter(|x| x[1] > x[0]).count();
    println!("Increased values = {}", count);
}

fn part2(values: &Vec<i64>) {
    let count = values.windows(4).filter(|x| x[3] > x[0]).count();
    println!("Increased values = {}", count);
}
