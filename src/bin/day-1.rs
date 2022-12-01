use std::cmp::Ordering;
use std::fs;

fn get_total_calories(path: &str) -> Vec<u32> {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
    let mut total_calories: Vec<u32> = Vec::new();
    let mut calories = 0;
    for line in contents.lines() {
        match line.trim().parse::<u32>() {
            Ok(num) => calories += num,
            Err(_) => {
                total_calories.push(calories);
                calories = 0;
            }
        };
    }
    total_calories
}

fn find_max_calories(total_calories: &Vec<u32>) -> u32 {
    let mut max_calories = 0;
    for calories in total_calories {
        match calories.cmp(&max_calories) {
            Ordering::Greater => max_calories = *calories,
            _ => (),
        }
    }
    max_calories
}

fn main() {
    let path = "src/bin/day-1-input.txt";
    let mut total_calories = get_total_calories(path);
    println!("Elf with most calories: {}", find_max_calories(&total_calories));
    total_calories.sort();
    total_calories.reverse();
    println!("Elf with most calories: {}", total_calories[0]);
    println!("Top 3 elves: {}", total_calories[..3].iter().sum::<u32>());
}
