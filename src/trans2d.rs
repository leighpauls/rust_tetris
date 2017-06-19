
#[derive(Clone)]
pub struct Trans2D {
    pub x: i32,
    pub y: i32,
}

impl Trans2D {
    pub fn new(x: i32, y: i32) -> Trans2D {
        Trans2D { x: x, y: y }
    }
    pub fn from_tup(t: (i32, i32)) -> Trans2D {
        Self::new(t.0, t.1)
    }
    pub fn invert(&self) -> Trans2D {
        Self::new(-self.x, -self.y)
    }
    pub fn trans(&self, by: &Trans2D) -> Trans2D {
        Self::new(self.x + by.x, self.y + by.y)
    }
}
