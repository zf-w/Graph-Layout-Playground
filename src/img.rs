use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::{BufWriter, Write};

pub trait Drawable {
    fn new(w: u32, h: u32) -> Self where Self: Sized;
    fn draw_line_f(&mut self, r0: f32, c0: f32, r1: f32, c1:f32);
    fn save(&self, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>>;
}

pub struct Img {
  w: u32,
  h: u32,
  data: Vec<u8>,
}

impl Img {
  fn get_index(&mut self, row: u32, col: u32) -> u32 {
    row * self.w + col
  }

  fn set_pixel(&mut self, row: u32, col: u32) {
    if row >= self.h || col >= self.w {
      return;
    }
    let offset = self.get_index(row, col);
    // for d in 0..4 {
    self.data[(offset * 4 + 0) as usize] = 255;
    self.data[(offset * 4 + 1) as usize] = 255;
    self.data[(offset * 4 + 2) as usize] = 255;
    self.data[(offset * 4 + 3) as usize] = 255;
    // }
  }

  pub fn draw_line(&mut self, r0: usize, c0: usize, r1: usize, c1: usize) {
    let r_d = (r1 as i32) - (r0 as i32);
    let c_d = (c1 as i32) - (c0 as i32);
    let step = r_d.abs().max(c_d.abs());
    let per_r: f32 = (r_d as f32) / (step as f32);
    let per_c: f32 = (c_d as f32) / (step as f32);

    let mut curr_r = r0 as f32;
    let mut curr_c = c0 as f32;
    for _i in 0..step {
      self.set_pixel(curr_r as u32, curr_c as u32);
      curr_r += per_r;
      curr_c += per_c;
    }
  }
}

impl Drawable for Img {
    fn new(w: u32, h: u32) -> Self where Self: Sized {
      let dim: u32 = 4;
      let mut data: Vec<u8> = Vec::with_capacity((w * h * dim) as usize);
      for _ in 0..(w*h) {
        data.push(230);
        data.push(230);
        data.push(255);
        data.push(0);
      }

      Img {w, h, data}
    }

    fn save(&self, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
      let path = Path::new(path);
      let file = File::create(path)?;
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

      writer.write_image_data(&self.data)?; // Save
      Ok(())
    }

    fn draw_line_f(&mut self, r0: f32, c0: f32, r1: f32, c1:f32) {
      let w = self.w as f32;
      let h = self.h as f32;
      self.draw_line((r0 * w) as usize,
      (c0 * h) as usize,
      (r1 * w) as usize,
      (c1 * h) as usize);
    }
}

pub struct Svg {
  w: u32,
  h: u32,
  lines: String
}

impl Drawable for Svg {
    fn new(w: u32, h: u32) -> Self where Self: Sized {
        Svg {
          w, h,
          lines: String::new()
        }
    }

    fn save(&self, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
      let ans: String = format!(
      r#"
<svg viewBox="0 0 {} {}" xmlns="http://www.w3.org/2000/svg">
  <path stroke="currentColor" d="{}"/>
</svg>"#, self.h, self.w, self.lines);


      let mut file = File::create(path)?;
      file.write_all(ans.as_bytes())?;
      Ok(())
    }

    fn draw_line_f(&mut self, r0: f32, c0: f32, r1: f32, c1:f32) {
      let w = self.w;
      let h = self.h;
      fn fh(r: f32, h: u32) -> usize {
        (r * h as f32) as usize
      }
      fn fw(c: f32, w: u32) -> usize {
        (c * w as f32) as usize
      }
      self.lines.push_str(&format!("M{} {}L{} {}",
        fw(c0, w), fh(r0, h),
        fw(c1, w), fh(r1, h))
      )
    }
}