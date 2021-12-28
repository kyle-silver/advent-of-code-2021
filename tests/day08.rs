use itertools::Itertools;

const INPUT: &str = include_str!("res/08.txt");

const DISPLAY: [[bool; 7]; 10] = [
    [true, true, true, false, true, true, true],
    [false, false, true, false, false, true, false],
    [true, false, true, true, true, false, true],
    [true, false, true, true, false, true, true],
    [false, true, true, true, false, true, false],
    [true, true, false, true, false, true, true],
    [true, true, false, true, true, true, true],
    [true, false, true, false, false, true, false],
    [true, true, true, true, true, true, true],
    [true, true, true, true, false, true, true],
];

#[derive(Debug)]
struct Mapping {
    inputs: Vec<Vec<char>>,
    outputs: Vec<Vec<char>>,
}

impl From<&str> for Mapping {
    fn from(line: &str) -> Self {
        let (inputs, outputs) = line.split_once(" | ").unwrap();
        let inputs = inputs
            .split(" ")
            .map(|token| token.chars().collect::<Vec<char>>())
            .collect();
        let outputs = outputs
            .split(" ")
            .map(|token| token.chars().collect::<Vec<char>>())
            .collect();
        Self { inputs, outputs }
    }
}

impl Mapping {
    fn unscramble(&self) -> [char; 7] {
        let ans = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g']
            .iter()
            .permutations(7)
            .filter(|combo| self.valid(combo))
            .next()
            .unwrap()
            .into_iter()
            .map(|c| *c)
            .collect_vec();
        ans.try_into().unwrap()
    }

    fn valid(&self, combo: &[&char]) -> bool {
        for digit in self.inputs.iter() {
            let displayed = Mapping::displayed(digit, combo);
            if !DISPLAY.contains(&displayed) {
                return false;
            }
        }
        true
    }

    fn displayed(digit: &[char], combo: &[&char]) -> [bool; 7] {
        let mut displayed = [false; 7];
        for segment in digit {
            match combo.iter().position(|c| *c == segment) {
                Some(index) => displayed[index] = true,
                None => {}
            };
        }
        displayed
    }

    fn output(&self, combo: &[char]) -> u32 {
        let combo = combo.iter().collect_vec();
        self.outputs
            .iter()
            .rev()
            .enumerate()
            .map(|(i, digit)| {
                let displayed = Mapping::displayed(digit, &combo);
                let value = DISPLAY.iter().position(|d| d == &displayed).unwrap();
                (i, value)
            })
            .map(|(i, value)| value as u32 * 10u32.pow(i as u32))
            .sum()
    }
}

#[test]
fn part1() {
    let outputs = INPUT
        .lines()
        .map(|line| line.split_once(" | ").unwrap().1)
        .map(|output| output.split(' ').collect::<Vec<&str>>())
        .flat_map(|outputs| outputs.into_iter())
        .map(|output| output.chars().collect::<Vec<char>>())
        .filter(|output| {
            let len = output.len();
            len == 2 || len == 3 || len == 4 || len == 7
        })
        .count();
    println!("Day 8, part 1: {}", outputs);
}

#[test]
fn part2() {
    let mappings = INPUT.lines().map(Mapping::from).collect_vec();
    let ans: u32 = mappings
        .iter()
        .map(|m| {
            let unscrambled = m.unscramble();
            m.output(&unscrambled)
        })
        .sum();
    println!("Day 8, part 2: {}", ans);
    assert_eq!(1012272, ans);
}
