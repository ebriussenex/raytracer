use crate::core::point3::Point;

const VIEWPORT_HEIGHT: f64 = 2.0;

// Camera represents abstraction over view on objects through pixel-viewport
// upleft_px_pos, px00_pos thus depend on camera pos, those should be updated on each camera pos
// changes.
// px_dw, px_dh is constant after creation of camera, while those depend on chosen arbitrary
// viewport size.
pub struct Camera {
    pos: Point,
    focal_len: f64,
    vpv_h: Point,
    vpv_w: Point,
    pub px_dw: Point,
    pub px_dh: Point,
}

impl Camera {
    pub fn new(initial_pos: Point, img_width: u32, img_height: u32, focal_len: f64) -> Self {
        // viewport, arbitrary size in virtual units
        let vp_w = VIEWPORT_HEIGHT * (img_width as f64 / img_height as f64);
        // viewport vectors
        let vpv_h = Point::new(0.0, -VIEWPORT_HEIGHT, 0.0);
        let vpv_w = Point::new(vp_w, 0.0, 0.0);

        // pixel spacing, pixel delta
        let px_dw = vpv_w / img_width as f64;
        let px_dh = vpv_h / img_height as f64;
        Camera {
            pos: initial_pos,
            focal_len,
            vpv_w,
            vpv_h,
            px_dw,
            px_dh,
        }
    }

    pub fn pos(&self) -> Point {
        self.pos
    }

    // upper left of viewport
    pub fn vp_upper_left(&self) -> Point {
        self.pos - Point::new(0.0, 0.0, self.focal_len) - (self.vpv_w + self.vpv_h) * 0.5
    }

    // px(0, 0), upper left pixel position
    pub fn upper_left_pixel_center(&self) -> Point {
        self.vp_upper_left() + (self.px_dw + self.px_dh) * 0.5
    }
}
