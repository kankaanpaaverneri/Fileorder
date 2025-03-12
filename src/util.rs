use std::ffi::{OsStr, OsString};

use crate::fileorder::Directory;

pub fn remove_directory_from_path(path: &OsStr) -> OsString {
    if let Some(str_path) = path.to_str() {
        let splitted: Vec<&str> = str_path.split("/").collect();
        let mut filtered_path = OsString::new();
        for (index, dir_name) in splitted.iter().enumerate() {
            if !dir_name.is_empty() && index <= splitted.len() - 2 {
                filtered_path.push("/");
                filtered_path.push(*dir_name);
            }
        }
        return filtered_path;
    }
    eprintln!("Couldn't convert OsStr path to String path");
    OsString::from(path)
}

pub fn find_directory_index_by_id(
    current_dir: &mut Directory,
    current_position: usize,
) -> Option<usize> {
    let mut index: Option<usize> = None;
    for (it, directory) in current_dir.get_directories().iter().enumerate() {
        if directory.get_directory_id() == current_position {
            index = Some(it);
        }
    }
    index
}
