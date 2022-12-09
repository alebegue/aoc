use anyhow::Result;
use std::collections::HashMap;

struct Rope {
    x: Vec<i32>,
    y: Vec<i32>,
    history: HashMap<(i32, i32), i32>,
}

impl Rope {
    fn move_one_step(&mut self, direction: char) {
        let tail_position = (self.x.last().unwrap().clone(), self.y.last().unwrap().clone());
        *self.history.entry(tail_position).or_insert(1) += 1;

        match direction {
            'U' => self.y.first_mut().map(|y| *y += 1),
            'D' => self.y.first_mut().map(|y| *y -= 1),
            'L' => self.x.first_mut().map(|x| *x -= 1),
            'R' => self.x.first_mut().map(|x| *x += 1),
            _ => panic!("Invalid direction"),
        };

        let dx = self.x.first().unwrap() - self.x.last().unwrap();
        let dy = self.y.first().unwrap() - self.y.last().unwrap();

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

        self.x.last_mut().map(|x| *x += move_x);
        self.y.last_mut().map(|y| *y += move_y);
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

    let mut rope = Rope {
        x: vec![0, 0],
        y: vec![0, 0],
        history: HashMap::new(),
    };

    actions.iter().for_each(|(action, value)| {
        for _ in 0..*value {
            rope.move_one_step(*action);
        }
    });

    println!("Number of visited points: {}", rope.history.len());

    println!("Part 2");

    return Ok(());
}
