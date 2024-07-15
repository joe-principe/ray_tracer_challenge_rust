extern crate nalgebra_glm as glm;

use glm::Vec3;
use glm::Mat4;

use crate::hittable::Hittable;
use crate::intersection::Intersection;
use crate::ray::Ray;

#[allow(dead_code)]
pub struct Sphere {
    origin: Vec3,
    radius: f32,
    pub transform: Mat4,
}

#[allow(dead_code)]
impl Sphere {
    pub fn new() -> Sphere {
        Sphere {
            origin: glm::vec3(0.0, 0.0, 0.0),
            radius: 1.0,
            transform: Mat4::identity(),
        }
    }

    pub fn build(o: &Vec3, r: f32, t: &Mat4) -> Sphere {
        Sphere {
            origin: o.clone(),
            radius: r,
            transform: t.clone(),
        }
    }

    pub fn origin(&self) -> &Vec3 {
        &self.origin
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }
}

impl Hittable for Sphere {
    fn intersect(&self, r: &Ray) -> Vec<Intersection> {
        let r2 = Ray::transform(&r, &glm::inverse(&self.transform));

        let sphere_to_ray: Vec3 = r2.origin - glm::Vec3::zeros();

        let a: f32 = glm::dot(&r2.direction, &r2.direction);
        let b: f32 = 2.0 * glm::dot(&r2.direction, &sphere_to_ray);
        let c: f32 = glm::dot(&sphere_to_ray, &sphere_to_ray) - 1.0;

        let discriminant: f32 = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return vec![];
        }

        let t1 = (-b - f32::sqrt(discriminant)) / (2.0 * a);
        let t2 = (-b + f32::sqrt(discriminant)) / (2.0 * a);

        let i1 = Intersection::build(t1, self);
        let i2 = Intersection::build(t2, self);

        vec![i1, i2]
    }
}
