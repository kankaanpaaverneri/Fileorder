use iced::Element;
use std::ffi::OsString;

use crate::{directory::Directory, file::FileMetadata, layouts, util};

#[derive(Debug)]
pub struct App {
    operating_system: OperatingSystem,
    root: Directory,
    layout: layouts::Layout,
    id_stack: Vec<usize>,
    current_path: OsString,
    directories_read: usize,
    external_storage_paths: Vec<OsString>,
    external_storage_directories: Vec<Directory>,
}

#[derive(Debug)]
pub enum OperatingSystem {
    MacOs,
    Windows,
    Linux,
    None,
}

fn detect_operating_system() -> OperatingSystem {
    match std::env::consts::OS {
        "macos" => OperatingSystem::MacOs,
        "windows" => OperatingSystem::Windows,
        "linux" => OperatingSystem::Linux,
        _ => OperatingSystem::None,
    }
}

const ROOTPATH: &str = "";

impl Default for App {
    fn default() -> Self {
        Self {
            operating_system: detect_operating_system(),
            root: Directory::new(),
            layout: layouts::Layout::Home,
            id_stack: Vec::new(),
            current_path: OsString::from(ROOTPATH),
            directories_read: 0,
            external_storage_paths: Vec::new(),
            external_storage_directories: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    HomeLayout,
    TemplateLayout,
    In(usize),
    Out,
    InExternal(usize),
}

impl App {
    pub fn view(&self) -> Element<Message> {
        if let OperatingSystem::None = self.operating_system {
            return layouts::error_layout(self, "Could not detect operating system");
        }
        match self.layout {
            layouts::Layout::Home => layouts::home_layout(),
            layouts::Layout::Templates => layouts::templates_layout(self),
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::HomeLayout => {
                self.layout = layouts::Layout::Home;
                self.id_stack = Vec::new();
            }
            Message::TemplateLayout => {
                self.external_storage_directories.clear();
                self.external_storage_paths =
                    util::get_external_storage_paths(&self.operating_system);
                    self.initialize_external_devices();
                
                self.current_path = OsString::from(ROOTPATH);
                self.root.clear_directories();

                let mut index = 0;
                let mut initial_path = OsString::new();
                initial_path.push("/");
                self.root
                    .write_directory_content(initial_path.as_os_str(), &mut index);
                self.directories_read = index;
                self.layout = layouts::Layout::Templates;
            }
            Message::In(selected_directory_id) => {
                println!("current path prev: {:?}", self.current_path);
                self.id_stack.push(selected_directory_id);
                self.root.insert_new_sub_directory(
                    &mut self.id_stack,
                    &mut self.current_path,
                    &mut self.directories_read,
                    selected_directory_id,
                );
                println!("current path after: {:?}", self.current_path);
            }
            Message::Out => {
                if let Some(_) = self.id_stack.last() {
                    let directory = self.root.find_directory_by_id(&self.id_stack);
                    let directory_ids = directory.get_directory_ids();
                    self.directories_read -= directory_ids.len();
                    directory.get_mut_directories().clear();
                    directory.get_mut_files().clear();
                    self.current_path =
                        util::remove_directory_from_path(&self.current_path.as_os_str(), &self.operating_system);
                    self.id_stack.pop();
                    println!("Current path: {:?}", self.current_path.as_os_str());
                }
            }
            Message::InExternal(selected_directory_id) => {
                match self.operating_system {
                    OperatingSystem::MacOs => self.change_storage_device_on_mac(selected_directory_id),
                    OperatingSystem::Windows => self.change_storage_device_on_windows(selected_directory_id),
                    _ => {}
                }
            }
        }
    }

    pub fn get_root(&self) -> &Directory {
        &self.root
    }

    pub fn get_id_stack(&self) -> &Vec<usize> {
        &self.id_stack
    }

    pub fn get_external_storage_devices(&self) -> &Vec<Directory> {
        &self.external_storage_directories
    }

    fn initialize_external_devices(&mut self) {
        for (i, path) in self.external_storage_paths.iter().enumerate() {
            if let Some(path_str) = path.to_str() {
                let mut last = "";
                for splitted in path_str.split("/") {
                    last = splitted;
                }
                let mut directory_name = OsString::new();
                directory_name.push(last);
                let storage_device = Directory::build(
                    i,
                    directory_name.as_os_str(),
                    Vec::new(),
                    Vec::new(),
                    FileMetadata::new(),
                );
                self.external_storage_directories.push(storage_device);
            } else {
                eprintln!("Failed to convert external storage path from &OsStr to &str");
            }
        }
    }

    fn change_storage_device_on_mac(&mut self, selected_directory_id: usize) {
        for directory in &self.external_storage_directories {
            let mut path_to_external_dir = OsString::new();
            path_to_external_dir.push("/Volumes/");
            if directory.get_directory_id() == selected_directory_id {
                path_to_external_dir.push(directory.get_name());
                self.root.clear_directories();
                self.directories_read = 0;
                self.root.write_directory_content(
                    path_to_external_dir.as_os_str(),
                    &mut self.directories_read,
                );

                self.current_path = OsString::new();
                self.current_path.push("/Volumes/");
                self.current_path.push(directory.get_name());
                self.id_stack.clear();
            }
        }
    }

    fn change_storage_device_on_windows(&mut self, selected_directory_id: usize) {
        for directory in &self.external_storage_directories {
            let mut path_to_external_dir: OsString = OsString::new();
            if directory.get_directory_id() == selected_directory_id {
                path_to_external_dir.push(directory.get_name());
                path_to_external_dir.push("/");
                self.root.clear_directories();
                self.directories_read = 0;
                self.root.write_directory_content(path_to_external_dir.as_os_str(), &mut self.directories_read);
                self.current_path = OsString::new();
                self.current_path.push(directory.get_name());
                self.id_stack.clear();
            }
        }
    }
}
