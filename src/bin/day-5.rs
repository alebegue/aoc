fn main() {
    let input = include_str!("day-5-input.txt");

    println!("Part 1");

    let contents = input.split("\n\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.split("\n").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let raw_stacks = &contents[0]
        .iter()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let num_stacks = (raw_stacks[0].len() + 1) / 4;

    let mut stacks: Vec<Vec<char>> = vec![vec![]; num_stacks];

    for stack in raw_stacks[0..(raw_stacks.len() - 1)].iter().rev() {
        for i in 0..num_stacks {
            if stack[i * 4 + 1] != ' ' {
                stacks[i].push(stack[i * 4 + 1]);
            }
        }
    }

    let raw_moves = &contents[1]
        .iter()
        .map(|x| x.split(" ").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let mut stacks_copy = stacks.clone();

    for row in raw_moves[0..(raw_moves.len() - 1)].iter() {
        let count = row[1].parse::<usize>().unwrap();
        let from = row[3].parse::<usize>().unwrap() - 1;
        let to = row[5].parse::<usize>().unwrap() - 1;

        for _ in 0..count {
            let card = stacks_copy[from].pop().unwrap();
            stacks_copy[to].push(card);
        }
    }

    let mut top_row = String::new();
    for stack in stacks_copy.iter() {
        top_row.push(stack[stack.len() - 1]);
    }

    println!("Crates at the top of each stack: {}", top_row);

    println!("Part 2");

    let mut stacks_copy = stacks.clone();

    for row in raw_moves[0..(raw_moves.len() - 1)].iter() {
        let count = row[1].parse::<usize>().unwrap();
        let from = row[3].parse::<usize>().unwrap() - 1;
        let to = row[5].parse::<usize>().unwrap() - 1;

        let length_to_keep = stacks_copy[from].len() - count;
        let mut cards_to_move = stacks_copy[from].split_off(length_to_keep);
        stacks_copy[to].append(&mut cards_to_move);
        stacks_copy[from].truncate(length_to_keep);
    }

    let mut top_row = String::new();
    for stack in stacks_copy.iter() {
        top_row.push(stack[stack.len() - 1]);
    }

    println!("Crates at the top of each stack: {}", top_row);
}
