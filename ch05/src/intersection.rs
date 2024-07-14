extern crate nalgebra_glm as glm;

use crate::hittable::Hittable;

#[allow(dead_code)]
pub struct Intersection<'a> {
    t: f32,
    obj: &'a (dyn Hittable + 'static),
}

#[allow(dead_code)]
impl<'a> Intersection<'a> {
    pub fn build(_t: f32, object: &'a (impl Hittable + 'static)) -> Intersection<'a> {
        Intersection {
            t: _t,
            obj: object,
        }
    }

    pub fn t(&self) -> f32 {
        self.t
    }

    pub fn obj(&self) -> &'a dyn Hittable {
        self.obj
    }
}
