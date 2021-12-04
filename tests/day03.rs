const INPUT: &str = include_str!("res/03.txt");

#[derive(Debug, Default, Clone, Copy)]
struct Frequencies {
    zeros: usize,
    ones: usize,
}

fn most_common<const N: usize>(data: &[[usize; N]]) -> [usize; N] {
    let mut counts = [Frequencies::default(); N];
    for row in data {
        for (i, c) in row.iter().enumerate() {
            match c {
                0 => counts[i].zeros += 1,
                _ => counts[i].ones += 1,
            }
        }
    }
    let mut result = [0; N];
    for (i, frequencies) in counts.iter().enumerate() {
        result[i] = if frequencies.zeros > frequencies.ones {
            0
        } else {
            1
        };
    }
    result
}

fn inverse<const N: usize>(data: &[usize; N]) -> [usize; N] {
    data.iter()
        .map(|x| if *x == 0 { 1 } else { 0 })
        .collect::<Vec<usize>>()
        .try_into()
        .unwrap()
}

fn score(most_common_bits: &[usize]) -> i64 {
    most_common_bits
        .iter()
        .rev()
        .enumerate()
        .filter(|(_, bit)| **bit != 0)
        .map(|(i, _)| 2i64.pow(i as u32))
        .sum()
}

fn oxygen_rating<const N: usize>(mut data: Vec<[usize; N]>) -> [usize; N] {
    let mut index = 0;
    while data.len() != 1 {
        let most_common_bit = most_common(&data)[index];
        data = data
            .into_iter()
            .filter(|num| num[index] == most_common_bit)
            .collect();
        index = (index + 1) % N;
    }
    *data.first().unwrap()
}

fn co2_rating<const N: usize>(mut data: Vec<[usize; N]>) -> [usize; N] {
    let mut index = 0;
    while data.len() != 1 {
        let least_common_bit = inverse(&most_common(&data))[index];
        data = data
            .into_iter()
            .filter(|num| num[index] == least_common_bit)
            .collect();
        index = (index + 1) % N;
    }
    *data.first().unwrap()
}

#[test]
fn part1() {
    let data: Vec<[usize; 12]> = INPUT
        .lines()
        .map(|line| {
            let data: Vec<usize> = line
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect();
            data.try_into().unwrap()
        })
        .collect();
    let most_common_bits = most_common(&data);
    let gamma_rate = score(&most_common_bits);
    let epsilon_rate = (2i64.pow(12) - 1) - gamma_rate;
    let ans = gamma_rate * epsilon_rate;
    println!("Day 3, part 1: {}", ans);
    assert_eq!(852500, ans);
}

#[test]
fn part2() {
    let data: Vec<[usize; 12]> = INPUT
        .lines()
        .map(|line| {
            let data: Vec<usize> = line
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect();
            data.try_into().unwrap()
        })
        .collect();
    let ans = score(&oxygen_rating(data.clone())) * score(&co2_rating(data));
    println!("Day 3, part 2: {}", ans);
    assert_eq!(1007985, ans);
}
