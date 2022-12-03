fn main () {
    let contents = include_str! ("day-3-input.txt");

    println!("Part 1");

    let mut properties_sum = 0;
    for line in contents.lines() {
        let length = line.len() / 2;
        for x in line[..length].chars() {
            if line[length..].chars().any(|y| x == y) {
                if x as u8 >= 97 && x as u8 <= 122 {
                    properties_sum += x as u32 - 96;
                    break;
                } else {
                    properties_sum += x as u32 - 38;
                    break;
                }
            }
        }
    }
    println! ("Sum of properties: {}", properties_sum);

    println!("Part 2");

    properties_sum = 0;

    contents.lines().collect::<Vec<&str>>().chunks(3).for_each(|chunk| {
        for c in chunk[0].chars() {
            if chunk[1].chars().any(|x| x == c) && chunk[2].chars().any(|x| x == c) {
                if c as u8 >= 97 && c as u8 <= 122 {
                    properties_sum += c as u32 - 96;
                    break;
                } else {
                    properties_sum += c as u32 - 38;
                    break;
                }
            }
        }
    });
    println!("Sum of properties: {}", properties_sum);
}
