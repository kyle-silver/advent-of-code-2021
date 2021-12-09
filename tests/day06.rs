const INPUT: &str = include_str!("res/06.txt");

#[derive(Debug)]
struct Fish(u32);

impl Fish {
    fn tick(&mut self) -> Option<Fish> {
        if self.0 == 0 {
            self.0 = 6;
            return Some(Fish(8));
        }
        self.0 -= 1;
        None
    }
}

#[derive(Debug)]
struct Ocean(Vec<Fish>);

impl Ocean {
    fn tick(&mut self) {
        let mut new_fish = Vec::new();
        for fish in &mut self.0 {
            if let Some(new_one) = fish.tick() {
                new_fish.push(new_one);
            }
        }
        self.0.append(&mut new_fish);
    }
}

#[test]
fn part1() {
    let fish: Vec<_> = INPUT
        .split(',')
        .map(str::parse)
        .map(Result::unwrap)
        .map(Fish)
        .collect();
    let mut ocean = Ocean(fish);
    for _ in 0..80 {
        ocean.tick();
    }
    println!("Day 6, part 1: {}", ocean.0.len());
    assert_eq!(373378, ocean.0.len());
}

#[test]
fn fish_test() {
    let mut ocean = Ocean(vec![Fish(8)]);
    for i in 0..25 {
        println!("{}\t{}\t{:?}", i, ocean.0.len(), ocean);
        ocean.tick();
    }
}

#[test]
fn part2() {
    let fish: Vec<usize> = INPUT
        .split(',')
        .map(str::parse)
        .map(Result::unwrap)
        .collect();
    // not my solution... thank you r/adventofcode...
    let mut counts: [u64; 9] = [0; 9];
    for f in fish.iter() {
        counts[*f] += 1;
    }
    for _ in 0..256 {
        counts.rotate_left(1);
        counts[6] += counts[8];
    }
    let sum: u64 = counts.iter().sum();
    println!("Day 6, part 2: {}", sum);
}
