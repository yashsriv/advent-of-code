const DAYS: usize = 256;

fn main() {
    let input = include_str!("../input.txt");
    let timers: Vec<usize> = input
        .trim()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();
    //part1(&timers);
    part2(&timers);
}

fn part1(timers: &Vec<usize>) {
    //println!("Initial state: {:?}", timers);
    let result = process1(timers, DAYS);
    println!("Number of fish: {}", result);
}

fn process1(timers: &Vec<usize>, remaining_days: usize) -> usize {
    if remaining_days == 0 {
        return timers.len();
    }

    let mut new_timers: Vec<usize> = Vec::new();
    let mut modified_timers: Vec<usize> = timers
        .iter()
        .map(|x| {
            if *x == 0 {
                new_timers.push(8);
                return 6;
            }
            x - 1
        })
        .collect();

    modified_timers.append(&mut new_timers);

    /*
     *println!(
     *    "Remaining days {}: {:?}",
     *    remaining_days - 1,
     *    modified_timers
     *);
     */
    process1(&modified_timers, remaining_days - 1)
}

// I used dynamic programming!!
fn part2(timers: &Vec<usize>) {
    let mut dp: [usize; DAYS + 7] = [1; DAYS + 7];
    for day in 0..DAYS + 7 {
        let mut result = 1;
        let mut temp_day = day;
        while temp_day >= 7 {
            let prev_fish_alive_days = day - (temp_day - temp_day % 7);
            if prev_fish_alive_days > 1 {
                let prev_fish_cycle_days = prev_fish_alive_days - 2;
                result += dp[prev_fish_cycle_days];
            } else {
                result += 1;
            }
            temp_day -= 7;
        }
        dp[day] = result;
    }

    let result: usize = timers.iter().map(|timer| dp[6 - timer + DAYS]).sum();
    println!("Number of fish: {}", result);
}
