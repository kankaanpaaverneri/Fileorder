use iced::Element;
use std::ffi::OsString;

use crate::{directory::Directory, layouts, util};

#[derive(Debug)]
pub struct App {
    operating_system: OperatingSystem,
    root: Directory,
    layout: layouts::Layout,
    id_stack: Vec<usize>,
    current_path: OsString,
    directories_read: usize,
    external_storage_paths: Vec<OsString>,
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
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    HomeLayout,
    TemplateLayout,
    In(usize),
    Out,
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

                
                self.root.clear_directories();
                
                self.external_storage_paths =
                    util::get_external_storage_paths(&self.operating_system);
                
                let mut initial_path = OsString::new();
                util::set_initial_path(&mut initial_path, &self.operating_system, &self.external_storage_paths);

                
                if let Some(drive_letter) = util::get_drive_letter_from_path(initial_path.as_os_str()) {
                    self.current_path = drive_letter;
                }

                let mut index = 0;
                self.root
                    .write_directory_content(initial_path.as_os_str(), &mut index);
                self.directories_read = index;
                self.layout = layouts::Layout::Templates;
                

                println!("Current path: {:?}", self.current_path.as_os_str());
            }
            Message::In(selected_directory_id) => {
                self.id_stack.push(selected_directory_id);
                self.root.insert_new_sub_directory(
                    &mut self.id_stack,
                    &mut self.current_path,
                    &mut self.directories_read,
                    selected_directory_id,
                );
                println!("Current path: {:?}", self.current_path.as_os_str());
            }
            Message::Out => {
                if let Some(_) = self.id_stack.last() {
                    let directory = self.root.find_directory_by_id(&self.id_stack);
                    let directory_ids = directory.get_directory_ids();
                    self.directories_read -= directory_ids.len();
                    directory.get_mut_directories().clear();
                    directory.get_mut_files().clear();
                    self.current_path =
                        util::remove_directory_from_path(&self.current_path.as_os_str());
                    self.id_stack.pop();
                    println!("Current path: {:?}", self.current_path.as_os_str());
                }
            },
        }
    }

    pub fn get_root(&self) -> &Directory {
        &self.root
    }

    pub fn get_id_stack(&self) -> &Vec<usize> {
        &self.id_stack
    }

    pub fn get_external_storage_paths(&self) -> &Vec<OsString> {
        &self.external_storage_paths
    }
}
