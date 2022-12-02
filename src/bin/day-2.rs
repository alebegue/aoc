fn main() {
    // Load the input file
    let contents = include_str!("day-2-input.txt");

    // Part 1

    // Parse the input file
    let mut rounds = Vec::new();
    for line in contents.lines() {
        let mut iter = line.split_whitespace();
        // Get what the opponent played
        let o = iter.next().unwrap();

        // Get what we played
        let p = iter.next().unwrap();

        // Based on what we played, determine the ordering of the strengths and the index of our play
        let (v, p) = match p {
            "X" => (vec![2, 3, 1], 0),
            "Y" => (vec![1, 2, 3], 1),
            "Z" => (vec![3, 1, 2], 2),
            _ => panic!("Invalid play"),
        };
        
        // Based on what the opponent played, get the index of the ordering of the strengths
        let o = match o {
            "A" => 0,
            "B" => 1,
            "C" => 2,
            _ => panic!("Invalid play"),
        };
        
        let score;
        let score_play = p as i32;
        if v[o] < v[p] {
            score = 6 + score_play + 1;
        } else if v[o] > v[p] {
            score = 0 + score_play + 1;
        } else {
            score = 3 + score_play + 1;
        }
        rounds.push(score);
    }

    // Number of rounds
    println!("Number of rounds: {:?}", rounds.len());

    // Count the total number of points
    println!("Total points: {}", rounds.iter().sum::<i32>());

    // Part 2

    // Parse the input file
    let mut rounds = Vec::new();
    for line in contents.lines() {
        let mut iter = line.split_whitespace();
        // Get what the opponent played
        let o = iter.next().unwrap();

        let (v, o) = match o {
            "A" => (vec![2, 3, 1], 0),
            "B" => (vec![1, 2, 3], 1),
            "C" => (vec![3, 1, 2], 2),
            _ => panic!("Invalid play"),
        };

        // Get what we played
        let p = iter.next().unwrap();

        let p = match p {
            "X" => v.iter().position(|&x| x == 1).unwrap(),
            "Y" => o,
            "Z" => v.iter().position(|&x| x == 3).unwrap(),
            _ => panic!("Invalid play"),
        };
        
        let score;
        let score_play = p as i32;
        if v[o] < v[p] {
            score = 6 + score_play + 1;
        } else if v[o] > v[p] {
            score = 0 + score_play + 1;
        } else {
            score = 3 + score_play + 1;
        }
        rounds.push(score);
    }

    // Number of rounds
    println!("Number of rounds: {:?}", rounds.len());

    // Count the total number of points
    println!("Total points: {}", rounds.iter().sum::<i32>());
}
