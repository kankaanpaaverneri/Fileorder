use std::{
    ffi::{OsStr, OsString},
    fs::{self, FileType, Metadata, ReadDir},
    io::Result,
};

use chrono::{DateTime, Local};
use iced::Element;

use crate::{
    layouts,
    util::{self},
};

#[derive(Debug, Clone)]
pub struct FileMetadata {
    created: Option<DateTime<Local>>,
    modified: Option<DateTime<Local>>,
    accessed: Option<DateTime<Local>>,
    permissions: bool,
}

pub struct FormattedDates {
    pub created: String,
    pub modified: String,
    pub accessed: String,
}

impl FileMetadata {
    pub fn get_created(&self) -> Option<DateTime<Local>> {
        self.created
    }

    pub fn get_modified(&self) -> Option<DateTime<Local>> {
        self.modified
    }

    pub fn get_accessed(&self) -> Option<DateTime<Local>> {
        self.accessed
    }
}

#[derive(Debug, Clone)]
pub struct File {
    name: OsString,
    metadata: FileMetadata,
}

impl File {
    pub fn get_name(&self) -> &OsStr {
        &self.name.as_os_str()
    }

    pub fn get_metadata(&self) -> &FileMetadata {
        &self.metadata
    }
}

#[derive(Debug, Clone)]
pub struct Directory {
    id: usize,
    name: OsString,
    directories: Vec<Directory>,
    files: Vec<File>,
    metadata: FileMetadata,
}

impl Drop for Directory {
    fn drop(&mut self) {}
}

impl Directory {
    pub fn new() -> Self {
        Self {
            id: 0,
            name: OsString::new(),
            directories: Vec::new(),
            files: Vec::new(),
            metadata: FileMetadata {
                created: None,
                modified: None,
                accessed: None,
                permissions: true,
            },
        }
    }
    pub fn clear_directories(&mut self) {
        self.directories.clear();
        self.files.clear();
        self.name.clear();
    }

    fn write_directory_content(&mut self, original_path: &OsStr, index: &mut usize) {
        let current_dir = self;
        let path = OsString::from(original_path);
        current_dir.insert_files_and_directories(path.as_os_str(), index);
    }

    fn insert_files_and_directories(&mut self, path: &OsStr, index: &mut usize) {
        match fs::read_dir(path).map_err(|error| {
            eprintln!("Error occured when reading directories: {}", error);
        }) {
            Ok(entries) => {
                let mut directories: Vec<Directory> = Vec::new();
                let mut files: Vec<File> = Vec::new();

                // Insert directories and files to current_dir
                self.read_entries(entries, &mut directories, &mut files, index);
                self.insert_files(files);
                self.insert_directories(directories);
            }
            _ => {}
        }
    }

    fn insert_directories(&mut self, directories: Vec<Directory>) {
        self.directories.clear();
        for directory in directories {
            self.directories.push(directory);
        }
    }

    fn insert_files(&mut self, files: Vec<File>) {
        self.files.clear();
        for file in files {
            self.files.push(file);
        }
    }

    fn read_entries(
        &self,
        entries: ReadDir,
        directories: &mut Vec<Directory>,
        files: &mut Vec<File>,
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
                            metadata: self.read_metadata_from_file(&file.metadata),
                        });
                        *index += 1;
                    } else if file.file_type.is_file() {
                        // File logic
                        files.push(File {
                            name: file.file_name,
                            metadata: self.read_metadata_from_file(&file.metadata),
                        });
                    }
                }
                Err(error) => {
                    eprintln!("Error occured when reading parsed files: {}", error);
                    std::process::exit(1);
                }
            }
        }
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

    fn read_metadata_from_file(&self, metadata: &Metadata) -> FileMetadata {
        let mut file_metadata = FileMetadata {
            created: None,
            modified: None,
            accessed: None,
            permissions: true,
        };
        if let Ok(created) = metadata.created() {
            file_metadata.created = Some(created.into());
        }
        if let Ok(modified) = metadata.modified() {
            file_metadata.modified = Some(modified.into());
        }

        if let Ok(accessed) = metadata.accessed() {
            file_metadata.accessed = Some(accessed.into());
        }

        if metadata.permissions().readonly() {
            file_metadata.permissions = false;
        }

        file_metadata
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

    pub fn get_files(&self) -> &Vec<File> {
        &self.files
    }
    pub fn get_metadata(&self) -> &FileMetadata {
        &self.metadata
    }
}

#[derive(Debug)]
pub struct App {
    position: Vec<usize>,
    root: Directory,
    layout: layouts::Layout,
    current_path: OsString,
    directories_read: usize,
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
                metadata: FileMetadata {
                    created: None,
                    modified: None,
                    accessed: None,
                    permissions: true,
                },
            },
            layout: layouts::Layout::Home,
            current_path: OsString::from("/Users/vernerikankaanpaa"),
            directories_read: 0,
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
                let mut root = Directory::new();
                let mut index = 0;
                root.write_directory_content(self.current_path.as_os_str(), &mut index);
                self.directories_read = index;

                self.root = root;
                self.layout = layouts::Layout::Templates
            }
            Message::In(selected_directory_id) => {
                self.position.push(selected_directory_id);
                let mut i = 0;
                let mut current_dir = &mut self.root;
                while i < self.position.len() {
                    let mut directory_found = false;
                    let result =
                        util::find_directory_index_by_id(&mut current_dir, selected_directory_id);
                    if let Some(index) = result {
                        directory_found = true;
                        self.current_path.push("/");
                        self.current_path
                            .push(current_dir.directories[index].name.as_os_str());
                        current_dir.directories[index].write_directory_content(
                            &self.current_path.as_os_str(),
                            &mut self.directories_read,
                        );
                    }

                    if !directory_found {
                        let result: Option<usize> =
                            util::find_directory_index_by_id(current_dir, self.position[i]);

                        if let Some(selected) = result {
                            current_dir = &mut current_dir.directories[selected];
                        }
                        i += 1;
                        continue;
                    }
                    break;
                }
            }
            Message::Out => {
                if let Some(_) = self.position.pop() {
                    self.current_path =
                        util::remove_directory_from_path(&self.current_path.as_os_str());
                }
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
