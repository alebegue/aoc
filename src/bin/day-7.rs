use std::collections::HashMap;

#[derive(Debug)]
struct Content {
    folders: HashMap<String, Content>,
    files: HashMap<String, u64>,
    total_size: u64,
}

fn parse_lines(content: &mut Content, lines: &mut Vec<&str>) {
    while !lines.is_empty() {
        let l = lines[0];
        lines.remove(0);

        if l.contains("$ cd") {
            let folder = l.split("$ cd ").collect::<Vec<&str>>()[1];
            if folder == ".." {
                return;
            } else {
                let mut new_content = Content {
                    folders: HashMap::new(),
                    files: HashMap::new(),
                    total_size: 0,
                };
                parse_lines(&mut new_content, lines);
                content.folders.insert(folder.to_string(), new_content);
            }
        } else {
            while !lines.is_empty() && !lines[0].contains("$ cd") {
                let parts = lines[0].split(" ").collect::<Vec<&str>>();
                let size = parts[0];
                let file = parts[1];
                if !size.contains("dir") {
                    content.files.insert(file.to_string(), size.parse::<u64>().unwrap());
                }
                lines.remove(0);
            }
        }
    }
}

fn compute_size(content: &mut Content) {
    for (_, folder) in content.folders.iter_mut() {
        compute_size(folder);
        content.total_size += folder.total_size;
    }
    for (_, size) in content.files.iter() {
        content.total_size += size;
    }
}

fn compute_sum_size(content: &mut Content) -> u64 {
    let mut sum = 0;
    for (_, folder) in content.folders.iter_mut() {
        sum += compute_sum_size(folder);
    }
    if content.total_size < 100000 {
        sum += content.total_size;
    }
    sum
}

fn get_sizes(content: &mut Content, sizes: &mut Vec<u64>) {
    for (_, folder) in content.folders.iter_mut() {
        get_sizes(folder, sizes);
    }
    sizes.push(content.total_size);
}

fn main() {
    let input = include_str!("day-7-input.txt");

    println!("Part 1");
    
    let mut lines = input
        .lines()
        .collect::<Vec<&str>>();

    let mut content = Content {
        folders: HashMap::new(),
        files: HashMap::new(),
        total_size: 0,
    };

    parse_lines(&mut content, &mut lines);

    compute_size(&mut content);

    println!("Total size of directories < 100000: {}", compute_sum_size(&mut content));

    println!("Part 2");

    let target = 30000000 - (70000000 - content.total_size);

    let mut sizes = Vec::new();
    get_sizes(&mut content, &mut sizes);

    let sol = sizes
        .iter()
        .filter(|&s| *s > target)
        .min()
        .unwrap();

    println!("Total size of directory to delete: {}", sol);
}
