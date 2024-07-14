#[cfg(test)]
mod ray_tests {
    extern crate nalgebra_glm as glm;
    use glm::Vec3;

    use crate::ray::Ray;

    #[test]
    fn build_ray_origin_same() {
        let origin_vec = glm::vec3(1.0, 2.0, 3.0);
        let direction_vec = glm::vec3(4.0, 5.0, 6.0);

        let r = Ray::build(&origin_vec, &direction_vec);

        assert_eq!(
            glm::vec3(true, true, true),
            glm::equal(&r.origin(), &origin_vec)
        );
    }

    #[test]
    fn build_ray_origin_diff_x() {
        let origin_vec = glm::vec3(1.0, 2.0, 3.0);
        let direction_vec = glm::vec3(4.0, 5.0, 6.0);

        let r = Ray::build(&origin_vec, &direction_vec);

        assert_eq!(
            glm::vec3(false, true, true),
            glm::equal(&r.origin(), &glm::vec3(0.0, 2.0, 3.0))
        );
    }

    #[test]
    fn build_ray_origin_diff_y() {
        let origin_vec = glm::vec3(1.0, 2.0, 3.0);
        let direction_vec = glm::vec3(4.0, 5.0, 6.0);

        let r = Ray::build(&origin_vec, &direction_vec);

        assert_eq!(
            glm::vec3(true, false, true),
            glm::equal(&r.origin(), &glm::vec3(1.0, 0.0, 3.0))
        );
    }

    #[test]
    fn build_ray_origin_diff_z() {
        let origin_vec = glm::vec3(1.0, 2.0, 3.0);
        let direction_vec = glm::vec3(4.0, 5.0, 6.0);

        let r = Ray::build(&origin_vec, &direction_vec);

        assert_eq!(
            glm::vec3(true, true, false),
            glm::equal(&r.origin(), &glm::vec3(1.0, 2.0, 0.0))
        );
    }

    #[test]
    fn build_ray_direction_same() {
        let origin_vec = glm::vec3(1.0, 2.0, 3.0);
        let direction_vec = glm::vec3(4.0, 5.0, 6.0);

        let r = Ray::build(&origin_vec, &direction_vec);

        assert_eq!(
            glm::vec3(true, true, true),
            glm::equal(&r.direction(), &&direction_vec)
        );
    }

    #[test]
    fn build_ray_direction_diff_x() {
        let origin_vec = glm::vec3(1.0, 2.0, 3.0);
        let direction_vec = glm::vec3(4.0, 5.0, 6.0);

        let r = Ray::build(&origin_vec, &direction_vec);

        assert_eq!(
            glm::vec3(false, true, true),
            glm::equal(&r.direction(), &glm::vec3(0.0, 5.0, 6.0))
        );
    }

    #[test]
    fn build_ray_direction_diff_y() {
        let origin_vec = glm::vec3(1.0, 2.0, 3.0);
        let direction_vec = glm::vec3(4.0, 5.0, 6.0);

        let r = Ray::build(&origin_vec, &direction_vec);

        assert_eq!(
            glm::vec3(true, false, true),
            glm::equal(&r.direction(), &glm::vec3(4.0, 0.0, 6.0))
        );
    }

    #[test]
    fn build_ray_direction_diff_z() {
        let origin_vec = glm::vec3(1.0, 2.0, 3.0);
        let direction_vec = glm::vec3(4.0, 5.0, 6.0);

        let r = Ray::build(&origin_vec, &direction_vec);

        assert_eq!(
            glm::vec3(true, true, false),
            glm::equal(&r.direction(), &glm::vec3(4.0, 5.0, 0.0))
        );
    }

    #[test]
    fn distance_along_ray() {
        let origin_vec = glm::vec3(2.0, 3.0, 4.0);
        let direction_vec = glm::vec3(1.0, 0.0, 0.0);

        let r = Ray::build(&origin_vec, &direction_vec);
        let t1: f32 = 0.0;
        let t2: f32 = 1.0;
        let t3: f32 = -1.0;
        let t4: f32 = 2.5;

        let dist1: Vec3 = r.position(t1);
        let dist2: Vec3 = r.position(t2);
        let dist3: Vec3 = r.position(t3);
        let dist4: Vec3 = r.position(t4);

        assert_eq!(
            glm::vec3(true, true, true),
            glm::equal(&dist1, &glm::vec3(2.0, 3.0, 4.0))
        );
        assert_eq!(
            glm::vec3(true, true, true),
            glm::equal(&dist2, &glm::vec3(3.0, 3.0, 4.0))
        );
        assert_eq!(
            glm::vec3(true, true, true),
            glm::equal(&dist3, &glm::vec3(1.0, 3.0, 4.0))
        );
        assert_eq!(
            glm::vec3(true, true, true),
            glm::equal(&dist4, &glm::vec3(4.5, 3.0, 4.0))
        );
    }
}

#[cfg(test)]
mod ray_sphere_tests {
    use std::ptr;

    extern crate nalgebra_glm as glm;

    use crate::ray::Ray;
    use crate::sphere::Sphere;
    use crate::intersection::Intersection;
    use crate::hittable::Hittable;

    #[test]
    fn ray_sphere_two_hit_val() {
        let r = Ray::build(&glm::vec3(0.0, 0.0, -5.0), &glm::vec3(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let xs: Vec<Intersection> = s.intersect(&r);

        assert_eq!(xs.len(), 2);
        float_cmp::assert_approx_eq!(f32, xs[0].t(), 4.0);
        float_cmp::assert_approx_eq!(f32, xs[1].t(), 6.0);
    }

    #[test]
    fn ray_sphere_two_hit_obj() {
        let r = Ray::build(&glm::vec3(0.0, 0.0, -5.0), &glm::vec3(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let xs: Vec<Intersection> = s.intersect(&r);

        assert_eq!(xs.len(), 2);
        assert!(ptr::addr_eq(xs[0].obj(), &s));
        assert!(ptr::addr_eq(xs[1].obj(), &s));
    }

    #[test]
    fn ray_sphere_one_hit_val() {
        let r = Ray::build(&glm::vec3(0.0, 1.0, -5.0), &glm::vec3(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let xs: Vec<Intersection> = s.intersect(&r);

        assert_eq!(xs.len(), 2);
        float_cmp::assert_approx_eq!(f32, xs[0].t(), 5.0);
        float_cmp::assert_approx_eq!(f32, xs[1].t(), 5.0);
    }

    #[test]
    fn ray_sphere_one_hit_obj() {
        let r = Ray::build(&glm::vec3(0.0, 1.0, -5.0), &glm::vec3(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let xs: Vec<Intersection> = s.intersect(&r);

        assert_eq!(xs.len(), 2);
        assert!(ptr::addr_eq(xs[0].obj(), &s));
        assert!(ptr::addr_eq(xs[1].obj(), &s));
    }

    #[test]
    fn ray_sphere_miss() {
        let r = Ray::build(&glm::vec3(0.0, 2.0, -5.0), &glm::vec3(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let xs: Vec<Intersection> = s.intersect(&r);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn ray_sphere_inside_val() {
        let r = Ray::build(&glm::vec3(0.0, 0.0, 0.0), &glm::vec3(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let xs: Vec<Intersection> = s.intersect(&r);

        assert_eq!(xs.len(), 2);
        float_cmp::assert_approx_eq!(f32, xs[0].t(), -1.0);
        float_cmp::assert_approx_eq!(f32, xs[1].t(), 1.0);
    }

    #[test]
    fn ray_sphere_inside_obj() {
        let r = Ray::build(&glm::vec3(0.0, 0.0, 0.0), &glm::vec3(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let xs: Vec<Intersection> = s.intersect(&r);

        assert_eq!(xs.len(), 2);
        assert!(ptr::addr_eq(xs[0].obj(), &s));
        assert!(ptr::addr_eq(xs[1].obj(), &s));
    }

    #[test]
    fn ray_sphere_behind_val() {
        let r = Ray::build(&glm::vec3(0.0, 0.0, 5.0), &glm::vec3(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let xs: Vec<Intersection> = s.intersect(&r);

        assert_eq!(xs.len(), 2);
        float_cmp::assert_approx_eq!(f32, xs[0].t(), -6.0);
        float_cmp::assert_approx_eq!(f32, xs[1].t(), -4.0);
    }

    #[test]
    fn ray_sphere_behind_obj() {
        let r = Ray::build(&glm::vec3(0.0, 0.0, 5.0), &glm::vec3(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let xs: Vec<Intersection> = s.intersect(&r);

        assert_eq!(xs.len(), 2);
        assert!(ptr::addr_eq(xs[0].obj(), &s));
        assert!(ptr::addr_eq(xs[1].obj(), &s));
    }
}

#[cfg(test)]
mod intersection_test {
    use std::ptr;

    extern crate nalgebra_glm as glm;

    use crate::sphere::Sphere;
    use crate::intersection::Intersection;

    #[test]
    fn create_intersection() {
        let s = Sphere::new();
        let i = Intersection::build(3.5, &s);

        float_cmp::assert_approx_eq!(f32, i.t(), 3.5);
        assert!(ptr::addr_eq(i.obj(), &s));
    }

    #[test]
    fn aggregate_intersections() {
        let s = Sphere::new();
        let i1 = Intersection::build(1.0, &s);
        let i2 = Intersection::build(2.0, &s);

        let xs = vec![i1, i2];

        assert_eq!(xs.len(), 2);
        float_cmp::assert_approx_eq!(f32, xs[0].t(), 1.0);
        float_cmp::assert_approx_eq!(f32, xs[1].t(), 2.0);
    }
}
