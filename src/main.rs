mod toyrobot;

use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;
use toyrobot::*;

fn execute(robot: Robot, command: String) -> Robot {
    println!("{}", command);
    robot
}

fn main() {
    let robot = Robot::new(Point::new(0.0,0.0), 0.0, None);

    //TODO: Catch file not found etc.
    BufReader::new(File::open("examples/example5.txt").unwrap())
                .lines()
                .fold(robot, |robot, line| execute(robot, line.unwrap()));
}
