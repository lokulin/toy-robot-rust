mod toyrobot;

extern crate regex;

use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;
use regex::Regex;
use toyrobot::*;

fn execute(robot: Robot, line: String) -> Robot {
    let re = Regex::new(r"^(?P<command>MOVE|LEFT|RIGHT|REPORT|PLACE\s?(\d+),(\d+),(NORTH|EAST|SOUTH|WEST))$").unwrap();
    let caps = re.captures(line.as_ref()).unwrap();

    match caps.name("command") {
        Some("MOVE") => robot.movef(),
        Some("LEFT") => robot.left(),
        Some("RIGHT") => robot.right(),
        Some("REPORT") => {
            robot.report(); 
            robot
        },
        _ => {
            let re = Regex::new(r"^(?P<place>PLACE)\s?(\d+),(\d+),(NORTH|EAST|SOUTH|WEST)$").unwrap();
            let caps = re.captures(line.as_ref()).unwrap();
            robot.place(Point::new(1.0,2.0), 0.5, Some(Table::new(Point::new(0.0,0.0),Point::new(4.0,4.0))))
        }
    }
}

fn main() {
    let robot = Robot::new(Point::new(0.0,0.0), 0.0, None);

    //TODO: Catch file not found etc.
    BufReader::new(File::open("examples/example5.txt").unwrap())
                .lines()
                .fold(robot, |robot, line| execute(robot, line.unwrap()));
}
