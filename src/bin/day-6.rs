fn main() {
    let buffer = include_str!("day-6-input.txt");

    println!("Part 1");

    let start_of_packer = buffer
        .chars()
        .collect::<Vec<char>>()
        .windows(4)
        .position(|window| {
            let mut v = window.to_vec();
            v.sort();
            v.dedup();
            v.len() == 4
        });

    println!("Start of packer: {}", start_of_packer.unwrap() + 4);

    println!("Part 2");

    let start_of_message = buffer
        .chars()
        .collect::<Vec<char>>()
        .windows(14)
        .position(|window| {
            let mut v = window.to_vec();
            v.sort();
            v.dedup();
            v.len() == 14
        });

    println!("Start of message: {}", start_of_message.unwrap() + 14);
}
