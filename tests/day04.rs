const INPUT: &str = include_str!("res/04.txt");

#[derive(Debug)]
struct Board {
    board: [[(u32, bool); 5]; 5],
    completed: bool,
}

impl Board {
    fn parse(lines: &[&str]) -> Board {
        let mut board = [[(0, false); 5]; 5];
        for (i, row) in lines.iter().enumerate() {
            let tokens: Vec<u32> = row
                .split_ascii_whitespace()
                .map(str::parse)
                .map(Result::unwrap)
                .collect();
            for j in 0..5 {
                board[i][j].0 = tokens[j];
            }
        }
        Board {
            board,
            completed: false,
        }
    }

    fn mark(&mut self, x: u32) {
        for row in self.board.iter_mut() {
            for (val, marked) in row.iter_mut() {
                if x == *val {
                    *marked = true;
                }
            }
        }
    }

    fn bingo(&self) -> bool {
        // rows
        for row in self.board {
            if row.iter().map(|(_, marked)| marked).all(|b| *b) {
                return true;
            }
        }
        // columns
        for i in 0..5 {
            let col = self
                .board
                .iter()
                .map(|row| row[i])
                .map(|(_, marked)| marked)
                .all(|b| b);
            if col {
                return true;
            }
        }
        return false;
    }

    fn score(&self) -> u32 {
        self.board
            .iter()
            .map(|row| {
                row.iter()
                    .filter_map(|(val, marked)| match marked {
                        true => None,
                        false => Some(val),
                    })
                    .sum::<u32>()
            })
            .sum()
    }
}

fn parse_input() -> (Vec<u32>, Vec<Board>) {
    let mut lines = INPUT.lines().peekable();
    let numbers: Vec<u32> = lines
        .next()
        .unwrap()
        .split(',')
        .map(str::parse)
        .map(Result::unwrap)
        .collect();
    let lines: Vec<_> = lines.collect();
    let mut boards: Vec<Board> = Vec::new();
    let mut index = 0;
    while index < lines.len() {
        boards.push(Board::parse(&lines[index + 1..index + 6]));
        index += 6;
    }
    (numbers, boards)
}

#[test]
fn part1() {
    let (numbers, mut boards) = parse_input();
    let mut ans = None;
    'outer: for number in numbers {
        for board in boards.iter_mut() {
            board.mark(number);
            if board.bingo() {
                ans = Some(board.score() * number);
                break 'outer;
            }
        }
    }
    println!("Day 4, part 1: {:?}", ans);
    assert_eq!(Some(33348), ans);
}

#[test]
fn part2() {
    let (numbers, mut boards) = parse_input();
    let mut winners: Vec<u32> = Vec::new();
    for number in numbers {
        for board in boards.iter_mut().filter(|board| board.completed == false) {
            board.mark(number);
            if board.bingo() {
                let score = board.score() * number;
                board.completed = true;
                winners.push(score);
            }
        }
    }
    println!("Day 4, part 2: {:?}", winners.last());
    assert_eq!(Some(&8112), winners.last());
}
