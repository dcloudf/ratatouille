use std::{fs::File, io::Write, path::Path};

fn main() {
    let image_width = 256;
    let image_heigth = 256;

    let path = Path::new("test.ppm");
    let mut file = File::create(&path).unwrap();
    let _ = file.write(format!("P3\n{} {}\n255\n", image_width, image_heigth).as_bytes());
    for j in 0..image_heigth {
        for i in 0..image_width {
            let _ = file
                .write(
                    format!(
                        "{} {} {}\n",
                        (256 * i / (image_width - 1)) as i64,
                        (256 * j / (image_heigth - 1)) as i64,
                        256 * 0
                    )
                    .as_bytes(),
                )
                .unwrap();
        }
    }
}
