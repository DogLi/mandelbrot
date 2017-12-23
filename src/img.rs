use std::path::Path;
use image::{Rgb, ImageBuffer};
use super::color;

pub struct Picture<'a> {
    buffer: ImageBuffer<Rgb<u8>, Vec<u8>>,
    path_name: &'a str,
}

impl<'a> Picture<'a> {
    pub fn new(width: usize, height: usize, name: &'a str) -> Self {
        // There's probably a cleaner way to figure out the right size
        let storage = vec![0; color::RGB_DIM * width * height];
        let buf = ImageBuffer::from_raw(width as u32, height as u32, storage).unwrap();

        Picture {
            buffer: buf,
            path_name: name,
        }
    }

    pub fn fill_color(&mut self, pixels: &[Rgb<u8>]) {
        for (index, p) in self.buffer.pixels_mut().enumerate() {
            let color = pixels[index];
            *p = color;
        }
        let path = Path::new(self.path_name);
        match self.buffer.save(path) {
            Ok(()) => {
                println!("create {:?} success!", path);
            }
            Err(e) => {
                println!("create {:?} failed: {:?}!", path, e);
            }
        }
    }
}
