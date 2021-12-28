const INPUT: &str = include_str!("res/07.txt");

fn optimal_score(positions: &[u32]) -> u32 {
    let max = *positions.iter().max().unwrap();
    (0..=max).map(|i| score(i, positions)).min().unwrap()
}

fn score(target: u32, positions: &[u32]) -> u32 {
    positions
        .iter()
        .map(|p| (target as i32 - *p as i32).abs() as u32)
        .sum()
}

fn optimal_crab_cost(positions: &[u32]) -> u32 {
    let max = *positions.iter().max().unwrap();
    (0..=max).map(|i| crab_score(i, positions)).min().unwrap()
}

fn crab_score(target: u32, positions: &[u32]) -> u32 {
    positions.iter().map(|p| crab_cost(*p, target)).sum()
}

fn crab_cost(start: u32, end: u32) -> u32 {
    let distance = (start as i32 - end as i32).abs() as u32;
    (distance * (distance + 1)) / 2
}

#[test]
fn part1() {
    let positions: Vec<u32> = INPUT
        .split(',')
        .map(str::parse::<u32>)
        .map(Result::unwrap)
        .collect();
    println!("Day 7, part 1: {}", optimal_score(&positions));
}

#[test]
fn test_crab_cost() {
    assert_eq!(66, crab_cost(16, 5));
}

#[test]
fn part2() {
    let positions: Vec<u32> = INPUT
        .split(',')
        .map(str::parse::<u32>)
        .map(Result::unwrap)
        .collect();
    println!("Day 7, part 2: {}", optimal_crab_cost(&positions));
}
