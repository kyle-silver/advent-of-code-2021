use itertools::Itertools;

const INPUT: &str = include_str!("res/09.txt");

#[derive(Debug)]
struct Grid(Vec<Vec<u32>>);

impl From<&str> for Grid {
    fn from(input: &str) -> Self {
        let grid = input
            .lines()
            .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
            .collect_vec();
        Grid(grid)
    }
}

impl Grid {
    fn get(&self, i: isize, j: isize) -> Option<u32> {
        self.0
            .get(i as usize)
            .and_then(|v| v.get(j as usize).map(|x| *x).or(None))
            .or(None)
    }

    fn neighbors(&self, i: isize, j: isize) -> Vec<u32> {
        let mut neighbors = vec![];
        if let Some(n) = self.get(i - 1, j) {
            neighbors.push(n);
        }
        if let Some(n) = self.get(i, j - 1) {
            neighbors.push(n);
        }
        if let Some(n) = self.get(i, j + 1) {
            neighbors.push(n);
        }
        if let Some(n) = self.get(i + 1, j) {
            neighbors.push(n);
        }
        neighbors
    }

    fn risk_score(&self, i: usize, j: usize) -> u32 {
        let (i, j) = (i as isize, j as isize);
        let x = self.get(i, j).unwrap();
        if self.neighbors(i, j).iter().all(|v| *v > x) {
            x + 1
        } else {
            0
        }
    }

    fn total_risk(&self) -> u32 {
        let mut risk = 0;
        for i in 0..self.0.len() {
            for j in 0..self.0.get(i).unwrap().len() {
                risk += self.risk_score(i, j);
            }
        }
        risk
    }
}

#[test]
fn part1() {
    let grid: Grid = INPUT.into();
    println!("Day 9, part 1: {}", grid.total_risk());
}
