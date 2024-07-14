use crate::ray::Ray;
use crate::intersection::Intersection;

pub trait Hittable {
    fn intersect(&self, r: &Ray) -> Vec<Intersection>;
}
