use static_files::resource_dir;
fn main() -> std::io::Result<()> {
  tauri_build::build();
  resource_dir("./static").build()
}


// fn main() -> std::io::Result<()> {
//     resource_dir("./static").build()?;
// }