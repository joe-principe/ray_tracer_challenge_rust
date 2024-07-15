#[cfg(test)]
mod ray_tests {
    extern crate nalgebra_glm as glm;
    use glm::Vec3;

    use crate::ray::Ray;

    // Creates a ray @ (1, 2, 3) in the direction (4, 5, 6)
    // Checks if the origin is the same
    #[test]
    fn build_ray_origin_same() {
        let origin_vec = glm::vec3(1.0, 2.0, 3.0);
        let direction_vec = glm::vec3(4.0, 5.0, 6.0);

        let r = Ray::build(&origin_vec, &direction_vec);

        assert_eq!(
            glm::vec3(true, true, true),
            glm::equal(&r.origin, &origin_vec)
        );
    }

    // Creates a ray @ (1, 2, 3) in the direction (4, 5, 6)
    // Checks that all but the x-coordinate are the same
    #[test]
    fn build_ray_origin_diff_x() {
        let origin_vec = glm::vec3(1.0, 2.0, 3.0);
        let direction_vec = glm::vec3(4.0, 5.0, 6.0);

        let r = Ray::build(&origin_vec, &direction_vec);

        assert_eq!(
            glm::vec3(false, true, true),
            glm::equal(&r.origin, &glm::vec3(0.0, 2.0, 3.0))
        );
    }

    // Creates a ray @ (1, 2, 3) in the direction (4, 5, 6)
    // Checks that all but the y-coordinate are the same
    #[test]
    fn build_ray_origin_diff_y() {
        let origin_vec = glm::vec3(1.0, 2.0, 3.0);
        let direction_vec = glm::vec3(4.0, 5.0, 6.0);

        let r = Ray::build(&origin_vec, &direction_vec);

        assert_eq!(
            glm::vec3(true, false, true),
            glm::equal(&r.origin, &glm::vec3(1.0, 0.0, 3.0))
        );
    }

    // Creates a ray @ (1, 2, 3) in the direction (4, 5, 6)
    // Checks that all but the z-coordinate are the same
    #[test]
    fn build_ray_origin_diff_z() {
        let origin_vec = glm::vec3(1.0, 2.0, 3.0);
        let direction_vec = glm::vec3(4.0, 5.0, 6.0);

        let r = Ray::build(&origin_vec, &direction_vec);

        assert_eq!(
            glm::vec3(true, true, false),
            glm::equal(&r.origin, &glm::vec3(1.0, 2.0, 0.0))
        );
    }

    // Creates a ray @ (1, 2, 3) in the direction (4, 5, 6)
    // Checks if the direction is the same
    #[test]
    fn build_ray_direction_same() {
        let origin_vec = glm::vec3(1.0, 2.0, 3.0);
        let direction_vec = glm::vec3(4.0, 5.0, 6.0);

        let r = Ray::build(&origin_vec, &direction_vec);

        assert_eq!(
            glm::vec3(true, true, true),
            glm::equal(&r.direction, &&direction_vec)
        );
    }

    // Creates a ray @ (1, 2, 3) in the direction (4, 5, 6)
    // Checks if all but the x-coordinate are the same
    #[test]
    fn build_ray_direction_diff_x() {
        let origin_vec = glm::vec3(1.0, 2.0, 3.0);
        let direction_vec = glm::vec3(4.0, 5.0, 6.0);

        let r = Ray::build(&origin_vec, &direction_vec);

        assert_eq!(
            glm::vec3(false, true, true),
            glm::equal(&r.direction, &glm::vec3(0.0, 5.0, 6.0))
        );
    }

    // Creates a ray @ (1, 2, 3) in the direction (4, 5, 6)
    // Checks if all but the y-coordinate are the same
    #[test]
    fn build_ray_direction_diff_y() {
        let origin_vec = glm::vec3(1.0, 2.0, 3.0);
        let direction_vec = glm::vec3(4.0, 5.0, 6.0);

        let r = Ray::build(&origin_vec, &direction_vec);

        assert_eq!(
            glm::vec3(true, false, true),
            glm::equal(&r.direction, &glm::vec3(4.0, 0.0, 6.0))
        );
    }

    // Creates a ray @ (1, 2, 3) in the direction (4, 5, 6)
    // Checks if all but the z-coordinate are the same
    #[test]
    fn build_ray_direction_diff_z() {
        let origin_vec = glm::vec3(1.0, 2.0, 3.0);
        let direction_vec = glm::vec3(4.0, 5.0, 6.0);

        let r = Ray::build(&origin_vec, &direction_vec);

        assert_eq!(
            glm::vec3(true, true, false),
            glm::equal(&r.direction, &glm::vec3(4.0, 5.0, 0.0))
        );
    }

    // Creates a ray @ (2, 3, 4) in the positive x-direction
    // Checks various t values along the ray
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

    use crate::hittable::Hittable;
    use crate::intersection::Intersection;
    use crate::ray::Ray;
    use crate::sphere::Sphere;

    // Collides with a sphere twice
    // Checks that there are two intersections
    // Checks that both intersections are positive
    #[test]
    fn ray_sphere_two_hit_val() {
        let r = Ray::build(&glm::vec3(0.0, 0.0, -5.0), &glm::vec3(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let xs: Vec<Intersection> = s.intersect(&r);

        assert_eq!(xs.len(), 2);
        float_cmp::assert_approx_eq!(f32, xs[0].t(), 4.0);
        float_cmp::assert_approx_eq!(f32, xs[1].t(), 6.0);
    }

    // Collides with a sphere twice
    // Checks if both intersections refer to the same object
    #[test]
    fn ray_sphere_two_hit_obj() {
        let r = Ray::build(&glm::vec3(0.0, 0.0, -5.0), &glm::vec3(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let xs: Vec<Intersection> = s.intersect(&r);

        assert_eq!(xs.len(), 2);
        assert!(ptr::addr_eq(xs[0].obj(), &s));
        assert!(ptr::addr_eq(xs[1].obj(), &s));
    }

    // Collides with a sphere once
    // Checks that there's one intersection (kinda)
    // Checks that the intersection is positive
    #[test]
    fn ray_sphere_one_hit_val() {
        let r = Ray::build(&glm::vec3(0.0, 1.0, -5.0), &glm::vec3(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let xs: Vec<Intersection> = s.intersect(&r);

        assert_eq!(xs.len(), 2);
        float_cmp::assert_approx_eq!(f32, xs[0].t(), 5.0);
        float_cmp::assert_approx_eq!(f32, xs[1].t(), 5.0);
    }

    // Collides with a sphere once
    // Checks that the intersection refers to the correct object
    #[test]
    fn ray_sphere_one_hit_obj() {
        let r = Ray::build(&glm::vec3(0.0, 1.0, -5.0), &glm::vec3(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let xs: Vec<Intersection> = s.intersect(&r);

        assert_eq!(xs.len(), 2);
        assert!(ptr::addr_eq(xs[0].obj(), &s));
        assert!(ptr::addr_eq(xs[1].obj(), &s));
    }

    // Doesn't collide with a sphere
    // Checks that there are no intersections
    #[test]
    fn ray_sphere_miss() {
        let r = Ray::build(&glm::vec3(0.0, 2.0, -5.0), &glm::vec3(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let xs: Vec<Intersection> = s.intersect(&r);

        assert_eq!(xs.len(), 0);
    }

    // Ray originates within the sphere
    // Intersects in front and behind
    // Checks for both intersections and positive/negative
    #[test]
    fn ray_sphere_inside_val() {
        let r = Ray::build(&glm::vec3(0.0, 0.0, 0.0), &glm::vec3(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let xs: Vec<Intersection> = s.intersect(&r);

        assert_eq!(xs.len(), 2);
        float_cmp::assert_approx_eq!(f32, xs[0].t(), -1.0);
        float_cmp::assert_approx_eq!(f32, xs[1].t(), 1.0);
    }

    // Ray originates within the sphere
    // Intersects in front and behind
    // Checks that both intersections refer to the same object
    #[test]
    fn ray_sphere_inside_obj() {
        let r = Ray::build(&glm::vec3(0.0, 0.0, 0.0), &glm::vec3(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let xs: Vec<Intersection> = s.intersect(&r);

        assert_eq!(xs.len(), 2);
        assert!(ptr::addr_eq(xs[0].obj(), &s));
        assert!(ptr::addr_eq(xs[1].obj(), &s));
    }

    // Ray intersects with the sphere behind it
    // Checks that both intersections are negative
    #[test]
    fn ray_sphere_behind_val() {
        let r = Ray::build(&glm::vec3(0.0, 0.0, 5.0), &glm::vec3(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let xs: Vec<Intersection> = s.intersect(&r);

        assert_eq!(xs.len(), 2);
        float_cmp::assert_approx_eq!(f32, xs[0].t(), -6.0);
        float_cmp::assert_approx_eq!(f32, xs[1].t(), -4.0);
    }

    // Ray intersects with the sphere behind it
    // Checks that both intersections refer to the same sphere
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

    use crate::intersection::Intersection;
    use crate::sphere::Sphere;

    // Creates an intersection object
    // Checks that the intersection was created correctly
    #[test]
    fn create_intersection() {
        let s = Sphere::new();
        let i = Intersection::build(3.5, &s);

        float_cmp::assert_approx_eq!(f32, i.t(), 3.5);
        assert!(ptr::addr_eq(i.obj(), &s));
    }

    // Creates a vector of intersections
    // Truly a useless test tbh
    // It was supposed to be for some "intersections" function, but that was pretty useless
    // considering the vec! macro can do exactly the same thing much more easily
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

#[cfg(test)]
mod sphere_hit_test {
    use std::ptr;

    extern crate nalgebra_glm as glm;

    use crate::intersection::Intersection;
    use crate::sphere::Sphere;

    // Ray intersects with both sides of a sphere.
    // Both intersections are positive t-values
    // i1 is the closer intersection
    #[test]
    fn two_hit_all_positive() {
        let s = Sphere::new();
        let i1 = Intersection::build(1.0, &s);
        let i2 = Intersection::build(2.0, &s);

        let i = Intersection::hit(&vec![i2, i1]);

        assert!(i.is_some());

        float_cmp::assert_approx_eq!(f32, i.unwrap().t(), i1.t());
        assert!(ptr::addr_eq(i.unwrap().obj(), i1.obj()));
    }

    // Ray intersects with both sides of a sphere
    // One intersection is positive, the other is negative
    // i2 is the positive intersection
    #[test]
    fn two_hit_one_pos_one_neg() {
        let s = Sphere::new();
        let i1 = Intersection::build(-1.0, &s);
        let i2 = Intersection::build(1.0, &s);

        let i = Intersection::hit(&vec![i1, i2]);

        assert!(i.is_some());

        float_cmp::assert_approx_eq!(f32, i.unwrap().t(), i2.t());
        assert!(ptr::addr_eq(i.unwrap().obj(), i2.obj()));
    }

    // Ray intersects with both sides of a sphere
    // Both intersections are negative
    #[test]
    fn two_hit_all_negative() {
        let s = Sphere::new();
        let i1 = Intersection::build(-2.0, &s);
        let i2 = Intersection::build(-1.0, &s);

        let i = Intersection::hit(&vec![i1, i2]);

        assert!(i.is_none());
    }

    // Multiple intersections with a sphere
    // One is negative, the others are positive
    // i4 is the closest intersection
    #[test]
    fn multiple_hit() {
        let s = Sphere::new();
        let i1 = Intersection::build(5.0, &s);
        let i2 = Intersection::build(7.0, &s);
        let i3 = Intersection::build(-3.0, &s);
        let i4 = Intersection::build(2.0, &s);

        let i = Intersection::hit(&vec![i1, i2, i3, i4]);

        assert!(i.is_some());

        float_cmp::assert_approx_eq!(f32, i.unwrap().t(), i4.t());
        assert!(ptr::addr_eq(i.unwrap().obj(), i4.obj()));
    }
}

#[cfg(test)]
mod ray_transform_test {
    extern crate nalgebra_glm as glm;

    use crate::ray::Ray;

    // Checks if a ray @ (1, 2, 3) is properly translated to (4, 6, 8)
    #[test]
    fn ray_translate() {
        let r = Ray::build(&glm::vec3(1.0, 2.0, 3.0), &glm::vec3(0.0, 1.0, 0.0));
        let m = glm::translation(&glm::vec3(3.0, 4.0, 5.0));

        let r2 = Ray::transform(&r, &m);

        assert_eq!(
            glm::vec3(true, true, true),
            glm::equal(&r2.origin, &glm::vec3(4.0, 6.0, 8.0))
        );

        assert_eq!(
            glm::vec3(true, true, true),
            glm::equal(&r2.direction, &glm::vec3(0.0, 1.0, 0.0))
        );
    }

    // Checks if a ray at (1, 2, 3) pointing at (0, 1, 0) is properly scaled to (2, 6, 12) and pointing at (0, 3, 0)
    #[test]
    fn ray_scale() {
        let r = Ray::build(&glm::vec3(1.0, 2.0, 3.0), &glm::vec3(0.0, 1.0, 0.0));
        let m = glm::scaling(&glm::vec3(2.0, 3.0, 4.0));
        let r2 = Ray::transform(&r, &m);

        assert_eq!(
            glm::vec3(true, true, true),
            glm::equal(&r2.origin, &glm::vec3(2.0, 6.0, 12.0))
        );

        assert_eq!(
            glm::vec3(true, true, true),
            glm::equal(&r2.direction, &glm::vec3(0.0, 3.0, 0.0))
        );
    }
}

#[cfg(test)]
mod sphere_transform_test {
    extern crate nalgebra_glm as glm;

    use crate::sphere::Sphere;
    use crate::ray::Ray;
    use crate::intersection::Intersection;
    use crate::hittable::Hittable;

    #[test]
    fn sphere_default_transform() {
        let s = Sphere::new();

        assert!(glm::Mat4::eq(&s.transform, &glm::Mat4::identity()))
    }

    #[test]
    fn sphere_translate() {
        let t = glm::translation(&glm::vec3(2.0, 3.0, 4.0));
        let s = Sphere::build(&glm::Vec3::zeros(), 1.0, &t);

        assert!(glm::Mat4::eq(&s.transform, &t));
    }

    #[test]
    fn scaled_sphere_intersect() {
        let r = Ray::build(&glm::vec3(0.0, 0.0, -5.0), &glm::vec3(0.0, 0.0, 1.0));
        let s = Sphere::build(&glm::Vec3::zeros(), 1.0, &glm::scaling(&glm::vec3(2.0, 2.0, 2.0)));

        let xs: Vec<Intersection> = s.intersect(&r);

        assert_eq!(xs.len(), 2);
        float_cmp::assert_approx_eq!(f32, xs[0].t(), 3.0);
        float_cmp::assert_approx_eq!(f32, xs[1].t(), 7.0);
    }

    #[test]
    fn translated_sphere_intersect() {
        let r = Ray::build(&glm::vec3(0.0, 0.0, -5.0), &glm::vec3(0.0, 0.0, 1.0));
        let s = Sphere::build(&glm::Vec3::zeros(), 1.0, &glm::translation(&glm::vec3(5.0, 0.0, 0.0)));

        let xs: Vec<Intersection> = s.intersect(&r);

        assert_eq!(xs.len(), 0);
    }
}
