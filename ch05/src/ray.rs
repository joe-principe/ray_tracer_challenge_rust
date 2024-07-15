extern crate nalgebra_glm as glm;
use glm::Mat4;
use glm::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
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

    pub fn position(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }

    pub fn transform(r: &Ray, m: &Mat4) -> Ray {
        Ray {
            origin: glm::vec4_to_vec3(
                &(m * glm::vec4(r.origin.x, r.origin.y, r.origin.z, 1.0)),
            ),
            direction: glm::vec4_to_vec3(
                &(m * glm::vec4(r.direction.x, r.direction.y, r.direction.z, 0.0)),
            ),
        }
    }
}
