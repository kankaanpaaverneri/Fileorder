use std::{
    ffi::{OsStr, OsString},
    fs,
};

use crate::{directory::Directory, fileorder::OperatingSystem};

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

pub fn get_external_storage_paths(operating_system: &OperatingSystem) -> Vec<OsString> {
    let mut storage_paths: Vec<OsString> = Vec::new();
    match operating_system {
        OperatingSystem::MacOs => get_external_storage_devices_on_macos(&mut storage_paths),
        OperatingSystem::Windows => {}
        OperatingSystem::Linux => {}
        OperatingSystem::None => {}
    }
    storage_paths
}

fn get_external_storage_devices_on_macos(storage_paths: &mut Vec<OsString>) {
    let result = fs::read_dir("/Volumes").map_err(|error| {
        eprintln!("Error when getting external storage devices: {}", error);
    });
    match result {
        Ok(entries) => {
            let results: Vec<_> = entries.map(|result| result.map(|r| r.path())).collect();
            for result in results {
                match result {
                    Ok(path) => {
                        storage_paths.push(OsString::from(path.as_os_str()));
                    }
                    Err(e) => {
                        eprintln!("Error occured while reading paths: {}", e)
                    }
                }
            }
        }
        _ => {}
    }
}
