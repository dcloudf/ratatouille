use crate::vec3::Vec3;
use std::{
    fs::File,
    io::{BufWriter, Result, Write},
};

pub fn write_color(writer: &mut BufWriter<File>, color: Vec3) -> Result<usize> {
    writer.write(format!("{} {} {}\n", color.x(), color.y(), color.z()).as_bytes())
}
