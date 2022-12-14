use anyhow::Result;

struct Sand<'a> {
    x: usize,
    y: usize,
    cave: &'a mut Vec<Vec<char>>,
}

impl <'a> Sand<'a> {
    fn new(x: usize, y: usize, cave: &'a mut Vec<Vec<char>>) -> Self {
        Self { x, y, cave }
    }

    fn move_down(&mut self) {
        self.y += 1;
    }

    fn move_left(&mut self) {
        self.x -= 1;
        self.y += 1;
    }

    fn move_right(&mut self) {
        self.x += 1;
        self.y += 1;
    }

    fn fall(&mut self) -> Option<()> {
        if self.y == self.cave.len() - 1 {
            return None;
        } else if self.cave[self.y + 1][self.x] == '.' {
            self.move_down();
            return self.fall();
        } else if self.x == 0 {
            return None;
        } else if self.cave[self.y + 1][self.x - 1] == '.' {
            self.move_left();
            return self.fall();
        } else if self.x == self.cave[0].len() - 1 {
            return None;
        } else if self.cave[self.y + 1][self.x + 1] == '.' {
            self.move_right();
            return self.fall();
        } else if self.cave[self.y][self.x] == 'o' {
            return None;
        }

        self.cave[self.y][self.x] = 'o';
        Some(())
    }
}

fn fill_of_sand(cave: &mut Vec<Vec<char>>) -> usize {
    let source_sand = cave[0].iter().position(|&x| x == '+').unwrap();
    let mut num_unit_sand = 0;
    let mut abyss = false;

    while !abyss {
        let mut sand = Sand::new(source_sand, 0, cave);

        match sand.fall() {
            Some(_) => num_unit_sand += 1,
            None => abyss = true,
        }
    }

    let screen = cave
        .iter()
        .map(|vec| vec.iter().collect::<String>())
        .collect::<Vec<_>>();

    println!("\n{}\n", screen.join("\n"));

    num_unit_sand
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("src/bin/day-14-test.txt")?;
    // let input = std::fs::read_to_string("src/bin/day-13-input.txt")?;

    println!("Part 1");

    // Parse input
    let scan = parse_input(&input)?;

    // Build cave
    let floor = false;
    let mut cave = build_cave(&scan, floor);

    // Fill of sand
    let num_unit_sand = fill_of_sand(&mut cave);

    // Print result
    println!("Number of unit of sand: {}\n", num_unit_sand);

    println!("Part 2");

    // Build cave
    let floor = true;
    let mut cave = build_cave(&scan, floor);

    // Fill of sand
    let num_unit_sand = fill_of_sand(&mut cave);

    // Print result
    println!("Number of unit of sand: {}", num_unit_sand);

    Ok(())
}

fn parse_input(input: &String) -> Result<Vec<Vec<(usize, usize)>>> {
    Ok(input
        .lines()
        .map(|line| {
            line
                .split(" -> ")
                .map(|position| {
                    let coord = position
                        .split(",")
                        .map(|c| c.trim().parse::<usize>().expect("invalid coord"))
                        .collect::<Vec<_>>();
                    (coord[0], coord[1])
                })
            .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>())
}

fn build_cave(scan: &Vec<Vec<(usize, usize)>>, floor: bool) -> Vec<Vec<char>> {
    let min_x = scan
        .iter()
        .flat_map(|vec| vec.iter().map(|x| x.0))
        .min()
        .unwrap()
        .to_owned();

    let max_y = scan
        .iter()
        .flat_map(|vec| vec.iter().map(|x| x.1))
        .max()
        .unwrap()
        .to_owned();

    let mut cave = vec![vec!['.'; 2 * (max_y + 3) + 1]; max_y + 3];

    let delta = max_y + 3 - (500 - min_x);
    // Draw rocks
    scan
        .iter()
        .for_each(|path| {
            let mut start = (path[0].0 - min_x + delta, path[0].1);
            for i in 1..path.len() {
                let end = (path[i].0 - min_x + delta, path[i].1);
                if start.0 == end.0 {
                    let min = start.1.min(end.1);
                    let max = start.1.max(end.1);
                    for j in min..=max {
                        cave[j][start.0] = '#';
                    }
                } else {
                    let min = start.0.min(end.0);
                    let max = start.0.max(end.0);
                    for j in min..=max {
                        cave[start.1][j] = '#';
                    }
                }
                start = end;
            }
        });

    // Source of sand
    cave[0][max_y + 3] = '+';

    // Add floor for part 2
    let bottom = cave.len() - 1;
    if floor {
        cave[bottom].iter_mut().for_each(|x| *x = '#');
    }

    cave
}
