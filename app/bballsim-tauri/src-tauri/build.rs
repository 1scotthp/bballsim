
// extern crate includedir_codegen;

use tauri_includedir_codegen::Compression;

fn main() {
  tauri_includedir_codegen::start("FILES")
  .dir("assets", Compression::Gzip)
  // .build("data.rs")
  .build("data.rs", ["darko.json".to_string()].to_vec())
  .unwrap();
  tauri_build::build()
}



