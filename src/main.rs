mod toyrobot;

use toyrobot::*;

fn main() {
    // n = 0.0, e = 0.5 , s = 1.0, w=1.5
    let llc = Point { x: 0.0, y: 0.0 };
    let urc = Point { x: 4.0, y: 4.0 };
    let table = Table { llc: llc, urc: urc };
    let loc = Point { x: 0.0, y: 0.0 };
    let mut robot = Robot { loc: loc, facing: 0.0, table: table };

    robot.report();
    robot = robot.movef();
    robot.report();
    robot = robot.right();
    robot.report();
    robot = robot.movef();
    robot.report();
}
