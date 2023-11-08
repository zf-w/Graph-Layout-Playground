use std::path::Path;
use std::fs::File;
use std::io::BufWriter;

pub struct Img {
  w: u32,
  h: u32,
  data: Vec<u8>,
}

impl Img {
    pub fn new(w: u32, h: u32) -> Self {
      let dim: u32 = 4;
      let mut data: Vec<u8> = Vec::with_capacity((w * h * dim) as usize);
      for i in 0..(w*h*dim) {
        data.push(0)
      }

      Img {w, h, data}
    }

    pub fn save(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
      let path = Path::new(path);
      let file = File::create(path).unwrap();
      let ref mut w = BufWriter::new(file);

      let mut encoder = png::Encoder::new(w, self.w, self.h); // Width is 2 pixels and height is 1.
      encoder.set_color(png::ColorType::Rgba);
      encoder.set_depth(png::BitDepth::Eight);
      encoder.set_source_gamma(png::ScaledFloat::from_scaled(45455)); // 1.0 / 2.2, scaled by 100000
      encoder.set_source_gamma(png::ScaledFloat::new(1.0 / 2.2));     // 1.0 / 2.2, unscaled, but rounded
      let source_chromaticities = png::SourceChromaticities::new(     // Using unscaled instantiation here
          (0.31270, 0.32900),
          (0.64000, 0.33000),
          (0.30000, 0.60000),
          (0.15000, 0.06000)
      );
      encoder.set_source_chromaticities(source_chromaticities);
      let mut writer = encoder.write_header()?;

       // An array containing a RGBA sequence. First pixel is red and second pixel is black.
      writer.write_image_data(&self.data)?; // Save
      Ok(())
    }
}