use toyrobot::point::*;
use toyrobot::table::*;

use std::f64::consts::PI;

#[derive(Copy)]
pub struct Robot {
    pub loc: Point,
    pub facing: f64,
    pub table: Option<Table>,
}

impl Robot {
    pub fn new(loc: Point, facing: f64, table: Option<Table>) -> Robot {
        Robot { loc: loc, facing: facing, table: table }
    }

    pub fn movef(self) -> Robot {
        let (x, y) = (self.facing * PI).sin_cos();
        self.place(Point::new(x + self.loc.x, y + self.loc.y), self.facing, self.table)
    }

    pub fn left(self) -> Robot {
        //TODO: %2?
        self.place(self.loc, self.facing - 0.5, self.table)
    }

    pub fn right(self) -> Robot {
        //TODO: %2?
        self.place(self.loc, self.facing + 0.5, self.table)
    }

    pub fn place(self, loc: Point, facing: f64, table: Option<Table>) -> Robot {
        match table {
            Some(t) => {
                if t.contains(&loc) {
                    Robot::new(loc, facing, table)
                } else {
                    self
                }
            },
            None => self
        }
    }

    pub fn report(&self) {
        //TODO: Write direction string instead of internal facing representation.
        match self.table {
            Some(_) => println!("{} {} {}", self.loc.x, self.loc.y, self.facing),
            None => ()
        }
    }
}
