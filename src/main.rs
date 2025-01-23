pub mod camera;
pub mod point3;
pub mod ray;
pub mod rgb;

use std::{
    io::{self, Write},
    process::exit,
};

use point3::Point;
use ray::Ray;
use rgb::Rgb;

const RATIO: u32 = 2;

fn blend_colorizer(r: &Ray) -> Rgb {
    let unit_dir = r.dir().unit();
    let a = 0.5 * (unit_dir.y() + 1.0);
    Rgb::new(1.0, 1.0, 1.0) * (1.0 - a) + Rgb::new(0.5, 0.7, 1.0) * a
}

fn main() {
    let img_width: u32 = 400;

    let img_height: u32 = if img_width / RATIO < 1 {
        1
    } else {
        img_width / RATIO
    };

    // camera
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (img_width as f64 / img_height as f64);
    // focal length is a distance between camera and viewport
    let focal_len = 1.0;
    let camera_center = Point::new(0.0, 0.0, 0.0);

    // vectors which describe viewport
    let vp_h = Point::new(0.0, -viewport_height, 0.0);
    let vp_w = Point::new(viewport_width, 0.0, 0.0);

    // pixel delta is a size between 2 pixels in each w and h direction
    let px_dw = vp_w / img_width as f64;
    let px_dh = vp_h / img_height as f64;

    // vector in 3d which points to upper left pixel
    let upleft_px_loc = camera_center - Point::new(0.0, 0.0, focal_len) - (vp_w + vp_h) * 0.5;
    assert_eq!(
        upleft_px_loc,
        Point::new(
            (vp_w.x() + vp_h.x()) * (-0.5),
            (vp_w.y() + vp_h.y()) * (-0.5),
            -focal_len,
        )
    );
    // (0, 0) pixel vector
    let px00_loc = upleft_px_loc + (px_dw + px_dh) * 0.5;

    if let Err(e) =
        io::stdout().write_all(format!("P3\n{} {}\n255\n", img_width, img_height).as_bytes())
    {
        print!("failed to write P3 header: {e}, exiting");
        exit(1);
    }

    (0..img_height).for_each(|j| {
        (0..img_width).for_each(|i| {
            let px_center = px00_loc + (px_dw * i as f64) + (px_dh * j as f64);
            let ray_dir = px_center - camera_center;
            let ray = Ray::new(camera_center, ray_dir);
            let _default_colorizer = |_ray: &Ray| Rgb::new(0.0, 0.0, 0.0);
            let px_color = ray.color(blend_colorizer);
            px_color.write(io::stdout()).unwrap();
        });
    });
}
