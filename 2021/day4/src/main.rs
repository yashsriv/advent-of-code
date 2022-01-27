use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
struct Board {
    elements: [[u64; 5]; 5],
    visited: [[bool; 5]; 5],
}

impl Board {
    /**
     *  Plays a move on the bingo board. Returns new board and
     *  true if an entry was found.
     */
    fn play_move(self: &Self, play: u64) -> (Board, bool) {
        let mut found = false;
        let mut visited = self.visited;
        for row in 0..5 {
            for col in 0..5 {
                if self.elements[row][col] == play {
                    visited[row][col] = true;
                    found = true;
                }
            }
        }
        (
            Board {
                elements: self.elements,
                visited,
            },
            found,
        )
    }

    /**
     *  Checks if board is bingoed.
     */
    fn is_bingo(self: &Self) -> bool {
        // Check rows
        for row in 0..5 {
            let mut is_bingo_row = true;
            for col in 0..5 {
                if !self.visited[row][col] {
                    is_bingo_row = false;
                }
            }
            if is_bingo_row {
                return true;
            }
        }
        // Check cols
        for col in 0..5 {
            let mut is_bingo_col = true;
            for row in 0..5 {
                if !self.visited[row][col] {
                    is_bingo_col = false;
                }
            }
            if is_bingo_col {
                return true;
            }
        }

        false
    }

    fn calc_score(self: &Self, play: u64) -> u64 {
        let mut sum = 0;
        for row in 0..5 {
            for col in 0..5 {
                if !self.visited[row][col] {
                    sum += self.elements[row][col];
                }
            }
        }
        sum * play
    }
}

impl FromStr for Board {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vec: Vec<Vec<u64>> = s
            .lines()
            .filter_map(|line| {
                if line.is_empty() {
                    return None;
                }
                Some(
                    line.split_whitespace()
                        .map(|x| x.parse().unwrap())
                        .collect(),
                )
            })
            .collect();
        let mapped: Vec<[u64; 5]> = vec.into_iter().map(|row| row.try_into().unwrap()).collect();
        Ok(Board {
            elements: mapped.try_into().unwrap(),
            visited: [[false; 5]; 5],
        })
    }
}

fn main() {
    let input = include_str!("../input.txt");

    let unique_splits: Vec<&str> = input.split("\n\n").collect();

    let (moves, boards) = unique_splits.split_at(1);
    let moves: Vec<u64> = moves[0].split(',').map(|x| x.parse().unwrap()).collect();

    let boards: Vec<Board> = boards.iter().map(|split| split.parse().unwrap()).collect();

    part1(&moves, &boards);
    part2(&moves, &boards);
}

fn part1(moves: &Vec<u64>, boards: &Vec<Board>) {
    let mut local_boards = boards.clone();
    for play in moves {
        let mut new_boards: Vec<Board> = Vec::with_capacity(boards.len());
        for board in &local_boards {
            let (new_board, found) = board.play_move(*play);
            new_boards.push(new_board);
            if found && new_board.is_bingo() {
                println!("Bingo! Score = {}", new_board.calc_score(*play));
                return;
            }
        }
        local_boards = new_boards;
    }
}

fn part2(moves: &Vec<u64>, boards: &Vec<Board>) {
    let mut local_boards = boards.clone();
    for play in moves {
        let mut new_boards: Vec<Board> = Vec::with_capacity(boards.len());
        for board in &local_boards {
            let (new_board, found) = board.play_move(*play);
            if !found || !new_board.is_bingo() {
                new_boards.push(new_board);
            }
            if local_boards.len() == 1 && found && new_board.is_bingo() {
                println!("Last Bingo! Score = {}", new_board.calc_score(*play))
            }
        }
        local_boards = new_boards;
    }
}
