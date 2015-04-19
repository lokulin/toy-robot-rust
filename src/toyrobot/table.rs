use toyrobot::point::*;

pub struct Table {
  pub llc: Point,
  pub urc: Point,
}

impl Table {
  pub fn contains(&self, location: &Point) -> bool {
    self.llc.x <= location.x && self.llc.y <= location.y
      && self.urc.x >= location.x && self.urc.y >= location.y
  }
}
