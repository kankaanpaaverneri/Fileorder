use std::{
    ffi::{OsStr, OsString},
    fs,
};

use crate::{directory::Directory, fileorder::OperatingSystem};

pub fn remove_directory_from_path(path: &OsStr, operating_system: &OperatingSystem) -> OsString {
    if let Some(str_path) = path.to_str() {
        let splitted: Vec<&str> = str_path.split("/").collect();
        let mut filtered_path = OsString::new();
        for (index, dir_name) in splitted.iter().enumerate() {
            if index == splitted.len() -1 {
                break;
            }
            if is_drive_indentifier(&dir_name) {
                if let OperatingSystem::Windows = operating_system {
                    filtered_path.push(dir_name);
                }
            } else {
                filtered_path.push("/");
                filtered_path.push(dir_name);
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
        OperatingSystem::Windows => get_external_storage_devices_on_windows(&mut storage_paths),
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

fn get_external_storage_devices_on_windows(storage_paths: &mut Vec<OsString>) {
    for drive_letter in 'A'..'Z' {
        
        let read_dir_path = format!("{}:", drive_letter);
        match fs::read_dir(read_dir_path) {
            Ok(_) => {
                let mut path = OsString::new();
                path.push(drive_letter.to_string().as_str());
                path.push(":");
                storage_paths.push(path);
            },
            _ => {}
        }
    }
}

fn is_drive_indentifier(directory_name: &str) -> bool {
    let mut correct: usize = 0;
    for (i, character) in directory_name.chars().enumerate() {
        if i == 0 && is_drive_letter(&character) {
            correct += 1;
        }
        if i == 1 && character == ':' {
            correct += 1;
        }
    }

    if correct == 2 {
        return true;
    }
    false
}

fn is_drive_letter(character: &char) -> bool {
    match character {
        'A'..'Z' => true,
        _ => false
    }
}
