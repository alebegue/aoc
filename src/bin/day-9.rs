use anyhow::Result;
use std::collections::HashMap;

#[derive(Eq, Hash, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

struct Rope {
    head: Position,
    tail: Position,
    history: HashMap<Position, i32>,
}

impl Rope {
    fn move_one_step(&mut self, direction: char) {
        let tail_position = Position {
            x: self.tail.x,
            y: self.tail.y,
        };

        *self.history.entry(tail_position).or_insert(1) += 1;

        match direction {
            'U' => self.head.y += 1,
            'D' => self.head.y -= 1,
            'L' => self.head.x -= 1,
            'R' => self.head.x += 1,
            _ => panic!("Invalid direction"),
        }

        let dx = self.head.x - self.tail.x;
        let dy = self.head.y - self.tail.y;

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

        self.tail.x += move_x;
        self.tail.y += move_y;
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
        head: Position { x: 0, y: 0 },
        tail: Position { x: 0, y: 0 },
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
