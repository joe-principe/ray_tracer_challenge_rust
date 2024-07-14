extern crate nalgebra_glm as glm;
use glm::Vec3;

pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

#[allow(dead_code)]
impl Ray {
    pub fn new() -> Ray {
        Ray {
            origin: glm::vec3(0.0, 0.0, 0.0),
            direction: glm::vec3(0.0, 0.0, 0.0),
        }
    }

    pub fn build(o: &Vec3, d: &Vec3) -> Ray {
        Ray {
            origin: o.clone(),
            direction: d.clone(),
        }
    }

    pub fn origin(&self) -> &Vec3 {
        &self.origin
    }

    pub fn direction(&self) -> &Vec3 {
        &self.direction
    }

    pub fn position(&self, t: f32) -> Vec3 {
        self.origin() + self.direction() * t
    }
}
