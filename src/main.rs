pub(crate) use color::write_color;
use std::fs::File;
use std::io::{self, BufWriter, Write};
pub(crate) use vec3::Vec3;

pub mod color;
pub mod vec3;

fn main() -> io::Result<()> {
    let image_width = 256 as usize;
    let image_height = 256 as usize;
    let blue = 0 as f64;

    let file = File::create("image.ppm")?;
    let mut writer = BufWriter::new(file);
    writer.write(format!("P3\n{} {}\n255\n", image_width, image_height).as_bytes())?;

    for j in 0..image_height {
        for i in 0..image_width {
            write_color(&mut writer, Vec3::new(i as f64, j as f64, blue));
        }
    }
    Ok(())
}
