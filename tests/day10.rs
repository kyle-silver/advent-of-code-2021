use std::collections::VecDeque;

use itertools::Itertools;

const INPUT: &str = include_str!("res/10.txt");

#[derive(Debug, PartialEq, Eq)]
enum LineValidation {
    Valid,
    Invalid(char),
    Incomplete(Vec<char>),
}

fn partner(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
        _ => panic!("invalid token"),
    }
}

fn validate(input: &[char]) -> LineValidation {
    let mut stack = VecDeque::new();
    for token in input {
        match token {
            '(' | '[' | '{' | '<' => {
                stack.push_front(*token);
            }
            _ => match stack.pop_front() {
                Some(c) => {
                    if c != partner(*token) {
                        return LineValidation::Invalid(*token);
                    }
                }
                // too many closing characters
                None => return LineValidation::Invalid(*token),
            },
        };
    }
    if stack.is_empty() {
        return LineValidation::Valid;
    }
    LineValidation::Incomplete(stack.iter().map(|c| partner(*c)).collect_vec())
}

fn closing_score(tokens: &[char]) -> u64 {
    let mut score = 0;
    for token in tokens {
        score *= 5;
        score += match token {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => panic!("unexpected token"),
        };
    }
    score
}

#[test]
fn test_invalid_detection() {
    assert_eq!(
        LineValidation::Invalid('}'),
        validate(&"{([(<{}[<>[]}>{[]{[(<()>".chars().collect_vec())
    );
    assert_eq!(
        LineValidation::Invalid(')'),
        validate(&"[[<[([]))<([[{}[[()]]]".chars().collect_vec())
    );
    assert_eq!(
        LineValidation::Invalid(']'),
        validate(&"[{[{({}]{}}([{[{{{}}([]".chars().collect_vec())
    );
    assert_eq!(
        LineValidation::Invalid(')'),
        validate(&"[<(<(<(<{}))><([]([]()".chars().collect_vec())
    );
    assert_eq!(
        LineValidation::Invalid('>'),
        validate(&"<{([([[(<>()){}]>(<<{{".chars().collect_vec())
    );
}

#[test]
fn test_completion() {
    assert_eq!(
        LineValidation::Incomplete("}}]])})]".chars().collect_vec()),
        validate(&"[({(<(())[]>[[{[]{<()<>>".chars().collect_vec())
    );
}

#[test]
fn part1() {
    let ans: u32 = INPUT
        .lines()
        .map(|line| line.chars().collect_vec())
        .filter_map(|tokens| match validate(&tokens) {
            LineValidation::Invalid(c) => Some(c),
            _ => None,
        })
        .map(|c| match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => unreachable!(),
        })
        .sum();
    println!("Day 10, part 1: {}", ans);
    assert_eq!(343863, ans);
}

#[test]
fn part2() {
    let mut scores = INPUT
        .lines()
        .map(str::chars)
        .map(itertools::Itertools::collect_vec)
        .filter_map(|tokens| match validate(&tokens) {
            LineValidation::Incomplete(expected) => Some(expected),
            _ => None,
        })
        .map(|tokens| closing_score(&tokens))
        .collect_vec();
    scores.sort_unstable();
    let ans = scores[scores.len() / 2];
    println!("Day 10, part 2: {}", ans);
    assert_eq!(2924734236, ans);
}
