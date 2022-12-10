use anyhow::Result;

#[derive(Debug)]
struct Register {
    x: Vec<i32>,
    cycle: Vec<i32>,
}

impl Register {
    fn new() -> Self {
        Register { x: vec![1], cycle: vec![0] }
    }

    fn addx(&mut self, v: i32) {
        self.x.push(self.x.last().unwrap().clone());
        self.cycle.push(self.cycle.last().unwrap().clone() + 1);
        let new_value = self.x.last().unwrap().clone() + v;
        self.x.push(new_value);
        self.cycle.push(self.cycle.last().unwrap().clone() + 1);
    }

    fn noop(&mut self) {
        self.x.push(self.x.last().unwrap().clone());
        self.cycle.push(self.cycle.last().unwrap().clone() + 1);
    }
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("src/bin/day-10-input.txt")?;

    println!("Part 1");

    let mut register = Register::new();

    input
        .lines()
        .map(|l| l.split_whitespace().collect::<Vec<_>>())
        .collect::<Vec<_>>()
        .iter()
        .for_each(|s| {
            if s.len() == 1 {
                register.noop();
            } else {
                let value = s[1].parse().unwrap();
                register.addx(value);
            }
        });

    let mut signal = Vec::new();

    for c in vec![20, 60, 100, 140, 180, 220] {
        let index = register.cycle.iter().position(|&x| x == c).unwrap();
        signal.push(c * register.x[index - 1]);
    }

    println!("Sum of signal: {}", signal.iter().sum::<i32>());

    return Ok(());
}
