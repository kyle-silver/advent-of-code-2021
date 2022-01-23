const INPUT: &str = include_str!("res/11ex.txt");

const NEIGHBOR_DELTAS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

#[derive(Debug)]
struct Grid<const N: usize> {
    flashes: u32,
    octopodes: [[u32; N]; N],
}

impl<const N: usize> Grid<N> {
    fn parse<'a>(mut lines: impl Iterator<Item = &'a str>) -> Option<Grid<N>> {
        let mut octopodes = [[0; N]; N];
        for row in &mut octopodes {
            let mut chars = lines.next()?.chars();
            for val in row.iter_mut() {
                *val = chars.next()?.to_digit(10)?;
            }
        }
        Some(Grid {
            flashes: 0,
            octopodes,
        })
    }

    fn get(&mut self, point: (isize, isize)) -> Option<&mut u32> {
        self.octopodes
            .get_mut(point.0 as usize)
            .and_then(|arr| arr.get_mut(point.1 as usize))
    }

    fn update(&mut self) {
        // first, increment everything by 1
        for row in self.octopodes.iter_mut() {
            for val in row.iter_mut() {
                *val += 1;
            }
        }
        // simulate the flash but don't zero anything out

        for i in 0..N {
            for j in 0..N {
                let val = &mut self.octopodes[i][j];
                if val <= &mut 9 {
                    continue;
                }
                for delta in NEIGHBOR_DELTAS {
                    if let Some(v) = self.get((i as isize + delta.0, j as isize + delta.1)) {
                        *v += 1;
                    }
                }
            }
        }
        // once all of the flashes have been triggered, NOW we can zero things
        // out
        for row in self.octopodes.iter_mut() {
            for val in row.iter_mut() {
                if *val <= 9 {
                    continue;
                }
                self.flashes += 1;
                *val = 0;
            }
        }
    }
}

#[test]
fn test_updates() {
    let lines = INPUT.lines();
    let mut grid = Grid::<10>::parse(lines).unwrap();
    for _ in 0..2 {
        grid.update();
        for row in grid.octopodes {
            for char in row {
                print!("{}", char);
            }
            println!();
        }
        println!();
    }
}

#[test]
fn part1() {
    let lines = INPUT.lines();
    let mut grid = Grid::<10>::parse(lines).unwrap();
    for _ in 0..10 {
        grid.update();
    }
    println!("Day 11, part 1: {}", grid.flashes);
}
