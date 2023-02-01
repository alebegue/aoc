use std::collections::BTreeMap;
use anyhow::Result;
use std::time::Instant;

struct Chamber {
    num_rocks: i64,
    jets: Vec<char>,
    jet_state: usize,
    rocks: Vec<Vec<(usize, usize)>>,
    rock_state: usize,
    board: Vec<Vec<char>>,
    counts: BTreeMap<i64, Vec<usize>>,
    height: usize,
    cycle: bool,
    cycle_start: usize,
    cycle_size: usize,
}

impl Chamber {
    fn falling_rock(&mut self, max_num_rocks: i64) {
        let old_height = self.board.len();
        self.num_rocks += 1;

        let mut rock = self.rocks[self.rock_state].clone();
        self.rock_state = (self.rock_state + 1) % self.rocks.len();

        let height_rock = rock.iter().map(|(_, y)| y).max().unwrap() + 1;

        self.board.append(&mut vec![vec!['.'; self.board[0].len()]; height_rock + 3]);

        let height_tower = self.board.len();

        rock
            .iter_mut()
            .for_each(|(_, y)| *y = height_tower - 1 - *y);

        let mut stop = false;

        while !stop {
            let jet = self.jets[self.jet_state];
            self.jet_state = (self.jet_state + 1) % self.jets.len();
            match jet {
                '>' => {
                    let room = rock
                        .iter()
                        .all(|r| r.0 != 0);

                    if room {
                        let check_collision = rock
                            .iter()
                            .any(|r| self.board[r.1][r.0 - 1] == '#');
                        if !check_collision {
                            rock
                                .iter_mut()
                                .for_each(|r| r.0 -= 1);
                        }
                    }
                }
                '<' => {
                    let room = rock
                        .iter()
                        .all(|r| r.0 != 6);

                    if room {
                        let check_collision = rock
                            .iter()
                            .any(|r| self.board[r.1][r.0 + 1] == '#');
                        if !check_collision {
                            rock
                                .iter_mut()
                                .for_each(|r| r.0 += 1);
                        }
                    }
                }
                _ => panic!("Unknown jet: {}", jet),
            };

            let check_collision = rock
                .iter()
                .any(|r| self.board[r.1 - 1][r.0] == '#');

            if check_collision {
                stop = true;
            } else {
                rock
                    .iter_mut()
                    .for_each(|r| r.1 -= 1);
            }
        }

        rock
            .iter()
            .for_each(|r| self.board[r.1][r.0] = '#');

        for _ in 0..(rock.len() + 3) {
            if self.board[self.board.len() - 1].iter().all(|x| *x == '.') {
                self.board.pop();
            } else {
                break;
            }
        }

        if !self.cycle {
            let counts = self.board
                .iter()
                .map(|row| row.iter().filter(|x| **x == '#').count())
                .collect::<Vec<_>>();

            self.height = self.board.len() - 1;

            for cycle_size in 2500..(counts.len() / 2) {
                if let Some(cycle_index) = find_cycle(&counts, cycle_size) {
                    self.cycle = true;
                    self.cycle_start = cycle_index;
                    self.cycle_size = cycle_size;

                    let mut first_rock_cycle = 0;
                    for _ in 0..cycle_size {
                        if let Some(first_counts) = self.counts.get(&(first_rock_cycle as i64)) {
                            if first_counts.len() > cycle_index && first_counts[cycle_index] == counts[cycle_index] {
                                break;
                            }
                        }
                        first_rock_cycle += 1;
                    }
                    let num_rocks_cycle = (self.num_rocks - first_rock_cycle as i64) / 2;
                    self.height = self.board.len() - 1;

                    let old_num_rocks = self.num_rocks;
                    let n = (max_num_rocks - old_num_rocks) / num_rocks_cycle;
                    self.num_rocks += n * num_rocks_cycle;
                    self.height += n as usize * cycle_size;

                    self.board = self.board[(cycle_index + cycle_size)..].to_vec();

                    break;
                }
            }
            self.counts.insert(self.num_rocks, counts);
        } else {
            self.height += self.board.len() - old_height;
        }
    }
}

fn find_cycle(v: &Vec<usize>, size: usize) -> Option<usize> {
    for i in size..(v.len() - size) {
        let window = &v[i..(i + size)];
        let prev_window = &v[(i - size)..i];
        let check_if_equal = window.iter().zip(prev_window.iter()).all(|(a, b)| a == b);
        if check_if_equal {
            return Some(i - size);
        }
    }
    None
}

fn main() -> Result<()> {
    // let input = std::fs::read_to_string("src/bin/day-17-test.txt")?;
    let input = std::fs::read_to_string("src/bin/day-17-input.txt")?;

    let jets = input.chars().filter(|c| c != &'\n').collect::<Vec<_>>();

    let rocks = vec![
        vec![(1, 0), (2, 0), (3, 0), (4, 0)],
        vec![(3, 2), (2, 1), (3, 1), (4, 1), (3, 0)],
        vec![(2, 2), (3, 2), (4, 2), (2, 1), (2, 0)],
        vec![(4, 3), (4, 2), (4, 1), (4, 0)],
        vec![(3, 1), (4, 1), (3, 0), (4, 0)],
    ];

    let board = vec![vec!['#'; 7]; 1];

    let mut chamber = Chamber {
        num_rocks: 0,
        jets: jets,
        jet_state: 0,
        rocks: rocks,
        rock_state: 0,
        board: board,
        counts: BTreeMap::new(),
        height: 0,
        cycle: false,
        cycle_start: 0,
        cycle_size: 0,
    };

    println!("Part 1");

    let start_total = Instant::now();

    let max_num_rocks = 2022;

    while chamber.num_rocks < max_num_rocks {
        chamber.falling_rock(max_num_rocks);
    }

    let duration_total = start_total.elapsed();

    println!("Total time elapsed is: {:?}", duration_total);

    // println!("Board: \n{}", chamber.board.iter().map(|r| r.iter().collect::<String>()).collect::<Vec<_>>().join("\n"));
    println!("Height of the tower: {}", chamber.height);

    println!("Part 2");

    let jets = input.chars().filter(|c| c != &'\n').collect::<Vec<_>>();

    let rocks = vec![
        vec![(1, 0), (2, 0), (3, 0), (4, 0)],
        vec![(3, 2), (2, 1), (3, 1), (4, 1), (3, 0)],
        vec![(2, 2), (3, 2), (4, 2), (2, 1), (2, 0)],
        vec![(4, 3), (4, 2), (4, 1), (4, 0)],
        vec![(3, 1), (4, 1), (3, 0), (4, 0)],
    ];

    let board = vec![vec!['#'; 7]; 1];

    let mut chamber = Chamber {
        num_rocks: 0,
        jets: jets,
        jet_state: 0,
        rocks: rocks,
        rock_state: 0,
        board: board,
        counts: BTreeMap::new(),
        height: 0,
        cycle: false,
        cycle_start: 0,
        cycle_size: 0,
    };

    let start_total = Instant::now();

    let max_num_rocks = 1_000_000_000_000;

    while chamber.num_rocks < max_num_rocks {
        chamber.falling_rock(max_num_rocks);
    }

    let duration_total = start_total.elapsed();

    println!("Total time elapsed is: {:?}", duration_total);

    // println!("Board: \n{}", chamber.board.iter().map(|r| r.iter().collect::<String>()).collect::<Vec<_>>().join("\n"));
    println!("Height of the tower: {}", chamber.height);

    Ok(())
}
