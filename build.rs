extern crate build_helper;

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use build_helper::target::pointer_width;

fn main() {
  let out_dir = env::var("OUT_DIR").unwrap();
  let dest_path = Path::new(&out_dir).join("fsize.rs");
  let mut f = File::create(&dest_path).unwrap();
  if let Some(pointer_width) = pointer_width() {
    write!(f, "#[allow(non_camel_case_types)] pub type fsize = f{};", pointer_width).unwrap();
  } else {
    write!(f, "compile_error!(\"Could not detect target pointer width.\");").unwrap();
  }
}