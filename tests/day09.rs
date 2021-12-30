use std::{collections::HashSet, mem::MaybeUninit};

use itertools::Itertools;

const INPUT: &str = include_str!("res/09.txt");

const NEIGHBOR_DELTAS: [(isize, isize); 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];

fn neighboring_points(i: isize, j: isize) -> [(usize, usize); 4] {
    let mut neighbors: [MaybeUninit<(usize, usize)>; 4] =
        unsafe { MaybeUninit::uninit().assume_init() };
    for (n, delta) in NEIGHBOR_DELTAS.iter().enumerate() {
        neighbors[n] = MaybeUninit::new(((i + delta.0) as usize, (j + delta.1) as usize));
    }
    unsafe { std::mem::transmute::<_, [(usize, usize); 4]>(neighbors) }
}

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
    }

    fn neighbors(&self, i: isize, j: isize) -> Vec<u32> {
        neighboring_points(i, j)
            .iter()
            .filter_map(|(r, c)| self.get(*r as isize, *c as isize))
            .collect()
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

    fn low_points(&self) -> Vec<(usize, usize)> {
        let mut low_points = vec![];
        for i in 0..self.0.len() {
            for j in 0..self.0.get(i).unwrap().len() {
                if self.risk_score(i, j) != 0 {
                    low_points.push((i, j));
                }
            }
        }
        low_points
    }

    fn fill_basin(&self, i: usize, j: usize, visited: &mut HashSet<(usize, usize)>) {
        let unvisited = neighboring_points(i as isize, j as isize)
            .iter()
            .filter(|point| !visited.contains(*point))
            .filter(|(r, c)| {
                self.get(*r as isize, *c as isize)
                    .filter(|v| *v < 9)
                    .is_some()
            })
            .map(|p| *p)
            .collect_vec();
        for point in &unvisited {
            visited.insert(point.clone());
        }
        for (r, c) in unvisited {
            self.fill_basin(r, c, visited);
        }
    }
}

#[test]
fn part1() {
    let grid: Grid = INPUT.into();
    let ans = grid.total_risk();
    println!("Day 9, part 1: {}", ans);
    assert_eq!(491, ans);
}

#[test]
fn part2() {
    let grid: Grid = INPUT.into();
    let low_points = grid.low_points();
    let mut basins = vec![];
    for (i, j) in low_points {
        let mut visited = HashSet::new();
        visited.insert((i, j));
        grid.fill_basin(i, j, &mut visited);
        println!("Low Point: {:?}, size: {}", (i, j), visited.len());
        for point in &visited {
            println!("\t{:?}", point);
        }
        basins.push(visited);
    }
    let mut sizes = basins.iter().map(|b| b.len()).collect_vec();
    sizes.sort_unstable();
    let ans: usize = sizes.iter().rev().take(3).product();
    println!("Day 9, part 2: {}", ans)
}
