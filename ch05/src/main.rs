mod hittable;
mod intersection;
mod ray;
mod sphere;
mod tests;

use minifb::{Key, Window, WindowOptions};
extern crate nalgebra_glm as glm;

use crate::sphere::Sphere;
use crate::ray::Ray;
use crate::intersection::Intersection;
use crate::hittable::Hittable;

const WIDTH: usize = 100;
const HEIGHT: usize = 100;

fn main() -> std::io::Result<()> {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .expect("Unable to create the window.\n");

    // Limit to max ~60 fps update rate
    window.set_target_fps(60);

    let s = Sphere::new();
    let mut r = Ray::build(
        &glm::vec3(0.0, 0.0, -5.0),
        &(glm::vec3(0.0, 0.0, 1.0) - glm::vec3(0.0, 0.0, -5.0)).normalize(),
    );

    let wall_z = 10.0;
    let wall_size: f32 = 7.0;
    let pixel_h_size: f32 = wall_size / WIDTH as f32;
    let pixel_v_size: f32 = wall_size / HEIGHT as f32;
    let half: f32 = wall_size / 2.0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for i in 0..WIDTH {
            let world_y = half - pixel_h_size * i as f32;

            for j in 0..HEIGHT  {
                let world_x = -half + pixel_v_size * j as f32;

                r.direction = (glm::vec3(world_x, world_y, wall_z) - r.origin).normalize();

                if Intersection::hit(&s.intersect(&r)).is_some() {
                    buffer[i + j * WIDTH] = 0x00FF0000;
                }
                else {
                    buffer[i + j * WIDTH] = 0;
                }
            }
        }

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }

    Ok(())
}
