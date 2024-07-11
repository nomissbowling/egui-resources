#![doc(html_root_url = "https://docs.rs/egui-resources/0.4.0")]
//! egui resources
//!
//! https://github.com/google/fonts/blob/main/ofl/firasans/FiraSans-Regular.ttf
//!

use std::error::Error;
use std::{fs, path::PathBuf};
use std::io::Read;
use image::{load_from_memory, DynamicImage, RgbaImage};
use image::imageops::FilterType;
use eframe::{self, egui::*};

/// create DynamicImage from ColorImage
/// - src: &amp;ColorImage
/// - result: DynamicImage
pub fn dynamic_image_from(src: &ColorImage) -> DynamicImage {
  let (sw, sh) = (src.width(), src.height());
unsafe {
  let p = &src.pixels[0] as *const Color32 as *const u8; // pointer
  let s = std::slice::from_raw_parts(p, sw * sh * 4); // 4 for Color32
  DynamicImage::from(
    // RgbaImage is an alias of ImageBuffer
    match RgbaImage::from_raw(sw as u32, sh as u32, s.to_vec()) {
    None => RgbaImage::new(sw as u32, sh as u32),
    Some(b) => b
    }
  )
}
}

/// create resized copy from ColorImage
/// - wh: [usize; 2] (to be resized)
/// - src: &amp;ColorImage
/// - filter: image::imageops::FilterType (Nearest, Lanczos3, etc)
/// - result: ColorImage
pub fn resized_copy_from(wh: [usize; 2], src: &ColorImage,
  filter: FilterType) -> ColorImage {
  let img = dynamic_image_from(src)
    .resize_to_fill(wh[0] as u32, wh[1] as u32, filter);
  // always should use resize_to_fill for any aspect
  color_image_from_dynamic_image(img)
  // ColorImage::from_rgba_unmultiplied(wh, &img.into_rgba8().into_raw())
}

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

/// create ColorImage from DynamicImage
/// - src: DynamicImage (move)
/// - result: ColorImage
pub fn color_image_from_dynamic_image(src: DynamicImage) -> ColorImage {
  let (rgba, width, height) = im_flat!(src);
  ColorImage::from_rgba_unmultiplied(
    [width as usize, height as usize], &rgba)
}

/// ResourceBase
pub struct ResourcesBase {
  /// base path
  pub basepath: PathBuf
}

/// ResourcesBase
impl ResourcesBase {
  /// constructor
  /// - basepath: PathBuf base path (move)
  pub fn new(basepath: PathBuf) -> Self {
    ResourcesBase{basepath}
  }

  /// load resource img
  /// - f: &amp;str filename
  /// - p: bool (true: self.basepath false: full path)
  /// - result: ColorImage
  pub fn resource_img(&self, f: &str, p: bool) -> ColorImage {
    let Ok(b) = self.read_bytes(f, p) else { return ColorImage::example(); };
    if let Ok(img) = load_from_memory(&b) {
      color_image_from_dynamic_image(img)
    }else{
      ColorImage::example()
    }
  }

  /// load resource icon
  /// - ico: &amp;str filename
  /// - p: bool (true: self.basepath false: full path)
  /// - result: Option eframe::IconData
  pub fn resource_icon(&self, ico: &str, p: bool) -> Option<eframe::IconData> {
    let Ok(b) = self.read_bytes(ico, p) else { return None; };
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
  /// - p: bool (true: self.basepath false: full path)
  /// - result: ()
  pub fn resource_font(&self, fonts: &mut FontDefinitions,
    n: &str, f: &str, t: FontFamily, p: bool) {
    let Ok(b) = self.read_bytes(f, p) else { return; };
    let n = n.to_string();
    let m = n.clone();
    fonts.font_data.insert(n, FontData::from_owned(b));
    // fonts.font_data.insert(n, FontData::from_static(include_bytes!(
    //   "static str path from src extended and read at the compile time")));
    fonts.families.entry(t).or_default().insert(0, m);
  }

  /// reg fonts
  /// - ffs: Vec&lt; (name, filename, family) &gt; (move)
  /// - result: FontDefinitions
  pub fn reg_fonts(&self, ffs: Vec<(&str, &str, FontFamily)>) ->
    FontDefinitions {
    let mut fonts = FontDefinitions::default();
    for (n, f, t) in ffs.into_iter() {
      self.resource_font(&mut fonts, n, f, t, !f.contains("/"));
    }
    fonts
  }

  /// read bytes
  /// - f: &amp;str filename
  /// - p: bool (true: self.basepath false: full path)
  /// - result: Result Vec u8
  pub fn read_bytes(&self, f: &str, p: bool) ->
    Result<Vec<u8>, Box<dyn Error>> {
    let p = if !p { PathBuf::from(f) } else { self.basepath.join(f) };
    let mut fi = fs::File::open(&p)?;
    let metadata = fs::metadata(&p)?;
    let mut buf = vec![0u8; metadata.len() as usize];
    fi.read(&mut buf)?;
    Ok(buf)
  }
}

/// tests
#[cfg(test)]
mod tests {
  use super::*;

  /// [-- --nocapture] [-- --show-output]
  #[test]
  fn test_resources() {
    let bp = ResourcesBase::new(PathBuf::from("./resources"));
    let flat = vec![
[255, 0, 0, 255], [255, 0, 0, 255], [0, 255, 0, 255], [0, 255, 0, 255],
[255, 0, 0, 255], [255, 0, 0, 255], [0, 255, 0, 255], [0, 255, 0, 255],
[0, 0, 255, 255], [0, 0, 255, 255], [255, 255, 0, 255], [255, 255, 0, 255],
[0, 0, 255, 255], [0, 0, 255, 255], [255, 255, 0, 255], [255, 255, 0, 255]
    ].into_iter().map(|c|
      Color32::from_rgba_premultiplied(c[0], c[1], c[2], c[3]) // unmultiplied
    ).collect::<Vec<_>>();
    let im = bp.resource_img("_4c_4x4.png", true);
    assert_eq!(im.size, [4, 4]);
    assert_eq!(im.pixels.len(), 16);
    assert_eq!(im.pixels, flat);

    let resized = vec![
      [255, 0, 0, 255], [0, 255, 0, 255],
      [0, 0, 255, 255], [255, 255, 0, 255]
    ].into_iter().map(|c|
      Color32::from_rgba_premultiplied(c[0], c[1], c[2], c[3]) // unmultiplied
    ).collect::<Vec<_>>();
    let img = resized_copy_from([2, 2], &im, FilterType::Nearest);
    assert_eq!(img.size, [2, 2]);
    assert_eq!(img.pixels.len(), 4);
    assert_eq!(img.pixels, resized);
  }
}