use std::io::{self, Write};

const WIDTH: u32 = 256;
const HEIGHT: u32 = 256;

fn main() {
    if let Err(e) = io::stdout().write_all(format!("P3\n{} {}\n255\n", WIDTH, HEIGHT).as_bytes()) {
        print!("failed to write P3 header: {e}");
    }

    (0..WIDTH).for_each(|i| {
        (0..HEIGHT).for_each(|j| {
            let r = i as f32 / (WIDTH as f32 - 1.0);
            let g = j as f32 / (HEIGHT as f32 - 1.0);
            let b: f32 = 0.0;

            let [ir, ig, ib]: [u8; 3] = [r, g, b].map(|x| (x * 255.999) as u8);

            io::stdout()
                .write_all(format!("{ir} {ig} {ib}\n").as_bytes())
                .unwrap();
        });
    });
}
