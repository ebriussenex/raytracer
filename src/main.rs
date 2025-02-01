mod camera;
mod core;
mod scene;
mod utils;

use core::{point3::Point, rgb::Rgb};
use std::{f64::consts::PI, rc::Rc};

use camera::camera::RenderError;
use scene::{
    hittable::{Hittable, Scene},
    material::{Dielectric, Lambertian, Material, Metal},
    sphere::Sphere,
};

use crate::camera::camera::Camera;

fn main() {
    let img_width: u32 = 1920;
    let ratio = 16.0 / 9.0;
    let lookfrom = Point::new(-2.0, 2.0, 1.0);
    let lookat = Point::new(0.0, 0.0, -1.0);
    let vup = Point::new(0.0, 1.0, 0.0);

    let c = Camera::new(
        Some(lookfrom),
        Some(lookat),
        Some(vup),
        img_width,
        ratio,
        Some(10),
        PI / 6.0,
    );

    let R = f64::cos(PI / 4.0);
    let lambert1: Rc<dyn Material> = Rc::new(Lambertian::new(Rgb::new(1.0, 1.0, 0.0), 1.0));
    let lambert2: Rc<dyn Material> = Rc::new(Lambertian::new(Rgb::new(0.0, 1.0, 1.0), 1.0));

    let mut scene2 = Scene::default();

    [
        Sphere::new(R, Point::new(-R, 0.0, -1.0), lambert1),
        Sphere::new(R, Point::new(R, 0.0, -1.0), lambert2),
    ]
    .into_iter()
    .map(|x| Rc::new(x) as Rc<dyn Hittable>)
    .for_each(|x| scene2.add(x));

    let mat_ground: Rc<dyn Material> = Rc::new(Lambertian::new(Rgb::new(0.8, 0.8, 0.1), 1.0));
    let mat_center: Rc<dyn Material> = Rc::new(Lambertian::new(Rgb::new(0.7, 0.2, 0.8), 1.0));
    let mat_left: Rc<dyn Material> = Rc::new(Dielectric::new(1.5));
    let mat_left_bubble: Rc<dyn Material> = Rc::new(Dielectric::new(1.0 / 1.5));
    let mat_right: Rc<dyn Material> = Rc::new(Metal::new(Rgb::new(0.8, 0.6, 0.2), Some(0.5)));

    let mut scene = Scene::default();

    [
        Sphere::new(100.0, Point::new(0.0, -100.5, -1.0), mat_ground),
        Sphere::new(0.5, Point::new(0.0, 0.0, -1.2), mat_center),
        Sphere::new(0.5, Point::new(-1.0, 0.0, -1.0), mat_left),
        Sphere::new(0.3, Point::new(-1.0, 0.0, -1.0), mat_left_bubble),
        Sphere::new(0.4, Point::new(1.0, 0.0, -1.0), mat_right),
    ]
    .into_iter()
    .map(|x| Rc::new(x) as Rc<dyn Hittable>)
    .for_each(|x| scene.add(x));

    if let Err(e) = c.render(&scene) {
        eprint!("render error: ");
        match e {
            RenderError::WriteHeader(e) => eprintln!("error writing P3 header {e}"),
            RenderError::WritePx(e) => eprintln!("error writing P3 pixel {e}"),
        }
    }
}
