use std::collections::{HashMap, HashSet};
use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    // let input = std::fs::read_to_string("src/bin/day-16-test.txt")?;
    let input = std::fs::read_to_string("src/bin/day-16-input.txt")?;

    let (f, i, t) = parse_input(&input);

    println!("Part 1");

    let mut released_pressure = HashMap::new();
    visit("AA", 30, 0, 0, &mut released_pressure, &f, &i, &t);
    println!("Most pressure released: {:?}", released_pressure.iter().map(|(_, x)| x).max().unwrap());

    println!("Part 2");

    let mut released_pressure = HashMap::new();
    visit("AA", 26, 0, 0, &mut released_pressure, &f, &i, &t);
    let max_released_pressure = released_pressure
        .iter()
        .tuple_combinations()
        .map(|((a, b), (c, d))| if a & c == 0 { b + d } else { 0 })
        .max()
        .unwrap();
    println!("Most pressure released: {:?}", max_released_pressure);

    Ok(())
}

fn visit(
    v: &str,
    budget: i32,
    state: i32,
    flow: i32,
    answer: &mut HashMap<i32, i32>,
    f: &HashMap<&str, i32>,
    i: &HashMap<&str, i32>,
    t: &HashMap<&str, HashMap<&str, i32>>,
) {
    // *answer.entry(state).or_insert(0) = *answer.get(&state).map(|x| x.max(&flow)).unwrap_or(&flow);
    *answer.entry(state).or_insert(0) = answer.get(&state).unwrap_or(&0).max(&flow).clone();
    for u in f.keys() {
        let newbudget = budget - t[v][u] - 1;
        if (i[u] & state) != 0 || newbudget <= 0 {
            continue;
        }
        visit(u, newbudget, state | i[u], flow + newbudget * f[u], answer, f, i, t);
    }
}

fn parse_input(input: &String) -> (HashMap<&str, i32>, HashMap<&str, i32>, HashMap<&str, HashMap<&str, i32>>) {
    let lines = input
        .lines()
        .map(|l| l.split(|c| c == ' ' || c == '=' || c == ';' || c == ',').filter(|c| !c.is_empty()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut g: HashMap<&str, HashSet<&str>> = HashMap::new();
    let mut f: HashMap<&str, i32> = HashMap::new();
    let mut i: HashMap<&str, i32> = HashMap::new();
    let mut t: HashMap<&str, HashMap<&str, i32>> = HashMap::new();

    for x in lines {
        g.insert(x[1], x[10..].iter().map(|x| *x).collect());
        if x[5] != "0" {
            f.insert(x[1], x[5].parse::<i32>().unwrap());
        }
    }

    // Bitmask for each node
    for (index, x) in f.keys().enumerate() {
        i.insert(x, 1 << index);
    }

    // Floyd–Warshall algorithm

    for x in g.keys() {
        let mut inner: HashMap<&str, i32> = HashMap::new();
        for y in g.keys() {
            inner.insert(y, if g[x].contains(y) { 1 } else { std::i32::MAX });
        }
        t.insert(x, inner);
    }

    for k in t.keys().cloned().collect::<Vec<_>>() {
        for i in t.keys().cloned().collect::<Vec<_>>() {
            for j in t.keys().cloned().collect::<Vec<_>>() {
                if t[i][k] < std::i32::MAX && t[k][j] < std::i32::MAX {
                    let new = t[i][j].min(t[i][k] + t[k][j]);
                    t.get_mut(i).unwrap().insert(j, new);
                }
            }
        }
    }

    (f, i, t)
}
