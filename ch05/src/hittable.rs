use crate::intersection::Intersection;
use crate::ray::Ray;

pub trait Hittable {
    fn intersect(&self, r: &Ray) -> Vec<Intersection>;
}
