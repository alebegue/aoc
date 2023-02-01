use std::collections::HashSet;
use std::time::Instant;
use anyhow::Result;

// Get the neighbors of a given cube
fn get_neighbors(cube: &Vec<i32>, cubes: &HashSet<Vec<i32>>) -> HashSet<Vec<i32>> {
    let mut neighbors = HashSet::new();
    for other_cube in cubes {
        // Check if the other cube is a face of the current cube
        if cube.iter().zip(other_cube.iter()).filter(|(a, b)| a == b).count() == 2 &&
            cube.iter().zip(other_cube.iter()).filter(|(a, b)| (*a - *b).abs() == 1).count() == 1 {
            neighbors.insert(other_cube.clone());
        }
    }
    neighbors
}

fn explore_space(cube: &Vec<i32>, direction: &Vec<i32>, cubes: &HashSet<Vec<i32>>, connected_to_outside: &mut HashSet<Vec<i32>>) -> Option<i32> {
    let other_cube = vec![cube[0] + direction[0], cube[1] + direction[1], cube[2] + direction[2]];


    let min_x = cubes.iter().map(|c| c[0]).min().unwrap();
    let max_x = cubes.iter().map(|c| c[0]).max().unwrap();
    let min_y = cubes.iter().map(|c| c[1]).min().unwrap();
    let max_y = cubes.iter().map(|c| c[1]).max().unwrap();
    let min_z = cubes.iter().map(|c| c[2]).min().unwrap();
    let max_z = cubes.iter().map(|c| c[2]).max().unwrap();


    let mut visited = HashSet::new();
    let mut queue = HashSet::new();
    queue.insert(other_cube.clone());

    while !queue.is_empty() {
        let current = queue.iter().next().unwrap().clone();
        queue.remove(&current);
        visited.insert(current.clone());
        for x in -1..=1 {
            for y in -1..=1 {
                for z in -1..=1 {
                    if ((x + y + z) as i32).abs() == 1 && (x == 0 || y == 0 || z == 0) {
                        let other_cube = vec![current[0] + x, current[1] + y, current[2] + z];
                        if !queue.contains(&other_cube) && !visited.contains(&other_cube) && !cubes.contains(&other_cube) {
                            if other_cube[0] < min_x || other_cube[0] > max_x ||
                                other_cube[1] < min_y || other_cube[1] > max_y ||
                                    other_cube[2] < min_z || other_cube[2] > max_z {
                                        visited.iter().for_each(|c| { connected_to_outside.insert(c.clone()); });
                                        return Some(1);
                                    }
                            queue.insert(other_cube.clone());
                        }
                    }
                }
            }
        }
    }

    None
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("src/bin/day-18-input.txt")?;
    // let input = std::fs::read_to_string("src/bin/day-18-test.txt")?;

    let cubes = input
        .lines()
        .map(|l| l.split(",")
             .map(|n| n.parse::<i32>().unwrap())
             .collect::<Vec<_>>())
        .collect::<HashSet<_>>();

    println!("Part 1");

    let start_total = Instant::now();

    let num_cubes = cubes.len();
    let mut exposed_faces = num_cubes * 6;

    for cube in &cubes {
        let neighbors = get_neighbors(cube, &cubes);
        exposed_faces -= neighbors.len();
    }

    println!("Number of exposed faces: {}", exposed_faces);

    let duration_total = start_total.elapsed();
    println!("Total time elapsed is: {:?}", duration_total);

    println!("Part 2");

    let start_total = Instant::now();

    let mut exposed_faces = 0;
    let mut connected_to_outside: HashSet<Vec<i32>> = HashSet::new();

    for cube in &cubes {
        for x in -1..=1 {
            for y in -1..=1 {
                for z in -1..=1 {
                    if ((x + y + z) as i32).abs() == 1 && (x == 0 || y == 0 || z == 0) {
                        let other_cube = vec![cube[0] + x, cube[1] + y, cube[2] + z];
                        if connected_to_outside.contains(&other_cube) && !cubes.contains(&other_cube) {
                            exposed_faces += 1;
                            continue;
                        }
                        if !cubes.contains(&other_cube) {
                            if let Some(_) = explore_space(&cube, &vec![x, y, z], &cubes, &mut connected_to_outside) {
                                exposed_faces += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    println!("Number of exposed faces: {}", exposed_faces);

    let duration_total = start_total.elapsed();
    println!("Total time elapsed is: {:?}", duration_total);

    Ok(())
}
