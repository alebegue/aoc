use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug)]
struct Rope {
    x: Vec<i32>,
    y: Vec<i32>,
    history: HashMap<Vec<i32>, i32>,
}

impl Rope {
    fn new(length: usize) -> Self {
        Rope {
            x: vec![0; length],
            y: vec![0; length],
            history: HashMap::from([(vec![0, 0], 1)]),
        }
    }

    fn update_history(&mut self) {
        *self.history.entry(vec![self.x.last().unwrap().clone(), self.y.last().unwrap().clone()]).or_insert(1) += 1;
    }

    fn move_one_step(&mut self, direction: char) {
        match direction {
            'U' => self.y[0] += 1,
            'D' => self.y[0] -= 1,
            'L' => self.x[0] -= 1,
            'R' => self.x[0] += 1,
            _ => panic!("Invalid direction"),
        }

        for i in 0..(self.x.len() - 1) {
            let x = &mut self.x[i..(i + 2)];
            let y = &mut self.y[i..(i + 2)];

            let dx = x.first().unwrap() - x.last().unwrap();
            let dy = y.first().unwrap() - y.last().unwrap();

            let mut move_x = 0;
            let mut move_y = 0;

            if dx.abs() > 1 && dy == 0 {
                move_x = dx - dx.signum();
            } else if dy.abs() > 1 && dx == 0 {
                move_y = dy - dy.signum();
            } else if dx.abs() > 1 {
                move_x = dx - dx.signum();
                move_y = dy.signum();
            } else if dy.abs() > 1 {
                move_x = dx.signum();
                move_y = dy - dy.signum();
            }

            x.last_mut().map(|x| *x += move_x);
            y.last_mut().map(|y| *y += move_y);
        }
        self.update_history();
    }
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("src/bin/day-9-input.txt")?;

    let actions = input
        .lines()
        .collect::<Vec<_>>()
        .iter()
        .map(|line| {
            let mut c = line.chars();
            let action = c.next().unwrap();
            let value = c.collect::<String>().trim().parse::<i32>().unwrap();
            (action, value)
        })
        .collect::<Vec<_>>();

    println!("Part 1");

    let num_knots = 2;

    let mut rope = Rope::new(num_knots);

    actions.iter().for_each(|(action, value)| {
        for _ in 0..*value {
            rope.move_one_step(*action);
        }
    });

    println!("Number of visited points: {}", rope.history.len());

    println!("Part 2");

    let num_knots = 10;

    let mut rope = Rope::new(num_knots);

    actions.iter().for_each(|(action, value)| {
        for _ in 0..*value {
            rope.move_one_step(*action);
        }
    });

    println!("Number of visited points: {}", rope.history.len());

    return Ok(());
}
