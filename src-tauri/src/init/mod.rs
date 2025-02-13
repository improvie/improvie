pub mod log;

#[cfg(debug_assertions)]
pub fn dev_folder() -> std::path::PathBuf {
    std::path::Path::new(std::env!("CARGO_MANIFEST_DIR")).join("dev")
}

// #[cfg(debug_assertions)]
// pub fn dev_file<P: AsRef<std::path::Path>>(items: &[P]) -> std::path::PathBuf {
//     let mut folder = dev_folder();
//     folder.extend(items);
//     folder
// }
