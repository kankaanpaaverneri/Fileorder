use crate::file::{File, FileMetadata};
use crate::util;
use std::ffi::{OsStr, OsString};
use std::fs::{self, FileType, Metadata, ReadDir};

#[derive(Debug)]
struct ParsedFile {
    file_type: FileType,
    file_name: OsString,
    metadata: Metadata,
}

#[derive(Debug, Clone)]
pub struct Directory {
    id: usize,
    name: OsString,
    directories: Vec<Directory>,
    files: Vec<File>,
    metadata: FileMetadata,
}

impl Directory {
    pub fn new() -> Self {
        Self {
            id: 0,
            name: OsString::new(),
            directories: Vec::new(),
            files: Vec::new(),
            metadata: FileMetadata::new(),
        }
    }
    pub fn clear_directories(&mut self) {
        self.directories.clear();
        self.files.clear();
        self.name.clear();
    }

    pub fn write_directory_content(&mut self, original_path: &OsStr, index: &mut usize) {
        let current_dir = self;
        let path = OsString::from(original_path);
        current_dir.insert_files_and_directories(path.as_os_str(), index);
    }

    pub fn find_directory_by_id(&mut self, id_stack: &Vec<usize>) -> &mut Directory {
        let mut current_dir = self;
        let mut iter = id_stack.iter();

        while let Some(iterator) = iter.next() {
            let index = util::find_directory_index_by_id(current_dir, *iterator);
            if let Some(i) = index {
                current_dir = &mut current_dir.get_mut_directories()[i];
            }
            if let None = index {
                break;
            }
        }

        current_dir
    }

    pub fn insert_new_sub_directory(
        &mut self,
        id_stack: &Vec<usize>,
        current_path: &mut OsString,
        directories_read: &mut usize,
        selected_directory_id: usize,
    ) {
        let mut current_dir = self;
        for i in 0..id_stack.len() {
            let result = util::find_directory_index_by_id(&mut current_dir, selected_directory_id);
            if let Some(index) = result {
                current_path.push("/");
                current_path.push(current_dir.get_directories()[index].get_name());

                current_dir.get_mut_directories()[index]
                    .write_directory_content(current_path.as_os_str(), directories_read);
                break;
            }

            let result: Option<usize> = util::find_directory_index_by_id(current_dir, id_stack[i]);

            if let Some(selected) = result {
                current_dir = &mut current_dir.get_mut_directories()[selected];
            }
        }
    }

    pub fn get_directory_id(&self) -> usize {
        self.id
    }

    pub fn get_directory_ids(&self) -> Vec<usize> {
        let mut dir_ids: Vec<usize> = Vec::new();
        for i in 0..self.directories.len() {
            dir_ids.push(self.directories[i].id);
        }
        dir_ids
    }

    pub fn get_name(&self) -> &OsStr {
        self.name.as_os_str()
    }

    pub fn get_directories(&self) -> &Vec<Self> {
        &self.directories
    }

    pub fn get_mut_directories(&mut self) -> &mut Vec<Self> {
        &mut self.directories
    }

    pub fn get_files(&self) -> &Vec<File> {
        &self.files
    }
    pub fn get_metadata(&self) -> &FileMetadata {
        &self.metadata
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
                        files.push(File::new_from(
                            file.file_name.as_os_str(),
                            self.read_metadata_from_file(&file.metadata),
                        ));
                    }
                }
                Err(error) => {
                    eprintln!("Error occured when reading parsed files: {}", error);
                    std::process::exit(1);
                }
            }
        }
    }

    fn get_entries(&self, entries: ReadDir) -> Vec<std::io::Result<ParsedFile>> {
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
        let mut file_metadata = FileMetadata::new();
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
}
