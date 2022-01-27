use std::cmp;

fn main() {
    let input = include_str!("../input.txt");
    let lines: Vec<((usize, usize), (usize, usize))> = input
        .lines()
        .map(|line| {
            let (pt1_str, pt2_str) = line.split_once(" -> ").unwrap();
            let (pt1x, pt1y) = pt1_str.split_once(",").unwrap();
            let (pt2x, pt2y) = pt2_str.split_once(",").unwrap();

            let pt1: (usize, usize) = (pt1x.parse().unwrap(), pt1y.parse().unwrap());
            let pt2: (usize, usize) = (pt2x.parse().unwrap(), pt2y.parse().unwrap());

            (pt1, pt2)
        })
        .collect();

    part1(&lines);
    part2(&lines);
}

fn part1(lines: &Vec<((usize, usize), (usize, usize))>) {
    let max_x = lines.iter().fold(0, |acc, ((x1, _), (x2, _))| {
        cmp::max(acc, cmp::max(*x1, *x2))
    });
    let max_y = lines.iter().fold(0, |acc, ((_, y1), (_, y2))| {
        cmp::max(acc, cmp::max(*y1, *y2))
    });
    let mut grid: Vec<Vec<usize>> = Vec::with_capacity(max_y);
    for _ in 0..max_y + 1 {
        let mut row: Vec<usize> = Vec::with_capacity(max_x);
        for _ in 0..max_x + 1 {
            row.push(0);
        }
        grid.push(row);
    }

    for ((x1, y1), (x2, y2)) in lines {
        let direction: Option<(i32, i32)> = if x1 == x2 {
            if y1 < y2 {
                Some((0, 1))
            } else {
                Some((0, -1))
            }
        } else if y1 == y2 {
            if x1 < x2 {
                Some((1, 0))
            } else {
                Some((-1, 0))
            }
        } else {
            None
        };
        if direction.is_none() {
            continue;
        }
        let (dir_x, dir_y) = direction.unwrap();
        let mut x = *x1;
        let mut y = *y1;
        while x != *x2 || y != *y2 {
            grid[y][x] += 1;
            if dir_x == 1 {
                x += 1;
            } else if dir_x == -1 {
                x -= 1;
            }
            if dir_y == 1 {
                y += 1;
            } else if dir_y == -1 {
                y -= 1;
            }
        }
        grid[y][x] += 1;
    }

    let count: usize = grid
        .iter()
        .map(|row| row.iter().filter(|val| **val >= 2).count())
        .sum();

    println!("{}", count);
}

fn part2(lines: &Vec<((usize, usize), (usize, usize))>) {
    let max_x = lines.iter().fold(0, |acc, ((x1, _), (x2, _))| {
        cmp::max(acc, cmp::max(*x1, *x2))
    });
    let max_y = lines.iter().fold(0, |acc, ((_, y1), (_, y2))| {
        cmp::max(acc, cmp::max(*y1, *y2))
    });
    let mut grid: Vec<Vec<usize>> = Vec::with_capacity(max_y);
    for _ in 0..max_y + 1 {
        let mut row: Vec<usize> = Vec::with_capacity(max_x);
        for _ in 0..max_x + 1 {
            row.push(0);
        }
        grid.push(row);
    }

    for ((x1, y1), (x2, y2)) in lines {
        let direction: (i32, i32) = if x1 == x2 {
            if y1 < y2 {
                (0, 1)
            } else {
                (0, -1)
            }
        } else if y1 == y2 {
            if x1 < x2 {
                (1, 0)
            } else {
                (-1, 0)
            }
        } else {
            match (x1 < x2, y1 < y2) {
                (true, true) => (1, 1),
                (true, false) => (1, -1),
                (false, true) => (-1, 1),
                (false, false) => (-1, -1),
            }
        };
        let (dir_x, dir_y) = direction;
        let mut x = *x1;
        let mut y = *y1;
        while x != *x2 || y != *y2 {
            grid[y][x] += 1;
            if dir_x == 1 {
                x += 1;
            } else if dir_x == -1 {
                x -= 1;
            }
            if dir_y == 1 {
                y += 1;
            } else if dir_y == -1 {
                y -= 1;
            }
        }
        grid[y][x] += 1;
    }

    let count: usize = grid
        .iter()
        .map(|row| row.iter().filter(|val| **val >= 2).count())
        .sum();

    println!("{}", count);
}
