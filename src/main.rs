mod camera;
mod core;
mod scene;
mod utils;

use core::{point3::Point, rgb::ARgb};
use std::{
    f64::consts::PI,
    path::{Path, PathBuf},
    sync::Arc,
};

use camera::camera::{InitError, RenderError};
use rand::Rng;
use scene::{
    hittable::{Hittable, Scene},
    image_loader::load_image_to_rgb,
    material::{Dielectric, Lambertian, Material, Metal},
    sphere::Sphere,
    texture::{CheckerTexture, ImageTexture},
};

use crate::camera::camera::Camera;

const MARS_TEXTURE: &str = "mars_1k_color.jpg";
const EARTH_TEXTURE: &str = "earthmap.jpg";
const TEXTURES_PATH: &str = "./presets/textures/";

fn main() {
    let img_width: u32 = 400;
    let ratio = 16.0 / 9.0;
    let lookfrom = Point::new(13.0, 2.0, 3.0);
    let _lookfrom = Point::default();
    let lookat = Point::new(0.0, 0.0, 0.0);
    let _lookat = Point::new(0.0, 0.0, -1.0);
    let vup = Point::new(0.0, 1.0, 0.0);
    let ray_bounce_depth: u32 = 50;
    let vfov = 25.0_f64.to_radians();
    let aa_samples_per_px = Some(100);
    let defocus_angle = 0.4_f64.to_radians();
    let focus_dist = 10.0;

    let look_from_mars = Point::new(0.0, 0.0, 12.0);

    let c = Camera::build(
        Some(look_from_mars),
        Some(lookat),
        Some(vup.unit()),
        img_width,
        ratio,
        aa_samples_per_px,
        Some(vfov),
        Some(focus_dist),
        Some(defocus_angle),
        Some(ray_bounce_depth),
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

    let r = f64::cos(PI / 4.0);
    let lambert1: Arc<dyn Material> = Arc::new(Lambertian::new(ARgb::new(1.0, 1.0, 0.0), 1.0));
    let lambert2: Arc<dyn Material> = Arc::new(Lambertian::new(ARgb::new(0.0, 1.0, 1.0), 1.0));

    let mut scene2 = Scene::default();

    [
        Sphere::new_static(r, Point::new(-r, 0.0, -1.0), lambert1),
        Sphere::new_static(r, Point::new(r, 0.0, -1.0), lambert2),
    ]
    .into_iter()
    .map(|x| Arc::new(x) as Arc<dyn Hittable>)
    .for_each(|x| scene2.add(&x));

    let mat_ground: Arc<dyn Material> = Arc::new(Lambertian::new(ARgb::new(0.8, 0.8, 0.1), 1.0));
    let mat_center: Arc<dyn Material> = Arc::new(Lambertian::new(ARgb::new(0.7, 0.2, 0.8), 1.0));
    let mat_left: Arc<dyn Material> = Arc::new(Dielectric::new(1.5));
    let mat_left_bubble: Arc<dyn Material> = Arc::new(Dielectric::new(1.0 / 1.5));
    let mat_right: Arc<dyn Material> = Arc::new(Metal::new(ARgb::new(0.8, 0.6, 0.2), Some(0.5)));

    let mut scene = Scene::default();

    [
        Sphere::new_static(100.0, Point::new(0.0, -100.5, -1.0), mat_ground.clone()),
        Sphere::new_static(0.5, Point::new(0.0, 0.0, -3.2), mat_center.clone()),
        Sphere::new_static(0.5, Point::new(-1.0, 0.0, -2.5), mat_left.clone()),
        Sphere::new_static(0.3, Point::new(-1.0, 0.0, -2.5), mat_left_bubble.clone()),
        Sphere::new_static(0.4, Point::new(1.0, 0.0, -1.0), mat_right.clone()),
        Sphere::new_static(0.4, Point::new(6.0, 0.0, -10.0), mat_right.clone()),
        Sphere::new_static(0.5, Point::new(-3.0, 0.0, -1.6), mat_center.clone()),
    ]
    .into_iter()
    .map(|x| Arc::new(x) as Arc<dyn Hittable>)
    .for_each(|x| scene.add(&x));

    let mut blur_scene = Scene::default();
    [
        Sphere::new_static(100.0, Point::new(0.0, -100.5, -1.0), mat_ground.clone()),
        Sphere::new_static(0.5, Point::new(0.0, 0.0, -1.2), mat_center.clone()),
        Sphere::new_static(0.5, Point::new(-1.0, 0.0, -1.0), mat_left.clone()),
        Sphere::new_static(0.3, Point::new(-1.0, 0.0, -1.0), mat_left_bubble.clone()),
        Sphere::new_static(0.4, Point::new(1.0, 0.0, -1.0), mat_right.clone()),
        Sphere::new_static(0.4, Point::new(6.0, 0.0, -10.0), mat_right.clone()),
        Sphere::new_static(0.5, Point::new(-3.0, 0.0, -1.6), mat_center.clone()),
    ]
    .into_iter()
    .map(|x| Arc::new(x) as Arc<dyn Hittable>)
    .for_each(|x| blur_scene.add(&x));

    let mut scene3 = bouncing_balls_scene();
    scene3.build_bvh();

    let mars_scene = mars_texture_scene();
    if let Err(e) = c.render(&mars_scene) {
        eprint!("render error: ");
        match e {
            RenderError::WriteHeader(e) => eprintln!("error writing P3 header {e}"),
            RenderError::WritePx(e) => eprintln!("error writing P3 pixel {e}"),
        }
    }
}

fn mars_texture_scene() -> Scene {
    let mut scene = Scene::default();
    let mut texture_path = PathBuf::from(TEXTURES_PATH);
    texture_path.push(EARTH_TEXTURE);
    let rgb_image = load_image_to_rgb(texture_path).expect("failed to load image");
    let mars_texture = Arc::new(ImageTexture::new(Arc::new(rgb_image)));
    let mars_material = Arc::new(Lambertian::with_texture(&mars_texture, 1.0));

    let mars_surface =
        Arc::new(Sphere::new_static(1.8, Point::default(), mars_material)) as Arc<dyn Hittable>;
    scene.add(&mars_surface);
    scene.build_bvh();
    scene
}

fn bouncing_balls_scene() -> Scene {
    let mut scene = Scene::default();

    let mut texture_path = PathBuf::from(TEXTURES_PATH);
    texture_path.push(MARS_TEXTURE);
    let rgb_image = load_image_to_rgb(texture_path).expect("failed to load image");
    let mars_texture = Arc::new(ImageTexture::new(Arc::new(rgb_image)));
    let mars_material = Arc::new(Lambertian::with_texture(&mars_texture, 1.0));
    let near3 = Point::new(-4.0, 1.0, 0.0);
    let near1 = Point::new(4.0, 1.0, 0.0);
    let near2 = Point::new(0.0, 1.0, 0.0);

    let mars_surface = Arc::new(Sphere::new_static(1.2, near1, mars_material)) as Arc<dyn Hittable>;

    let checker_tx = Arc::new(CheckerTexture::new(
        0.32,
        ARgb::new(0.2, 0.3, 0.1),
        ARgb::new(0.9, 0.9, 0.9),
    ));
    let ground_material: Arc<dyn Material> = Arc::new(Lambertian::with_texture(checker_tx, 1.0));
    let ground_sphere: Arc<dyn Hittable> = Arc::new(Sphere::new_static(
        1000.0,
        Point::new(0.0, -1000.0, 0.0),
        ground_material,
    ));

    let glass_mat: Arc<dyn Material> = Arc::new(Dielectric::new(1.5));
    let glass_sphere: Arc<dyn Hittable> =
        Arc::new(Sphere::new_static(1.0, near2, Arc::clone(&glass_mat)));

    let yellow = ARgb::new(1.0, 1.0, 0.3);
    let yellow_lambert: Arc<dyn Material> = Arc::new(Lambertian::new(yellow, 1.0));
    let yellow_lamber_sphere: Arc<dyn Hittable> = Arc::new(Sphere::new_static(
        1.0,
        Point::new(-4.0, 1.0, 0.0),
        Arc::clone(&yellow_lambert),
    ));

    let metallic: Arc<dyn Material> = Arc::new(Metal::new(ARgb::new(0.7, 0.6, 0.5), None));
    let metallic_sphere: Arc<dyn Hittable> =
        Arc::new(Sphere::new_static(1.0, near3, Arc::clone(&metallic)));

    [glass_sphere, mars_surface, metallic_sphere]
        .into_iter()
        .for_each(|x| scene.add(&x));

    (-11..11).for_each(|a| {
        (-11..11).for_each(|b| {
            let mut rng = rand::rng();
            let choose_mat = rng.random_range(0.0..=1.0);
            let center = Point::new(
                f64::from(a) + 0.9 * rng.random_range(0.0..=1.0),
                0.2,
                f64::from(b) + 0.9 * rng.random_range(0.0..=1.0),
            );

            if (center - Point::new(4.0, 0.2, 0.0)).size() > 0.9 {
                let mut center2 = None;
                let mat: Arc<dyn Material> = if choose_mat < 0.8 {
                    let albedo = ARgb::random(&mut rng) * ARgb::random(&mut rng);
                    let reflectance = rng.random_range(0.8..=1.0);
                    center2 = Some(center + Point::new(0.0, rng.random_range(0.0..=0.5), 0.0));
                    Arc::new(Lambertian::new(albedo, reflectance))
                } else if choose_mat < 0.95 {
                    let albedo = ARgb::random_with_interval(&mut rng, 0.5..=1.0);
                    let fuzz = rng.random_range(0.0..0.5);
                    Arc::new(Metal::new(albedo, Some(fuzz)))
                } else {
                    Arc::clone(&glass_mat)
                };
                let temp: Arc<dyn Hittable> = Arc::new(Sphere::new(0.2, center, center2, mat));
                scene.add(&temp);
            }
        });
        scene.add(&Arc::clone(&ground_sphere));
    });

    scene
}
