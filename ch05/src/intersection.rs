extern crate nalgebra_glm as glm;

use crate::hittable::Hittable;

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct Intersection<'a> {
    t: f32,
    obj: &'a (dyn Hittable + 'static),
}

#[allow(dead_code)]
impl<'a> Intersection<'a> {
    pub fn build(_t: f32, object: &'a (impl Hittable + 'static)) -> Intersection<'a> {
        Intersection { t: _t, obj: object }
    }

    pub fn hit(intersections: &Vec<Intersection<'a>>) -> Option<Intersection<'a>> {
        let mut t_min: f32 = f32::MAX;
        let mut hit: Option<Intersection<'a>> = None;

        for i in intersections.into_iter() {
            if i.t() < t_min && i.t() > 0.0 {
                t_min = i.t();
                hit = Some(*i);
            }
        }

        hit
    }

    pub fn t(&self) -> f32 {
        self.t
    }

    pub fn obj(&self) -> &'a dyn Hittable {
        self.obj
    }
}
