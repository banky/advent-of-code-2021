use std::fs;
use std::process;

fn main() {
    let filename = "src/input.txt";

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines: Vec<_> = contents
        .lines()
        .map(|line| Command::new(line))
        .map(|command_result| {
            command_result.unwrap_or_else(|err| {
                eprintln!("Problem parsing arguments: {}", err);
                process::exit(1);
            })
        })
        .collect();

    part_1(&lines);
    part_2(&lines);
}

#[derive(PartialEq, Debug)]
enum Command {
    Forward(u32),
    Down(u32),
    Up(u32),
}

impl Command {
    fn new(command: &str) -> Result<Command, &str> {
        let mut pattern = command.split(" ");
        let direction = match pattern.next() {
            Some(x) => x,
            None => return Err("No direction for line"),
        };

        let magnitude: u32 = match pattern.next() {
            Some(x) => x.parse().unwrap(),
            None => return Err("No magnitude for line"),
        };

        if direction.contains("down") {
            Ok(Command::Down(magnitude))
        } else if direction.contains("up") {
            Ok(Command::Up(magnitude))
        } else {
            Ok(Command::Forward(magnitude))
        }
    }
}

fn part_1(lines: &Vec<Command>) {
    let mut h_pos = 0;
    let mut depth = 0;

    for line in lines.iter() {
        match *line {
            Command::Down(magnitude) => depth += magnitude,
            Command::Up(magnitude) => depth -= magnitude,
            Command::Forward(magnitude) => h_pos += magnitude,
        }
    }

    println!("Depth x Horizontal position = {}", h_pos * depth);
}

fn part_2(lines: &Vec<Command>) {
    let mut aim = 0;
    let mut h_pos = 0;
    let mut depth = 0;

    for line in lines.iter() {
        match *line {
            Command::Down(magnitude) => aim += magnitude,
            Command::Up(magnitude) => aim -= magnitude,
            Command::Forward(magnitude) => {
                h_pos += magnitude;
                depth += aim * magnitude;
            }
        }
    }

    println!("Depth x Horizontal position = {}", h_pos * depth);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn forward_command() {
        let command = Command::new("forward 1").unwrap();
        assert_eq!(command, Command::Forward(1));
    }

    #[test]
    fn up_command() {
        let command = Command::new("up 1").unwrap();
        assert_eq!(command, Command::Up(1));
    }

    #[test]
    fn down_command() {
        let command = Command::new("down 1").unwrap();
        assert_eq!(command, Command::Down(1));
    }

    #[test]
    fn invalid_command() {
        let error = Command::new("down").unwrap_err();
        assert_eq!(error, "No magnitude for line");
    }
}
