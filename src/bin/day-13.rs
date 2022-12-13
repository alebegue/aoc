use eval::{eval, Value};

use anyhow::Result;
use regex::Regex;

fn check_packet(left: &Value, right: &Value) -> Option<bool> {
    match (left, right) {
        (Value::Number(l), Value::Number(r)) => {
            if l.as_i64() < r.as_i64() {
                return Some(true);
            } else if l.as_i64() > r.as_i64() {
                return Some(false);
            }
        },
        (Value::Array(l), Value::Array(r)) => {
            for (i, j) in (0..l.len()).zip(0..r.len()) {
                if let Some(b) = check_packet(&l[i], &r[j]) {
                    return Some(b);
                }
            }

            if l.len() < r.len() {
                return Some(true);
            } else if l.len() > r.len() {
                return Some(false);
            }
        },
        (Value::Number(_), Value::Array(_)) => {
            let left = Value::from(vec![left.to_owned()]);
            return check_packet(&left, &right);
        },
        (Value::Array(_), Value::Number(_)) => {
            let right = Value::from(vec![right.to_owned()]);
            return check_packet(&left, &right);
        },
        _ => panic!("invalid value"),
    }

    None
}

fn sort_packets(array: &mut Vec<Value>) {
    for i in 0..array.len() {
        for j in 0..array.len() - i - 1 {
            if check_packet(&array[j], &array[j + 1]).unwrap() {
                array.swap(j, j + 1);
            }
        }
    }
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("src/bin/day-13-input.txt")?;
    let re = Regex::new(r"\[").unwrap();
    let input = re.replace_all(&input, "array(");
    let re = Regex::new(r"\]").unwrap();
    let input = re.replace_all(&input, ")");


    println!("Part 1");

    let message = input
        .split("\n\n")
        .map(|b| b.lines().collect::<Vec<_>>().iter().map(|v| eval(v).expect("issue with eval")).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut checks = Vec::new();

    message
        .iter()
        .for_each(|packet| {
            let left = &packet[0];
            let right = &packet[1];
            if let Some(true) = check_packet(left, right) {
                checks.push(true);
            } else {
                checks.push(false);
            }
        });

    let sum_indices = checks
        .iter()
        .enumerate()
        .filter(|(_, v)| **v)
        .map(|(i, _)| {
            i + 1
        })
        .sum::<usize>();
    println!("Sum of indices: {}", sum_indices);

    println!("Part 2");

    let new_input = input + "array(array(2))\narray(array(6))";

    let mut message = new_input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| eval(l).expect("issue with eval"))
        .collect::<Vec<_>>();


    sort_packets(&mut message);
    message.reverse();

    let indices = message
        .iter()
        .enumerate()
        .filter(|(_, v)| {
            *v == &eval("array(array(2))").unwrap() || *v == &eval("array(array(6))").unwrap()
        })
        .map(|(i, _)| {
            i + 1
        })
        .product::<usize>();

    println!("Product of indices: {:?}", indices);

    return Ok(());
}
