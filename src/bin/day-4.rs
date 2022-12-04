fn main() {
    let contents = include_str!("day-4-input.txt");

    println!("Part 1");
    let mut count = 0;
    for line in contents.lines() {
        let bounds = line.split(",")
            .map(|x| x.split("-").collect::<Vec<&str>>())
            .map(|x| (x[0].parse::<u32>().unwrap(), x[1].parse::<u32>().unwrap()))
            .collect::<Vec<_>>();
        if ((bounds[0].0 >= bounds[1].0) & (bounds[0].1 <= bounds[1].1)) | ((bounds[0].0 <= bounds[1].0) & (bounds[0].1 >= bounds[1].1)) {
            count += 1;
        }
    }
    println!("{}", count);

    println!("Part 2");
    let mut count = 0;
    for line in contents.lines() {
        let bounds = line.split(",")
            .map(|x| x.split("-").collect::<Vec<&str>>())
            .map(|x| (x[0].parse::<u32>().unwrap(), x[1].parse::<u32>().unwrap()))
            .collect::<Vec<_>>();
        if ((bounds[0].0 >= bounds[1].0) & (bounds[0].0 <= bounds[1].1))
            | ((bounds[0].1 >= bounds[1].0) & (bounds[0].1 <= bounds[1].1))
            | ((bounds[1].0 >= bounds[0].0) & (bounds[1].0 <= bounds[0].1))
            | ((bounds[1].1 >= bounds[0].0) & (bounds[1].1 <= bounds[0].1)) {
            count += 1;
        }
    }
    println!("{}", count);
}
