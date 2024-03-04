use geometry::Vec2;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Zoom(u8);

impl Zoom {
    pub fn new(n: u8) -> Self {
        Self(n)
    }

    pub fn asf32(&self) -> f32 {
        (*self).into()
    }

    pub fn as_scale(&self) -> Vec2 {
        let f = self.asf32();
        Vec2::new(f, f)
    }

    pub fn inc(&mut self) -> bool {
        if self.0 < 7 {
            self.0 += 1;
            true
        } else {
            false
        }
    }

    pub fn dec(&mut self) -> bool {
        if self.0 > 1 {
            self.0 -= 1;
            true
        } else {
            false
        }
    }
}

impl Default for Zoom {
    fn default() -> Self {
        Self(4)
    }
}

impl From<Zoom> for f32 {
    fn from(z: Zoom) -> Self {
        match z.0 {
            1 | 0 => 0.25,
            2 => 0.5,
            3 => 1.0,
            4 => 2.0,
            5 => 3.0,
            6 => 4.0,
            7.. => 5.0,
        }
    }
}
