use toyrobot::point::*;
use toyrobot::table::*;

pub struct Robot {
  pub location: Point,
  pub facing: f64,
  pub table: Table,
}

impl Robot {
  pub fn movef(self) -> Robot {
    self
  }
  
  pub fn left(self) -> Robot {
    self
  }

  pub fn right(self) -> Robot {
    self
  }
  
  pub fn place(self, location: Point, facing: f64, table: Table) -> Robot {
    if table.contains(&location) {
      Robot { location: location, facing: facing, table: table }
    } else {
      self
    }
  }
  
  pub fn report(&self) {
    println!("Hello world!");
  }

}

