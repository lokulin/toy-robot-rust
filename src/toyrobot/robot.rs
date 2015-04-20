use toyrobot::point::*;
use toyrobot::table::*;

use std::f64::consts::PI;

#[derive(Copy)]
pub struct Robot {
    pub loc: Point,
    pub facing: f64,
    pub table: Table,
}

impl Robot {
    pub fn movef(self) -> Robot {
        let (x, y) = (self.facing * PI).sin_cos();
        self.place(Point { x: x + self.loc.x, y: y + self.loc.y }, self.facing, self.table)
    }

    pub fn left(self) -> Robot {
        self.place(self.loc, self.facing - 0.5, self.table)
    }

    pub fn right(self) -> Robot {
        self.place(self.loc, self.facing + 0.5, self.table)
    }

    pub fn place(self, loc: Point, facing: f64, table: Table) -> Robot {
        if table.contains(&loc) {
            Robot { loc: loc, facing: facing, table: table }
        } else {
            self
        }
    }

    pub fn report(&self) {
        println!("{} {} {}", self.loc.x, self.loc.y, self.facing);
    }

}
