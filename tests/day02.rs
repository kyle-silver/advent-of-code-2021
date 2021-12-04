const INPUT: &str = include_str!("res/02.txt");

#[derive(Debug)]
enum Direction {
    Forward(i64),
    Down(i64),
    Up(i64),
}

impl Direction {
    fn parse(line: &str) -> Direction {
        let (instr, amount) = line.split_once(' ').unwrap();
        let amount = amount.parse().unwrap();
        match instr {
            "forward" => Direction::Forward(amount),
            "down" => Direction::Down(amount),
            "up" => Direction::Up(amount),
            _ => panic!("unrecognized instruction"),
        }
    }
}

#[derive(Debug, Default)]
struct Submarine {
    horizontal: i64,
    depth: i64,
    aim: i64,
}

impl Submarine {
    fn update(&mut self, dir: Direction) {
        match dir {
            Direction::Forward(x) => self.horizontal += x,
            Direction::Down(x) => self.depth += x,
            Direction::Up(x) => self.depth -= x,
        }
    }

    fn smart_update(&mut self, dir: Direction) {
        match dir {
            Direction::Forward(x) => {
                self.horizontal += x;
                self.depth += x * self.aim;
            }
            Direction::Down(x) => self.aim += x,
            Direction::Up(x) => self.aim -= x,
        }
    }
}

#[test]
fn part1() {
    let directions: Vec<_> = INPUT.lines().map(Direction::parse).collect();
    let mut sub = Submarine::default();
    for direction in directions {
        sub.update(direction);
    }
    println!("Day 2, part 1: {}", sub.horizontal * sub.depth);
    assert_eq!(2091984, sub.horizontal * sub.depth)
}

#[test]
fn part2() {
    let directions: Vec<_> = INPUT.lines().map(Direction::parse).collect();
    let mut sub = Submarine::default();
    for direction in directions {
        sub.smart_update(direction);
    }
    println!("Day 2, part 1: {}", sub.horizontal * sub.depth);
    assert_eq!(2086261056, sub.horizontal * sub.depth)
}
