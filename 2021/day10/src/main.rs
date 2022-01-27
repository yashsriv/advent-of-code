fn main() {
    let input = include_str!("../input.txt");
    let navigations: Vec<&str> = input.lines().collect();
    part1(&navigations);
    part2(&navigations);
}

fn part1(navigations: &Vec<&str>) {
    let result: u32 = navigations
        .iter()
        .filter_map(|navigation| {
            let mut stack = vec![];
            for ch in navigation.chars() {
                match ch {
                    '(' => stack.push(ch),
                    '[' => stack.push(ch),
                    '{' => stack.push(ch),
                    '<' => stack.push(ch),
                    ')' => {
                        match stack.last() {
                            Some('(') => stack.pop(),
                            _ => return Some(3),
                        };
                    }
                    ']' => {
                        match stack.last() {
                            Some('[') => stack.pop(),
                            _ => return Some(57),
                        };
                    }
                    '}' => {
                        match stack.last() {
                            Some('{') => stack.pop(),
                            _ => return Some(1197),
                        };
                    }
                    '>' => {
                        match stack.last() {
                            Some('<') => stack.pop(),
                            _ => return Some(25137),
                        };
                    }
                    _ => unreachable!("can't reach here"),
                };
            }
            return None;
        })
        .sum();

    println!("Result: {}", result);
}

fn part2(navigations: &Vec<&str>) {
    let mut result: Vec<u64> = navigations
        .iter()
        .filter_map(|navigation| {
            let mut stack = vec![];
            for ch in navigation.chars() {
                match ch {
                    '(' => stack.push(ch),
                    '[' => stack.push(ch),
                    '{' => stack.push(ch),
                    '<' => stack.push(ch),
                    ')' => {
                        match stack.last() {
                            Some('(') => stack.pop(),
                            _ => return None,
                        };
                    }
                    ']' => {
                        match stack.last() {
                            Some('[') => stack.pop(),
                            _ => return None,
                        };
                    }
                    '}' => {
                        match stack.last() {
                            Some('{') => stack.pop(),
                            _ => return None,
                        };
                    }
                    '>' => {
                        match stack.last() {
                            Some('<') => stack.pop(),
                            _ => return None,
                        };
                    }
                    _ => unreachable!("can't reach here"),
                };
            }
            if stack.is_empty() {
                return None;
            }
            let mut score: u64 = 0;
            while !stack.is_empty() {
                match stack.pop() {
                    Some('(') => score = score * 5 + 1,
                    Some('[') => score = score * 5 + 2,
                    Some('{') => score = score * 5 + 3,
                    Some('<') => score = score * 5 + 4,
                    _ => unreachable!("should be one of the above"),
                }
            }
            return Some(score);
        })
        .collect();

    result.sort();

    println!("Result: {}", result[(result.len() - 1) / 2]);
}
