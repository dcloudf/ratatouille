use crate::interval::Interval;
use crate::vec3::Vec3;
use std::{
    fs::File,
    io::{BufWriter, Result, Write},
};

pub(crate) fn linear_to_gamma(linear_component: f64) -> f64 {
    match linear_component > 0. {
        true => linear_component.sqrt(),
        false => 0.,
    }
}

pub(crate) fn write_color(writer: &mut BufWriter<File>, pixel_color: Vec3) -> Result<()> {
    let r = linear_to_gamma(pixel_color.x());
    let g = linear_to_gamma(pixel_color.y());
    let b = linear_to_gamma(pixel_color.z());

    let intensity = Interval::new(0., 0.999);
    let rbyte = (256. * intensity.clamp(r)) as i32;
    let gbyte = (256. * intensity.clamp(g)) as i32;
    let bbyte = (256. * intensity.clamp(b)) as i32;
    writer.write_all(format!("{rbyte} {gbyte} {bbyte}\n").as_bytes())
}
