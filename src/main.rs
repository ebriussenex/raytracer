pub mod camera;
pub mod point3;
pub mod ray;
pub mod rgb;
pub mod sphere;

use std::{
    io::{self, Write},
    process::exit,
};

use camera::Camera;
use point3::Point;
use ray::{Colorizer, Ray};
use rgb::Rgb;
use sphere::Sphere;

const RATIO: u32 = 2;

// TODO: more allocation, same could be done with no box, and with impl
fn with_sphere(sphere: Sphere) -> Colorizer {
    Box::new(move |r| blend_colorizer(sphere, r))
}

fn blend_colorizer(sphere: Sphere, r: &Ray) -> Rgb {
    if sphere.hit(r) {
        Rgb::new(1.0, 0.0, 0.0)
    } else {
        let unit_dir = r.dir().unit();
        let a = 0.5 * (unit_dir.y() + 1.0);
        Rgb::new(1.0, 1.0, 1.0) * (1.0 - a) + Rgb::new(0.5, 0.7, 1.0) * a
    }
}

fn main() {
    let img_width: u32 = 400;
    let img_height: u32 = if img_width / RATIO < 1 {
        1
    } else {
        img_width / RATIO
    };

    let c = Camera::new(Point::new(0.0, 0.0, 0.0), img_width, img_height, 1.0);
    let sphere = Sphere::new(0.5, Point::new(0.0, 0.0, -1.0));

    if let Err(e) =
        io::stdout().write_all(format!("P3\n{} {}\n255\n", img_width, img_height).as_bytes())
    {
        print!("failed to write P3 header: {e}, exiting");
        exit(1);
    }

    (0..img_height).for_each(|j| {
        (0..img_width).for_each(|i| {
            let px_center =
                c.upper_left_pixel_center() + (c.px_dw * i as f64) + (c.px_dh * j as f64);
            let ray_dir = px_center - c.pos();
            let ray = Ray::new(c.pos(), ray_dir);
            let _default_colorizer = |_ray: &Ray| Rgb::new(0.0, 0.0, 0.0);
            let px_color = ray.color(with_sphere(sphere));
            px_color.write(io::stdout()).unwrap();
        });
    });
}
