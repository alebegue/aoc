use anyhow::Result;
use regex::Regex;

fn main() -> Result<()> {
    // let input = std::fs::read_to_string("src/bin/day-15-test.txt")?;
    let input = std::fs::read_to_string("src/bin/day-15-input.txt")?;

    println!("Part 1");

    // let line = 10;
    let line = 2_000_000;

    let (sensors, beacons, distances) = parse_input(&input);

    let ((sensors, beacons), distances): ((Vec<(i32, i32)>, Vec<(i32, i32)>), Vec<i32>) = sensors
        .iter()
        .zip(beacons.iter())
        .zip(distances.iter())
        .filter(|((sensor, _), &distance)| {
            (sensor.1 - line).abs() <= distance
        })
        .unzip();

    let min_x = sensors.iter().map(|(x, _)| x).min().unwrap().to_owned();
    let max_x = sensors.iter().map(|(x, _)| x).max().unwrap().to_owned();
    let max_distance = distances.iter().max().unwrap().to_owned();

    let mut count = 0;
    let mut row = vec!['.'; (max_x - min_x + 1 + 2 * max_distance) as usize];

    sensors
        .iter()
        .zip(beacons.iter())
        .zip(distances.iter())
        .for_each(|((sensor, beacon), distance)| {
            let sensor = (sensor.0 - min_x + max_distance, sensor.1);
            let beacon = (beacon.0 - min_x + max_distance, beacon.1);

            if sensor.1 == line {
                row[sensor.0 as usize] = 'S';
            }

            if beacon.1 == line {
                row[beacon.0 as usize] = 'B';
            }

            for x in 0..row.len() {
                if compute_manhattan_distance(sensor, (x as i32, line)) <= *distance && row[x as usize] == '.' {
                    row[x as usize] = '#';
                    count += 1;
                }
            }
        });

    println!("Number of positions that cannot contain a beacon: {}", count);

    println!("Part 2");

    let lb = 0;
    // let ub = 20;
    let ub = 4_000_000;

    let (sensors, _, distances) = parse_input(&input);

    let mut y = lb;
    let position = 'outer_y: loop {
        let mut x = lb;
        'outer_x: loop {
            let mut under_sensor = false;
            for (sensor, distance) in sensors.iter().zip(distances.iter()) {
                let d = compute_manhattan_distance(*sensor, (x, y));
                if  d <= *distance {
                    if x < sensor.0 {
                        x += 2 * (sensor.0 - x) + 1;
                    } else {
                        x += distance - d + 1;
                    }
                    under_sensor = true;
                    break;
                }
            }

            if x > ub {
                break 'outer_x;
            }

            if !under_sensor {
                break 'outer_y Some((x, y));
            }
        }
        y += 1;
        if y > ub {
            break 'outer_y None;
        }
    };

    println!("Position: {:?}", position);
    println!("Tuning frequency: {}", position.unwrap().0 as i64 * 4_000_000 + position.unwrap().1 as i64);

    Ok(())
}

fn compute_manhattan_distance((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> i32 {
    (x1 - x2).abs() + (y1 - y2).abs()
}

fn parse_input(input: &str) -> (Vec<(i32, i32)>, Vec<(i32, i32)>, Vec<i32>) {
    let mut sensors = Vec::new();
    let mut beacons = Vec::new();

    let re = Regex::new(r"x=(-?\d*), y=(-?\d*).*x=(-?\d*), y=(-?\d*)").unwrap();

    re.captures_iter(&input)
        .map(|cap| {
            (
                cap[1].parse::<i32>().unwrap(),
                cap[2].parse::<i32>().unwrap(),
                cap[3].parse::<i32>().unwrap(),
                cap[4].parse::<i32>().unwrap(),
            )
        })
        .for_each(|(x1, y1, x2, y2)| {
            sensors.push((x1, y1));
            beacons.push((x2, y2));
        });

    let distances = sensors
        .iter()
        .zip(beacons.iter())
        .map(|((x1, y1), (x2, y2))| compute_manhattan_distance((*x1, *y1), (*x2, *y2)))
        .collect::<Vec<_>>();

    (sensors, beacons, distances)
}
