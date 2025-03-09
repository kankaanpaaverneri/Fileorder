use std::{
    ffi::{OsStr, OsString},
    fs::{self, FileType, Metadata, ReadDir},
    io::Result,
};

use iced::Element;

use crate::layouts;

#[derive(Debug, Clone)]
pub struct Directory {
    id: usize,
    name: OsString,
    directories: Vec<Directory>,
    files: Vec<OsString>,
}

impl Directory {
    pub fn clear_directories(&mut self) {
        self.directories.clear();
        self.files.clear();
        self.name.clear();
    }

    fn read_directories(&mut self, path: &OsStr, mut index: usize) -> usize {
        let read_result = fs::read_dir(path);
        match read_result {
            Ok(entries) => {
                let mut directories: Vec<Directory> = Vec::new();
                let mut files: Vec<OsString> = Vec::new();
                self.read_entries(entries, &mut directories, &mut files, &mut index);

                // Copy files
                for file in files {
                    self.files.push(file);
                }

                // Copy directories
                for mut directory in directories {
                    index = directory.read_directories(
                        &self.get_updated_path(path, directory.name.as_os_str()),
                        index,
                    );
                    self.directories.push(directory);
                }
            }
            Err(error) => {
                eprintln!("Error occured while reading entries: {}", error);
                std::process::exit(1);
            }
        }
        index
    }

    fn get_entries(&self, entries: ReadDir) -> Vec<Result<ParsedFile>> {
        entries
            .map(|entry| {
                entry.map(|result| {
                    let metadata = result.metadata().unwrap_or_else(|error| {
                        eprintln!("Error occured when getting metadata: {}", error);
                        std::process::exit(1);
                    });
                    let file_type = result.file_type().unwrap_or_else(|error| {
                        eprintln!("Error occured when getting filetype: {}", error);
                        std::process::exit(1);
                    });
                    let file_name = result.file_name();
                    ParsedFile {
                        file_type,
                        file_name,
                        metadata,
                    }
                })
            })
            .collect()
    }

    fn read_entries(
        &self,
        entries: ReadDir,
        directories: &mut Vec<Directory>,
        files: &mut Vec<OsString>,
        index: &mut usize,
    ) {
        let list_of_files: Vec<_> = self.get_entries(entries);
        for file in list_of_files {
            match file {
                Ok(file) => {
                    if file.file_type.is_dir() {
                        directories.push(Directory {
                            id: *index,
                            name: file.file_name,
                            directories: Vec::new(),
                            files: Vec::new(),
                        });
                        *index += 1;
                    } else if file.file_type.is_file() {
                        files.push(file.file_name);
                    }
                }
                Err(error) => {
                    eprintln!("Error occured when reading parsed files: {}", error);
                    std::process::exit(1);
                }
            }
        }
    }

    fn get_updated_path(&self, path: &OsStr, new_directory_name: &OsStr) -> OsString {
        let mut updated_path = OsString::from(path);
        if path != "/" {
            updated_path.push("/");
        }
        updated_path.push(new_directory_name);
        updated_path
    }

    pub fn get_directory_id(&self) -> usize {
        self.id
    }

    pub fn get_name(&self) -> &OsStr {
        self.name.as_os_str()
    }

    pub fn get_directories(&self) -> &Vec<Self> {
        &self.directories
    }

    pub fn get_files(&self) -> &Vec<OsString> {
        &self.files
    }
}

#[derive(Debug)]
pub struct App {
    position: Vec<usize>,
    root: Directory,
    layout: layouts::Layout,
}

#[derive(Debug)]
struct ParsedFile {
    file_type: FileType,
    file_name: OsString,
    metadata: Metadata,
}

impl Default for App {
    fn default() -> Self {
        Self {
            position: Vec::new(),
            root: Directory {
                id: 0,
                name: OsString::new(),
                directories: Vec::new(),
                files: Vec::new(),
            },
            layout: layouts::Layout::Home,
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
        match self.layout {
            layouts::Layout::Home => layouts::home_layout(),
            layouts::Layout::Templates => layouts::templates_layout(self),
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::HomeLayout => {
                self.layout = layouts::Layout::Home;
                self.position = Vec::new();
            }
            Message::TemplateLayout => {
                self.root.clear_directories();
                self.root.read_directories(
                    OsString::from("/Users/vernerikankaanpaa/Maze").as_os_str(),
                    1,
                );
                self.layout = layouts::Layout::Templates
            }
            Message::In(directory_index) => {
                self.position.push(directory_index);
            }
            Message::Out => {
                self.position.pop();
            }
        }
    }

    pub fn get_root(&self) -> &Directory {
        &self.root
    }

    pub fn get_position(&self) -> &Vec<usize> {
        &self.position
    }
}
