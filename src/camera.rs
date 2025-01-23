use crate::point3::Point;

const VIEWPORT_HEIGHT: f64 = 2.0;
// Camera represents abstraction over view on objects through pixel-viewport
// upleft_px_pos, px00_pos thus depend on camera pos, those should be updated on each camera pos
// changes.
// px_dw, px_dh is constant after creation of camera, while those depend on chosen arbitrary
// viewport size.
pub struct Camera {
    pub pos: Point,
    px_dw: Point,
    px_dh: Point,
    px00_pos: Point,
    upleft_px_pos: Point,
}

impl Camera {
    pub fn new(pos: Point, img_width: u32, ratio: u32, focal_len: f64) -> Self {
        let img_height: u32 = if img_width / ratio < 1 {
            1
        } else {
            img_width / ratio
        };

        // viewport, arbitrary size in virtual units
        let vp_w = VIEWPORT_HEIGHT * (img_width as f64 / img_height as f64);
        // viewport vectors
        let vpv_h = Point::new(0.0, -VIEWPORT_HEIGHT, 0.0);
        let vpv_w = Point::new(vp_w, 0.0, 0.0);

        // pixel spacing, pixel delta
        let px_dw = vpv_w / img_width as f64;
        let px_dh = vpv_h / img_height as f64;

        // upper left pixel in viewport
        let upleft_px_pos = pos - Point::new(0.0, 0.0, focal_len) - (vpv_w + vpv_h) * 0.5;
        // (0,0) pixel pos
        let px00_pos = upleft_px_pos + (px_dw + px_dh) * 0.5;

        Camera {
            pos,
            px_dw,
            px_dh,
            px00_pos,
            upleft_px_pos,
        }
    }
}
