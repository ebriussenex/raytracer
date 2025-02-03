mod camera;
mod core;
mod scene;
mod utils;

use core::{point3::Point, rgb::Rgb};
use std::{f64::consts::PI, rc::Rc};

use camera::camera::{InitError, RenderError};
use rand::Rng;
use scene::{
    hittable::{Hittable, Scene},
    material::{Dielectric, Lambertian, Material, Metal},
    sphere::Sphere,
};

use crate::camera::camera::Camera;

fn main() {
    let img_width: u32 = 900;
    let ratio = 16.0 / 9.0;
    let lookfrom = Point::new(13.0, 2.0, 3.0);
    let _lookfrom = Point::default();
    let lookat = Point::new(0.0, 0.0, 0.0);
    let vup = Point::new(0.0, 1.0, 0.0);

    let c = Camera::build(
        Some(lookfrom),
        Some(lookat),
        Some(vup),
        img_width,
        ratio,
        Some(50),
        Some(20.0_f64.to_radians()),
        Some(4.0),
        Some(0.6_f64.to_radians()),
    );

    let c = match c {
        Ok(value) => value,
        Err(InitError::Antialiaser(e)) => {
            eprint!("antialiaser initialization failure:");
            match e {
                rand::distr::uniform::Error::EmptyRange => {
                    eprintln!("empty range provided to rand::uniform");
                }
                rand::distr::uniform::Error::NonFinite => {
                    eprintln!("non finite range provided to rand::uniform");
                }
            }
            return;
        }
    };

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
        Sphere::new(0.5, Point::new(0.0, 0.0, -3.2), mat_center.clone()),
        Sphere::new(0.5, Point::new(-1.0, 0.0, -2.5), mat_left),
        Sphere::new(0.3, Point::new(-1.0, 0.0, -2.5), mat_left_bubble),
        Sphere::new(0.4, Point::new(1.0, 0.0, -1.0), mat_right.clone()),
        Sphere::new(0.4, Point::new(6.0, 0.0, -10.0), mat_right.clone()),
        Sphere::new(0.5, Point::new(-3.0, 0.0, -1.6), mat_center.clone()),
    ]
    .into_iter()
    .map(|x| Rc::new(x) as Rc<dyn Hittable>)
    .for_each(|x| scene.add(x));

    let scene3 = prepare_scene();

    if let Err(e) = c.render(&scene3) {
        eprint!("render error: ");
        match e {
            RenderError::WriteHeader(e) => eprintln!("error writing P3 header {e}"),
            RenderError::WritePx(e) => eprintln!("error writing P3 pixel {e}"),
        }
    }
}

fn prepare_scene() -> Scene {
    let mut scene = Scene::default();

    let ground_material: Rc<dyn Material> = Rc::new(Lambertian::new(Rgb::new(0.7, 0.4, 0.4), 1.0));
    let ground_sphere: Rc<dyn Hittable> = Rc::new(Sphere::new(
        1000.0,
        Point::new(0.0, -1000.0, 0.0),
        ground_material,
    ));

    (-11..11).for_each(|a| {
        (-11..11).for_each(|b| {
            let mut rng = rand::rng();
            let choose_mat = rng.random_range(0.0..=1.0);
            let center = Point::new(
                a as f64 + 0.9 * rng.random_range(0.0..=1.0),
                0.2,
                b as f64 + 0.9 * rng.random_range(0.0..=1.0),
            );

            let glass_mat: Rc<dyn Material> = Rc::new(Dielectric::new(1.5));
            let glass_sphere: Rc<dyn Hittable> = Rc::new(Sphere::new(
                1.0,
                Point::new(0.0, 1.0, 0.0),
                Rc::clone(&glass_mat),
            ));

            let yellow = Rgb::new(1.0, 1.0, 0.3);
            let yellow_lambert: Rc<dyn Material> = Rc::new(Lambertian::new(yellow, 1.0));
            let yellow_lamber_sphere: Rc<dyn Hittable> = Rc::new(Sphere::new(
                1.0,
                Point::new(-4.0, 1.0, 0.0),
                Rc::clone(&yellow_lambert),
            ));

            let metallic: Rc<dyn Material> = Rc::new(Metal::new(Rgb::new(0.7, 0.6, 0.5), None));
            let metallic_sphere: Rc<dyn Hittable> = Rc::new(Sphere::new(
                1.0,
                Point::new(4.0, 1.0, 0.0),
                Rc::clone(&metallic),
            ));

            [glass_sphere, yellow_lamber_sphere, metallic_sphere]
                .into_iter()
                .for_each(|x| scene.add(x));

            if (center - Point::new(4.0, 0.2, 0.0)).size() > 0.9 {
                let mat: Rc<dyn Material> = if choose_mat < 0.8 {
                    let albedo = Rgb::random(&mut rng) * Rgb::random(&mut rng);
                    let reflectance = rng.random_range(0.8..=1.0);
                    Rc::new(Lambertian::new(albedo, reflectance))
                } else if choose_mat < 0.95 {
                    let albedo = Rgb::random_with_interval(&mut rng, 0.5..=1.0);
                    let fuzz = rng.random_range(0.0..0.5);
                    Rc::new(Metal::new(albedo, Some(fuzz)))
                } else {
                    Rc::clone(&glass_mat)
                };
                scene.add(Rc::new(Sphere::new(0.2, center, mat)));
            }
        });
        scene.add(Rc::clone(&ground_sphere));
    });

    scene
}
