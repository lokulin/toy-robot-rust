mod toyrobot;

use toyrobot::*;

fn main() {
    let llc = Point { x: 0.0, y: 0.0 };
    let urc = Point { x: 4.0, y: 4.0 };
    let table = Table { llc: llc, urc: urc };
    let location = Point { x: 0.0, y: 0.0 };
    let robot = Robot { location: location, facing: 0.0, table: table };
   
    robot.report(); 
}

