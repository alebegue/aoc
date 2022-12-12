use std::collections::HashMap;
use anyhow::Result;

#[derive(Debug)]
struct Monkey<'a> {
    id: u32,
    items: Vec<u64>,
    rule: &'a str,
    weight: &'a str,
    test: u32,
    target: Vec<u32>,
}

fn worry_level(old: u64, weight: u64, relief: u64, operation: &str) -> u64 {
    match operation {
        "+" => ((old + weight) as f64 / relief as f64).floor() as u64,
        "*" => ((old * weight) as f64 / relief as f64).floor() as u64,
        _ => panic!("Unknown operation"),
    }
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("src/bin/day-11-input.txt")?;

    println!("Part 1");

    let mut monkeys = input
        .split("\n\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|m| {
            let info = m
                .lines()
                .collect::<Vec<&str>>();

            let id = info[0]
                .split(" ")
                .collect::<Vec<&str>>()[1]
                .split(":")
                .collect::<Vec<&str>>()[0]
                .parse::<u32>()
                .unwrap();

            let items = info[1]
                .split(":")
                .collect::<Vec<&str>>()[1]
                .split(",")
                .collect::<Vec<&str>>()
                .iter()
                .map(|i| i.trim().parse::<u64>().unwrap())
                .collect::<Vec<u64>>();

            let rule = info[2]
                .split("=")
                .collect::<Vec<&str>>()[1]
                .trim()
                .split(" ")
                .collect::<Vec<&str>>();

            let weight = rule[2];
            let rule = rule[1];

            let test = info[3]
                .split("by")
                .collect::<Vec<&str>>()[1]
                .trim()
                .parse::<u32>()
                .unwrap();

            let mut target = Vec::new();

            let tmp = info[4]
                .split("monkey")
                .collect::<Vec<&str>>()[1]
                .trim()
                .parse::<u32>()
                .unwrap();

            target.push(tmp);

            let tmp = info[5]
                .split("monkey")
                .collect::<Vec<&str>>()[1]
                .trim()
                .parse::<u32>()
                .unwrap();

            target.push(tmp);

            Monkey {
                id,
                items,
                rule,
                weight,
                test,
                target,
            }
        })
        .collect::<Vec<Monkey>>();

    let mut counts = HashMap::new();

    for _ in 0..20 {
        for m in 0..monkeys.len() {
            *counts.entry(monkeys[m].id).or_insert(0) += monkeys[m].items.len() as u32;
            let mut items_moved = Vec::new();

            for item in &monkeys[m].items {
                let weight = match monkeys[m].weight {
                    "old" => *item,
                    _ => monkeys[m].weight.parse::<u64>().unwrap(),
                };
                let worry = worry_level(*item, weight, 3, monkeys[m].rule);

                let test = worry % monkeys[m].test as u64 == 0;

                let target = match test {
                    true => monkeys[m].target[0],
                    false => monkeys[m].target[1],
                };

                items_moved.push((item.clone(), worry, target));
            }

            items_moved.iter().for_each(|(item, worry, target)| {
                monkeys[m].items.retain(|i| i != item);
                monkeys[*target as usize].items.push(*worry);
            });
        }
    }

    let mut counts = counts
        .iter()
        .collect::<Vec<(&u32, &u32)>>();

    counts.sort_by_key(|c| c.1);
    counts.reverse();

    println!("counts: {:?}", counts);

    println!("Level of monkey business: {}", counts[0].1 * counts[1].1);

    println!("Part 2");

    let mut monkeys = input
        .split("\n\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|m| {
            let info = m
                .lines()
                .collect::<Vec<&str>>();

            let id = info[0]
                .split(" ")
                .collect::<Vec<&str>>()[1]
                .split(":")
                .collect::<Vec<&str>>()[0]
                .parse::<u32>()
                .unwrap();

            let items = info[1]
                .split(":")
                .collect::<Vec<&str>>()[1]
                .split(",")
                .collect::<Vec<&str>>()
                .iter()
                .map(|i| i.trim().parse::<u64>().unwrap())
                .collect::<Vec<u64>>();

            let rule = info[2]
                .split("=")
                .collect::<Vec<&str>>()[1]
                .trim()
                .split(" ")
                .collect::<Vec<&str>>();

            let weight = rule[2];
            let rule = rule[1];

            let test = info[3]
                .split("by")
                .collect::<Vec<&str>>()[1]
                .trim()
                .parse::<u32>()
                .unwrap();

            let mut target = Vec::new();

            let tmp = info[4]
                .split("monkey")
                .collect::<Vec<&str>>()[1]
                .trim()
                .parse::<u32>()
                .unwrap();

            target.push(tmp);

            let tmp = info[5]
                .split("monkey")
                .collect::<Vec<&str>>()[1]
                .trim()
                .parse::<u32>()
                .unwrap();

            target.push(tmp);

            Monkey {
                id,
                items,
                rule,
                weight,
                test,
                target,
            }
        })
        .collect::<Vec<Monkey>>();
    let mut counts = HashMap::new();

    let common_divisor = monkeys.iter().map(|m| m.test).fold(1u64, |a, b| a * b as u64);

    for _ in 0..10_000 {
        for m in 0..monkeys.len() {
            *counts.entry(monkeys[m].id as u64).or_insert(0) += monkeys[m].items.len() as u64;
            let mut items_moved = Vec::new();

            for item in &monkeys[m].items {
                let weight = match monkeys[m].weight {
                    "old" => *item,
                    _ => monkeys[m].weight.parse::<u64>().unwrap(),
                };
                let mut worry = worry_level(*item, weight, 1, monkeys[m].rule);
                worry = worry % common_divisor;

                let test = worry % monkeys[m].test as u64 == 0;

                let target = match test {
                    true => monkeys[m].target[0],
                    false => monkeys[m].target[1],
                };

                items_moved.push((item.clone(), worry, target));
            }

            items_moved.iter().for_each(|(item, worry, target)| {
                monkeys[m].items.retain(|i| i != item);
                monkeys[*target as usize].items.push(*worry);
            });
        }
    }

    let mut counts = counts
        .iter()
        .collect::<Vec<(&u64, &u64)>>();

    counts.sort_by_key(|c| c.1);
    counts.reverse();

    println!("counts: {:?}", counts);

    println!("Level of monkey business: {}", counts[0].1 * counts[1].1);

    return Ok(());
}
