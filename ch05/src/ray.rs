extern crate nalgebra_glm as glm;
use glm::Vec3;

struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    fn new() -> Ray {
        Ray {
            origin: glm::vec3(0.0, 0.0, 0.0),
            direction: glm::vec3(0.0, 0.0, 0.0)
        }
    }

    fn build(o: Vec3, d: Vec3) -> Ray {
        Ray {
            origin: o,
            direction: d,
        }
    }

    fn origin(&self) -> &Vec3 {
        &self.origin
    }

    fn direction(&self) -> &Vec3 {
        &self.direction
    }
}
