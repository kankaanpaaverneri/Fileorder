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
                if is_drive_identifier(dir_name) {
                    filtered_path.push(*dir_name);
                } else {
                    filtered_path.push("/");
                    filtered_path.push(*dir_name);
                }
                
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

pub fn set_initial_path(initial_path: &mut OsString, operating_system: &OperatingSystem, external_storage_paths: &Vec<OsString>) {
    match operating_system {
        OperatingSystem::MacOs => {
            initial_path.push("/");
        },
        OperatingSystem::Windows => {
            if let Some(path) = external_storage_paths.last() {
                initial_path.push(path);
                initial_path.push("/");
            }
        },
        OperatingSystem::Linux => {},
        OperatingSystem::None => {}
    }
}

pub fn get_drive_letter_from_path(initial_path: &OsStr) -> Option<OsString> {
    let mut drive_letter = OsString::new();
    let mut check = 0;
    if let Some(path) = initial_path.to_str() {
        for (i, character) in path.chars().enumerate() {
            if i == 0 && is_valid_drive_character(character) {
                drive_letter.push(character.to_string().as_str());
                check += 1;
            }

            if i == 1 && character == ':' {
                drive_letter.push(character.to_string().as_str());
                check += 1;
            }
        }
        if check == 2 {
            return Some(drive_letter);
        }
    }

    None
}

fn is_drive_identifier(slice: &str) -> bool {
    let mut has_valid_drive_letter = false;
    let mut has_colon_as_second_character = false;
    for (i, character) in slice.chars().enumerate() {

        // FIX THIS LATER
        if i == 0 && is_valid_drive_character(character) {
            has_valid_drive_letter = true;
        }
        if i == 1 && character == ':' {
            has_colon_as_second_character = true;
            break;
        }
    }

    if has_valid_drive_letter && has_colon_as_second_character {
        return true;
    }

    false
}

fn is_valid_drive_character(character: char) -> bool {
    for ch in 'A'..'Z' {
        if character == ch {
            return true;
        }
    }
    false
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
    for character in 'A'..'Z' {
        let drive_letter = format!("{character}:");
        match fs::read_dir(drive_letter.as_str()) {
            Ok(_) => {
                //let drives_collected: Vec<_> = drives.map(|result| {result.map(|r| {r.file_type()})}).collect();
                storage_paths.push(OsString::from(drive_letter.as_str()));
            },
            Err(_) => {}
        }
    }
}
