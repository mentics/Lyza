use std::path::{Path, PathBuf};

pub const DIR_DATA:&str = "G:/My Drive/sync/data/market";

// pub fn dir_data(paths:Vec<&str>) -> Path {
pub fn dir_data<T:Into<String>+AsRef<Path>>(paths:&[T]) -> PathBuf {
    let mut path = PathBuf::from(DIR_DATA);
    paths.iter().for_each(|p| path.push(p));
    std::fs::create_dir_all(&path).unwrap();
    return path;
}
