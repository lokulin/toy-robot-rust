mod toyrobot;

extern crate regex;

use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;
use std::env;
use regex::Regex;
use toyrobot::*;

fn execute(robot: Robot, line: String, table: Option<Table>, validator: &Regex) -> Robot {
    let caps = validator.captures(line.as_ref());

    //TODO: This fn could be neater
    match caps {
        Some(cmd) => match cmd.name("command") {
            Some("MOVE") => robot.movef(),
            Some("LEFT") => robot.left(),
            Some("RIGHT") => robot.right(),
            Some("REPORT") => {
                robot.report();
                robot
            },
            _ => {
                let x: f64 = cmd.name("x").unwrap().parse().unwrap();
                let y: f64 = cmd.name("y").unwrap().parse().unwrap();
                let loc = Point::new(x, y);
                match cmd.name("dir") {
                    //TODO: Clean up repetition
                    Some("NORTH") => robot.place(loc, 0.0, table),
                    Some("EAST") => robot.place(loc, 0.5, table),
                    Some("SOUTH") => robot.place(loc, 1.0, table),
                    Some("WEST") => robot.place(loc, 1.0, table),
                    _ => robot
                }
            }
        },
        None => robot
    }
}

fn main() {
    let robot = Robot::new(Point::new(0.0,0.0), 0.0, None);
    let table = Some(Table::new(Point::new(0.0,0.0),Point::new(4.0,4.0)));
    let validator = Regex::new(r"^(?P<command>MOVE|LEFT|RIGHT|REPORT|PLACE\s?(?P<x>\d+),(?P<y>\d+),(?P<dir>NORTH|EAST|SOUTH|WEST))$").unwrap();

    //TODO: This could be neater
    if env::args().count() == 2 {
        match File::open(env::args().nth(1).unwrap()) {
            Ok(file) => {
                BufReader::new(file)
                    .lines()
                    .fold(robot, |robot, line| execute(robot, line.unwrap(), table, &validator));
                    ()
            },
            _ => {
                println!("Error reading file: {}", env::args().nth(1).unwrap());
                ()
            }
        }
    } else {
        println!("usage: toyrobot <filename>");
    }
}
