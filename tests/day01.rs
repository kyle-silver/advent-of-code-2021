const INPUT: &str = include_str!("res/01.txt");

#[test]
fn part1() {
    let data: Vec<i32> = INPUT.lines().map(str::parse).map(Result::unwrap).collect();
    let times_increased = data.windows(2).filter(|pair| pair[1] > pair[0]).count();
    println!("Day 1, part 1: {}", times_increased);
    assert_eq!(times_increased, 1559);
}

#[test]
fn part2() {
    let data: Vec<i32> = INPUT.lines().map(str::parse).map(Result::unwrap).collect();
    let times_increased = data
        .windows(4)
        .filter(|quartet| quartet[1..].iter().sum::<i32>() > quartet[..3].iter().sum::<i32>())
        .count();
    println!("Day 1, part 2: {}", times_increased);
    assert_eq!(times_increased, 1600)
}
