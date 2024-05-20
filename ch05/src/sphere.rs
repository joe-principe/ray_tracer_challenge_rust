extern crate nalgebra_glm as glm;
use glm::Vec3;

use crate::ray::Ray;

pub struct Sphere {
    origin: Vec3,
    radius: f32,
}

impl Sphere {
    pub fn new() -> Sphere {
        Sphere {
            origin: glm::vec3(0.0, 0.0, 0.0),
            radius: 1.0,
        }
    }

    pub fn build(o: &Vec3, r: f32) -> Sphere {
        Sphere {
            origin: o.clone(),
            radius: r,
        }
    }

    pub fn intersect(&self, r: &Ray) -> Vec<f32> {
        let sphere_to_ray: Vec3 = r.origin() - glm::vec3(0.0, 0.0, 0.0);

        let a: f32 = glm::dot(r.direction(), r.direction());
        let b: f32 = 2.0 * glm::dot(r.direction(), &sphere_to_ray);
        let c: f32 = glm::dot(&sphere_to_ray, &sphere_to_ray) - 1.0;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return vec![];
        }

        let t1 = (-b - f32::sqrt(discriminant)) / (2.0 * a);
        let t2 = (-b + f32::sqrt(discriminant)) / (2.0 * a);

        vec![t1, t2]
    }
}
