use anyhow::Result;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Node {
    state: (usize, usize),
    parent: (usize, usize),
}

fn find_neighbours(grid: &Vec<Vec<i32>>, node: &Node) -> Vec<Node> {
    let mut neighbours = Vec::new();
    let row = node.state.0;
    let col = node.state.1;

    if row > 0 && grid[row - 1][col] - grid[row][col] <= 1 {
        neighbours.push(Node {
            state: (row - 1, col),
            parent: (row, col),
        });
    }

    if row < grid.len() - 1 && grid[row + 1][col] - grid[row][col] <= 1 {
        neighbours.push(Node {
            state: (row + 1, col),
            parent: (row, col),
        });
    }

    if col > 0 && grid[row][col - 1] - grid[row][col] <= 1 {
        neighbours.push(Node {
            state: (row, col - 1),
            parent: (row, col),
        });
    }

    if col < grid[0].len() - 1 && grid[row][col + 1] - grid[row][col] <= 1 {
        neighbours.push(Node {
            state: (row, col + 1),
            parent: (row, col),
        });
    }

    neighbours
}

fn find_path(grid: &Vec<Vec<i32>>, start: (usize, usize), end: (usize, usize)) -> Vec<(usize, usize)> {
    let mut path = vec![];
    let mut frontier = HashSet::new();
    let mut visited = HashMap::new();

    frontier.insert(Node {
        state: start,
        parent: start,
    });

    while !frontier.is_empty() {
        let mut next_frontier: HashSet<Node> = HashSet::new();
        let mut old_frontier = HashSet::new();
        frontier.iter().for_each(|n| {
            old_frontier.insert(n.clone());
        });

        for node in old_frontier {
            // println!("node: {:?}", node);
            if node.state == end {
                path.push(node.state);

                let mut n = node;
                while n.parent != start {
                    path.push(n.parent);
                    n = visited[&n.parent];
                }
                path.push(start);
                return path;
            }

            if !visited.contains_key(&node.state) {
                visited.insert(node.state, node);
                next_frontier.extend(find_neighbours(grid, &node).iter().filter(|n| {
                    !visited.contains_key(&n.state) && !frontier.contains(&n)
                }));
            }
        }
        frontier = next_frontier;
    }

    path
}

fn main() -> Result<()> {
    // let path = "src/bin/day-12-test.txt";
    let path = "src/bin/day-12-input.txt";
    let mut grid = load_grid(path)?;

    println!("Part 1");

    let start = find_position(&grid, 83);
    grid[start.0][start.1] = 97;

    let end = find_position(&grid, 69);
    grid[end.0][end.1] = 122;

    let optimal_path = find_path(&grid, start, end);

    println!("Optimal path length: {}", optimal_path.len() - 1);

    println!("Part 2");

    let mut starts = Vec::new();
    let mut optimal_pathes = Vec::new();

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 97 {
                starts.push((i, j));
                let optimal_path = find_path(&grid, (i, j), end).len() as i32 - 1;
                optimal_pathes.push(optimal_path);
            }
        }
    }

    println!("Optimal path length: {}", optimal_pathes.iter().filter(|&l| l > &0).min().unwrap());

    return Ok(());
}

fn load_grid(path: &str) -> Result<Vec<Vec<i32>>> {
    let input = std::fs::read_to_string(path)?;

    let grid = input
        .lines()
        .map(|line| {
            line
                .chars()
                .map(|c| c as i32)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    Ok(grid)
}

fn find_position(grid: &Vec<Vec<i32>>, value: i32) -> (usize, usize) {
    let start_row = grid
        .iter()
        .position(|row| row.contains(&value))
        .unwrap();

    let start_col = grid[start_row]
        .iter()
        .position(|&c| c == value)
        .unwrap();

    (start_row, start_col)
}
