use std::cmp::Ordering;

fn get_total_calories(contents: &str) -> Vec<u32> {
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
    // Load the input file
    let contents = include_str!("day-1-input.txt");

    // Get the total calories for each elf
    let mut total_calories = get_total_calories(&contents);

    // Find the max calories
    println!("Elf with most calories: {}", find_max_calories(&total_calories));

    // Sort the total calories
    total_calories.sort();
    total_calories.reverse();

    // Find the max calories (again)
    println!("Elf with most calories: {}", total_calories[0]);

    // Sum the first 3 elves
    println!("Top 3 elves: {}", total_calories[..3].iter().sum::<u32>());
}
