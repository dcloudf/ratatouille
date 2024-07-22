use std::fs::File;
use std::io::{self, BufWriter, Write};

pub mod vec3;

fn main() -> io::Result<()> {
    let image_width = 256 as usize;
    let image_height = 256 as usize;
    let blue = 0 as f32;

    let file = File::create("image.ppm")?;
    let mut writer = BufWriter::new(file);
    writer.write(format!("P3\n{} {}\n255\n", image_width, image_height).as_bytes())?;

    for j in 0..image_height {
        for i in 0..image_width {
            writer.write(format!("{} {} {}\n", i, j, blue).as_bytes())?;
        }
    }
    Ok(())
}
