use std::collections::{hash_map::Entry, HashMap, HashSet};

const INPUT: &str = include_str!("res/05.txt");

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point(i32, i32);

impl Point {
    fn parse(pair: &str) -> Point {
        let (x, y) = pair.split_once(',').unwrap();
        Point(x.parse().unwrap(), y.parse().unwrap())
    }
}

#[derive(Debug, Clone)]
struct Line(Point, Point);

impl Line {
    fn parse(line: &str) -> Line {
        let (start, end) = line.split_once(" -> ").unwrap();
        Line(Point::parse(start), Point::parse(end))
    }

    fn horizontal(&self) -> bool {
        self.0 .1 == self.1 .1
    }

    fn vertical(&self) -> bool {
        self.0 .0 == self.1 .0
    }

    fn horizontal_or_vertical(&self) -> bool {
        self.horizontal() || self.vertical()
    }

    fn horizontal_slope(&self) -> i32 {
        self.1 .0 - self.0 .0
    }

    fn vertical_slope(&self) -> i32 {
        self.1 .1 - self.0 .1
    }
}

impl<'a> IntoIterator for &'a Line {
    type Item = Point;

    type IntoIter = LineIterator;

    fn into_iter(self) -> Self::IntoIter {
        LineIterator {
            line: self.clone(),
            position: self.0.clone(),
            completed: false,
        }
    }
}

#[derive(Debug)]
struct LineIterator {
    line: Line,
    position: Point,
    completed: bool,
}

impl Iterator for LineIterator {
    type Item = Point;

    /*
     (6, 4) -> (2, 0)
     (6, 4)
     m = (y2 - y1) / (x2 - x1) [slope]
     m = -4 / -4 = 1
     (6, 4) + m(1,1)
     (x(t), y(t)) = (6 - t, 4 - t)
     (6,4) -> 5,3 -> 4,2 -> 3,1 -> 2,0

     (9,4) -> (3,4)
     (xt, yt) = (9 - t, 4)

     (5,5) -> (8,2)
     (xt, yt) = (5 + t, 5 - t)
    */

    fn next(&mut self) -> Option<Self::Item> {
        if self.completed {
            return None;
        }
        if self.position == self.line.1 {
            self.completed = true;
        }
        // only works for horizontal and vertical lines
        let mut next = self.position.clone();
        if self.line.horizontal() {
            next.0 += self.line.horizontal_slope().signum();
        } else if self.line.vertical() {
            next.1 += self.line.vertical_slope().signum();
        } else {
            unimplemented!()
        }
        let to_return = self.position.clone();
        self.position = next;
        Some(to_return)
    }
}

#[derive(Debug, Default)]
struct Grid(HashMap<Point, u32>);

impl Grid {
    fn update(&mut self, line: &Line) {
        for point in line {
            match self.0.entry(point) {
                Entry::Occupied(mut e) => *e.get_mut() += 1,
                Entry::Vacant(e) => {
                    let _ = *e.insert(1);
                }
            }
        }
    }

    fn intersections(&self) -> HashSet<&Point> {
        self.0
            .iter()
            .filter_map(|(k, v)| if *v > 1 { Some(k) } else { None })
            .collect()
    }
}

#[test]
fn part1() {
    let lines: Vec<_> = INPUT
        .lines()
        .map(Line::parse)
        .filter(Line::horizontal_or_vertical)
        .collect();
    let mut grid = Grid::default();
    for line in &lines {
        grid.update(line);
    }
    let ans = grid.intersections().len();
    println!("Day 5, part 1: {}", ans);
    assert_eq!(6710, ans);
}
