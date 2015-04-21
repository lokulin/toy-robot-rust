use toyrobot::point::*;

#[derive(Copy)]
pub struct Table {
    pub llc: Point,
    pub urc: Point,
}

impl Table {
    pub fn new(llc: Point, urc: Point) -> Table {
        Table { llc: llc, urc: urc }
    }

    pub fn contains(&self, location: &Point) -> bool {
        self.llc.x <= location.x && self.llc.y <= location.y
            && self.urc.x >= location.x && self.urc.y >= location.y
    }
}
