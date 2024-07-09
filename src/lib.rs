#![doc(html_root_url = "https://docs.rs/egui-resources/0.1.0")]
//! egui resources
//!
//! https://github.com/google/fonts/blob/main/ofl/firasans/FiraSans-Regular.ttf
//!

use std::error::Error;
use std::fs;
use std::io::Read;
use image::load_from_memory;
use eframe::{self, egui::*};

/// macro im_flat
/// - img: image::DynamicImage
/// - result: ([u8], u32, u32)
#[macro_export]
macro_rules! im_flat {
  ($img: expr) => {{
    let im = $img.into_rgba8();
    let (width, height) = im.dimensions();
    let rgba = im.into_raw();
    (rgba, width, height)
  }}
}
// pub use im_flat;

/// load resource img
/// - f: &amp;str filename
/// - p: bool (true: ./resources false: full path)
/// - result: ColorImage
pub fn resource_img(f: &str, p: bool) -> ColorImage {
  let Ok(b) = read_bytes(f, p) else { return ColorImage::example(); };
  if let Ok(img) = load_from_memory(&b) {
    let (rgba, width, height) = im_flat!(img);
    ColorImage::from_rgba_unmultiplied(
      [width as usize, height as usize], &rgba)
  }else{
    ColorImage::example()
  }
}

/// load resource icon
/// - ico: &amp;str filename
/// - result: Option eframe::IconData
pub fn resource_icon(ico: &str) -> Option<eframe::IconData> {
  let Ok(b) = read_bytes(ico, true) else { return None; };
  if let Ok(img) = load_from_memory(&b) {
    let (rgba, width, height) = im_flat!(img);
    Some(eframe::IconData{rgba, width, height})
  }else{
    None
  }
}

/// load resource font
/// - fonts: &amp;mut FontDefinitions
/// - n: &amp;str name
/// - f: &amp;str filename
/// - t: FontFamily family (move)
/// - result: ()
pub fn resource_font(fonts: &mut FontDefinitions,
  n: &str, f: &str, t: FontFamily) {
  let Ok(b) = read_bytes(f, true) else { return; };
  let n = n.to_string();
  let m = n.clone();
  fonts.font_data.insert(n, FontData::from_owned(b));
  // fonts.font_data.insert(n, FontData::from_static(include_bytes!(
  //   "static str path from src extended and read at the compile time")));
  fonts.families.entry(t).or_default().insert(0, m);
}

/// reg fonts
/// - ffs: Vec&gt; (name, filename, family) &lt; (move)
/// - result: FontDefinitions
pub fn reg_fonts(ffs: Vec<(&str, &str, FontFamily)>) -> FontDefinitions {
  let mut fonts = FontDefinitions::default();
  for (n, f, t) in ffs.into_iter() { resource_font(&mut fonts, n, f, t); }
  fonts
}

/// read bytes
/// - f: &amp;str filename
/// - p: bool (true: ./resources false: full path)
/// - result: Result Vec u8
pub fn read_bytes(f: &str, p: bool) -> Result<Vec<u8>, Box<dyn Error>> {
  let p = if !p { f.to_string() } else { format!("./resources/{}", f) };
  let mut fi = fs::File::open(&p)?;
  let metadata = fs::metadata(&p)?;
  let mut buf = vec![0u8; metadata.len() as usize];
  fi.read(&mut buf)?;
  Ok(buf)
}

/// tests
#[cfg(test)]
mod tests {
  use super::*;

  /// [-- --nocapture] [-- --show-output]
  #[test]
  fn test_resources() {
    let flat = vec![
[255, 0, 0, 255], [255, 0, 0, 255], [0, 255, 0, 255], [0, 255, 0, 255],
[255, 0, 0, 255], [255, 0, 0, 255], [0, 255, 0, 255], [0, 255, 0, 255],
[0, 0, 255, 255], [0, 0, 255, 255], [255, 255, 0, 255], [255, 255, 0, 255],
[0, 0, 255, 255], [0, 0, 255, 255], [255, 255, 0, 255], [255, 255, 0, 255]
    ].into_iter().map(|c|
      Color32::from_rgba_premultiplied(c[0], c[1], c[2], c[3]) // unmultiplied
    ).collect::<Vec<_>>();
    let im = resource_img("_4c_4x4.png", true);
    assert_eq!(im.size, [4, 4]);
    assert_eq!(im.pixels.len(), 16);
    assert_eq!(im.pixels, flat);
  }
}