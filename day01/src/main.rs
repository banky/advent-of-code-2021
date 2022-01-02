use std::fs;

fn main() {
    let filename = "src/input.txt";

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines = contents.lines();

    // part_1(lines);
    part_2(lines);
}

fn part_1(lines: std::str::Lines) {
    let mut num_increases = -1; // Prevent counting first
    let mut prev_maximum = 0;

    for line in lines {
        let line: u32 = line.trim().parse().expect("Could not parse line");

        if prev_maximum < line {
            num_increases += 1;
        }

        prev_maximum = line;
    }

    println!("{}", format!("Number of increases: {}", num_increases));
}

fn part_2(lines: std::str::Lines) {
    fn window_sum(window: &Vec<u32>) -> u32 {
        window.iter().sum()
    }

    let mut num_increases = -1; // Prevent counting first
    let mut prev_maximum = 0;

    let mut window: Vec<u32> = Vec::new();

    for line in lines {
        let line: u32 = line.trim().parse().expect("Could not parse line");

        window.push(line);
        if window.len() <= 3 {
            continue;
        }

        if window.len() > 3 {
            window.remove(0);
        }

        if prev_maximum < window_sum(&window) {
            num_increases += 1;
        }

        prev_maximum = window_sum(&window);
    }

    println!("{}", format!("Number of increases: {}", num_increases));
}
