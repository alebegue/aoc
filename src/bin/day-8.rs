use anyhow::Result;

fn main() -> Result<()> {
    let grid = std::fs::read_to_string("src/bin/day-8-input.txt")?;
    println!("Part 1");

    let grid = grid
        .lines()
        .map(|line| line
             .chars()
             .map(|c| c.to_digit(10).unwrap())
             .collect::<Vec<u32>>()
        )
        .collect::<Vec<_>>();

    println!("Num rows: {}", grid.len());
    println!("Num cols: {}", grid[0].len());

    let mut visible_trees = Vec::new();

    for i in 1..(grid.len() - 1) {
        visible_trees.push(Vec::new());
        for j in 1..(grid[0].len() - 1) {
            let max_left = grid[i][0..j].iter().max().unwrap();
            let max_right = grid[i][(j + 1)..].iter().max().unwrap();
            let max_top = grid[0..i].iter().flat_map(|v| v.get(j)).max().unwrap();
            let max_bottom = grid[(i + 1)..].iter().flat_map(|v| v.get(j)).max().unwrap();

            if grid[i][j] > *max_left || grid[i][j] > *max_right || grid[i][j] > *max_top || grid[i][j] > *max_bottom {
                visible_trees[i - 1].push(j);
            }
        }
    }

    let mut  num_visible_trees = visible_trees
        .iter()
        .map(|v| v.len())
        .sum::<usize>();

    num_visible_trees += grid[0].len() * 2 + grid.len() * 2 - 4;

    println!("Num visible trees: {}", num_visible_trees);

    println!("Part 2");

    let mut scenic_score = Vec::new();
    
    for i in 1..(grid.len() - 1) {
        scenic_score.push(Vec::new());
        for j in 1..(grid[0].len() - 1) {
            let num_trees_left = grid[i][0..j].iter().rev().position(|&x| x >= grid[i][j]).unwrap_or(j - 1) + 1;
            let num_trees_right = grid[i][(j + 1)..].iter().position(|&x| x >= grid[i][j]).unwrap_or(grid[0].len() - j - 2) + 1;
            let num_trees_top = grid[0..i].iter().rev().flat_map(|v| v.get(j)).position(|&x| x >= grid[i][j]).unwrap_or(i - 1) + 1;
            let num_trees_bottom = grid[(i + 1)..].iter().flat_map(|v| v.get(j)).position(|&x| x >= grid[i][j]).unwrap_or(grid.len() - i - 2) + 1;
            scenic_score[i - 1].push(num_trees_left * num_trees_right * num_trees_top * num_trees_bottom);
        }
    }

    let max_scenic_score = scenic_score
        .iter()
        .flat_map(|v| v.iter())
        .max()
        .unwrap();

    println!("Max scenic score: {}", max_scenic_score);

    return Ok(());
}
