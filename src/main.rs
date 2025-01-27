mod camera;
mod core;
mod scene;

use core::point3::Point;
use std::rc::Rc;

use camera::camera::RenderError;
use scene::{
    hittable::{Hittable, Scene},
    sphere::Sphere,
};

use crate::camera::camera::Camera;

fn main() {
    let img_width: u32 = 800;
    let ratio = 16.0 / 9.0;

    let c = Camera::new(Point::new(0.0, 0.0, 0.0), img_width, ratio, 1.0);
    let sphere_a: Rc<dyn Hittable> = Rc::new(Sphere::new(0.5, Point::new(0.0, 0.0, -1.0)));
    let sphere_b: Rc<dyn Hittable> = Rc::new(Sphere::new(100.0, Point::new(0.0, -100.5, -1.0)));
    let mut scene = Scene::default();
    scene.add(Rc::clone(&sphere_a));
    scene.add(Rc::clone(&sphere_b));
    if let Err(e) = c.render(&scene) {
        eprint!("render error: ");
        match e {
            RenderError::WriteHeader(e) => eprintln!("error writing P3 header {e}"),
            RenderError::WritePx(e) => eprintln!("error writing P3 pixel {e}"),
        }
    }
}
