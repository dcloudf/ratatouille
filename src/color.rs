use crate::interval::Interval;
use crate::vec3::Vec3;
use std::{
    fs::File,
    io::{BufWriter, Result, Write},
};

pub(crate) fn write_color(writer: &mut BufWriter<File>, pixel_color: Vec3) -> Result<()> {
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    let intensity = Interval::new(0., 0.999);
    let rbyte = (256. * intensity.clamp(r)) as i32;
    let gbyte = (256. * intensity.clamp(g)) as i32;
    let bbyte = (256. * intensity.clamp(b)) as i32;
    writer.write_all(format!("{rbyte} {gbyte} {bbyte}\n").as_bytes())
}
