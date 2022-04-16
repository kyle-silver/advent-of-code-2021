use std::collections::HashSet;

const INPUT: &str = include_str!("res/13.txt");

fn parse(token: &str) -> (u32, u32) {
    let (x, y) = token.split_once(',').unwrap();
    (x.parse().unwrap(), y.parse().unwrap())
}

fn fold_over(x: u32, line: u32) -> u32 {
    if x < line {
        return x;
    }
    println!("{} {}", x, line);
    let distance_from_line = (x - line);
    return line - distance_from_line;
}

fn fold_x(point: (u32, u32), x_line: u32) -> (u32, u32) {
    (fold_over(point.0, x_line), point.1)
}

fn fold_y(point: (u32, u32), y_line: u32) -> (u32, u32) {
    (point.0, fold_over(point.1, y_line))
}

// fn fold(point: (u32, u32), lines: (u32, u32)) -> (u32, u32) {
//     (fold_over(point.0, lines.0), fold_over(point.1, lines.1))
// }

#[test]
fn part1() {
    /*
    fold along x=655
    fold along y=447
    fold along x=327
    fold along y=223
    fold along x=163
    fold along y=111
    fold along x=81
    fold along y=55
    fold along x=40
    fold along y=27
    fold along y=13
    fold along y=6
    // (655, 447)
    // (327, 223)
    // (163, 111)
    // (81, 55)
    // (40, 27)
    // ()
    */
    let points: HashSet<_> = INPUT
        .lines()
        .map(parse)
        .map(|p| fold_x(p, 655))
        .map(|p| fold_y(p, 447))
        .map(|p| fold_x(p, 327))
        .map(|p| fold_y(p, 223))
        .map(|p| fold_x(p, 163))
        .map(|p| fold_y(p, 111))
        .map(|p| fold_x(p, 81))
        .map(|p| fold_y(p, 55))
        .map(|p| fold_x(p, 40))
        .map(|p| fold_y(p, 27))
        .map(|p| fold_y(p, 13))
        .map(|p| fold_y(p, 6))
        .collect();
    // let points: HashSet<_> = INPUT
    //     .lines()
    //     .map(parse)
    //     .map(|p| fold_y(p, 7))
    //     .map(|p| fold_x(p, 5))
    //     .collect();
    for x in 0..10 {
        for y in 0..10 {
            print!("{}", if points.contains(&(x, y)) { '#' } else { '.' });
        }
        println!();
    }
    println!("Day 13, part 1: {}", points.len());
}
