use std::collections::{hash_map::Entry, HashMap};

use itertools::Itertools;

const INPUT: &str = include_str!("res/14.txt");
const SEED: &str = "HBHVVNPCNFPSVKBPPCBH";

#[derive(Debug)]
struct Pairs {
    pairs: HashMap<(char, char), u64>,
    seed: String,
}

impl Pairs {
    fn from(seed: &str) -> Self {
        let pairs = seed
            .chars()
            .tuple_windows()
            .fold(HashMap::new(), |mut map, pair| {
                match map.entry(pair) {
                    Entry::Occupied(mut occupied) => {
                        *occupied.get_mut() += 1;
                    }
                    Entry::Vacant(vacant) => {
                        vacant.insert(1);
                    }
                };
                return map;
            });
        Self {
            pairs,
            seed: seed.into(),
        }
    }

    fn char_occurrences(&self) -> HashMap<char, u64> {
        let mut occurrences = self.pairs.iter().map(|((l, _), count)| (l, count)).fold(
            HashMap::new(),
            |mut map, (&c, &count)| {
                match map.entry(c) {
                    Entry::Occupied(mut occupied) => {
                        *occupied.get_mut() += count;
                    }
                    Entry::Vacant(vacant) => {
                        vacant.insert(count);
                    }
                };
                return map;
            },
        );
        // a very annoying off-by-one error to account for the last element
        if let Some(last_character) = self.seed.chars().last() {
            *occurrences.get_mut(&last_character).unwrap() += 1;
        }
        return occurrences;
    }

    fn max_min_diff(&self) -> u64 {
        let occurrences = self.char_occurrences();
        let mut iter = occurrences.values();
        let mut min = *iter.next().unwrap();
        let mut max = min;
        for &val in iter {
            if val < min {
                min = val;
                continue;
            }
            if val > max {
                max = val;
            }
        }
        return max - min;
    }
}

#[derive(Debug)]
struct Rules(HashMap<(char, char), char>);

impl Rules {
    fn from<'a>(lines: impl Iterator<Item = &'a str>) -> Self {
        let rules = lines
            .map(|line| line.split_once(" -> ").unwrap())
            .map(|(first, second)| {
                let mut first = first.chars();
                let a = first.next().unwrap();
                let b = first.next().unwrap();
                let c = second.chars().next().unwrap();
                ((a, b), c)
            })
            .collect();
        return Rules(rules);
    }

    fn next(&self, pairs: &Pairs) -> Pairs {
        let new = pairs
            .pairs
            .iter()
            .filter_map(|(pair, &count)| {
                if let Some(&generated) = self.0.get(pair) {
                    let l = ((pair.0, generated), count);
                    let r = ((generated, pair.1), count);
                    return Some([l, r]);
                }
                None
            })
            .flat_map(|arr| arr.into_iter())
            .fold(HashMap::new(), |mut map, (pair, count)| {
                match map.entry(pair) {
                    Entry::Occupied(mut occupied) => {
                        *occupied.get_mut() += count;
                    }
                    Entry::Vacant(vacant) => {
                        vacant.insert(count);
                    }
                }
                return map;
            });
        return Pairs {
            pairs: new,
            seed: pairs.seed.clone(),
        };
    }
}

#[test]
fn part1() {
    let mut pairs = Pairs::from(SEED);
    let rules = Rules::from(INPUT.lines());
    for _ in 0..10 {
        pairs = rules.next(&pairs);
    }
    let ans = pairs.max_min_diff();
    println!("Day 14, part 1: {ans}");
    assert_eq!(4244, ans);
}

#[test]
fn part2() {
    let mut pairs = Pairs::from(SEED);
    let rules = Rules::from(INPUT.lines());
    for _ in 0..40 {
        pairs = rules.next(&pairs);
    }
    let ans = pairs.max_min_diff();
    println!("Day 14, part 2: {ans}");
    assert_eq!(4807056953866, ans);
}
