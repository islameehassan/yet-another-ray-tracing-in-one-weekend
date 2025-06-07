use crate::vec3::vec3::Point3;

#[derive(Debug, Clone, Default)]
pub struct Ray {
    origin: Point3,
    direction: Point3,
}

impl Ray {
    pub fn new(ori: Point3, dir: Point3) -> Self {
        Ray {
            origin: ori,
            direction: dir,
        }
    }

    pub fn at(&self, t: f32) -> Point3 {
        self.origin + t * self.direction
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn direction(&self) -> Point3 {
        self.direction
    }
}
