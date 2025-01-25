mod camera;
mod core;
mod scene;

use core::{point3::Point, ray::Ray, rgb::Rgb};
use std::{
    io::{self, Write},
    process::exit,
    rc::Rc,
};

use scene::{
    hittable::{Hittable, Scene},
    sphere::Sphere,
};

use crate::camera::camera::Camera;

const RATIO: u32 = 2;

fn blend_colorizer(scene: &Scene, r: &Ray) -> Rgb {
    if let Some(rec) = scene.hit(r, 0.0, f64::INFINITY) {
        let n = rec.n;
        (Rgb::new(n.x(), n.y(), n.z()) + Rgb::new(1.0, 1.0, 1.0)) * 0.5
    } else {
        let unit_dir = r.dir().unit();
        let a = 0.5 * (unit_dir.y() + 1.0);
        Rgb::new(1.0, 1.0, 1.0) * (1.0 - a) + Rgb::new(0.5, 0.7, 1.0) * a
    }
}

fn main() {
    let img_width: u32 = 800;
    let img_height: u32 = if img_width / RATIO < 1 {
        1
    } else {
        img_width / RATIO
    };

    let c = Camera::new(Point::new(0.0, 0.0, 0.0), img_width, img_height, 1.0);
    let sphere_a: Rc<dyn Hittable> = Rc::new(Sphere::new(0.5, Point::new(0.0, 0.0, -1.0)));
    let sphere_b: Rc<dyn Hittable> = Rc::new(Sphere::new(100.0, Point::new(0.0, -100.5, -1.0)));
    let mut scene = Scene::default();
    scene.add(Rc::clone(&sphere_a));
    scene.add(Rc::clone(&sphere_b));

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
            let with_scene = |r| blend_colorizer(&scene, r);
            let px_color = ray.color(with_scene);
            px_color.write(io::stdout()).unwrap();
        });
    });
}
