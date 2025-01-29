mod camera;
mod core;
mod scene;
mod utils;

use core::{point3::Point, rgb::Rgb};
use std::rc::Rc;

use camera::camera::RenderError;
use scene::{
    hittable::{Hittable, Scene},
    material::{Lambertian, Material, Metal},
    sphere::Sphere,
};

use crate::camera::camera::Camera;

fn main() {
    let img_width: u32 = 800;
    let ratio = 16.0 / 9.0;

    let c = Camera::new(Point::new(0.0, 0.0, 0.0), img_width, ratio, 1.0, Some(10));

    let mat_ground: Rc<dyn Material> = Rc::new(Lambertian::new(Rgb::new(0.8, 0.8, 0.1), 1.0));
    let mat_center: Rc<dyn Material> = Rc::new(Lambertian::new(Rgb::new(0.7, 0.2, 0.8), 1.0));
    let mat_left: Rc<dyn Material> = Rc::new(Metal::new(Rgb::new(0.8, 0.8, 0.8), None));
    let mat_right: Rc<dyn Material> = Rc::new(Metal::new(Rgb::new(0.8, 0.6, 0.2), Some(0.5)));

    let mut scene = Scene::default();

    [
        Sphere::new(100.0, Point::new(0.0, -100.5, -1.0), mat_ground),
        Sphere::new(0.5, Point::new(0.0, 0.0, -1.2), mat_center),
        Sphere::new(0.5, Point::new(-1.0, 0.0, -1.0), mat_left),
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
