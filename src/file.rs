use std::ffi::{OsStr, OsString};

use chrono::{DateTime, Local};

#[derive(Debug, Clone)]
pub struct FileMetadata {
    pub created: Option<DateTime<Local>>,
    pub modified: Option<DateTime<Local>>,
    pub accessed: Option<DateTime<Local>>,
    pub permissions: bool,
}

impl FileMetadata {
    pub fn new() -> Self {
        Self {
            created: None,
            modified: None,
            accessed: None,
            permissions: true,
        }
    }
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
    pub fn new_from(name: &OsStr, metadata: FileMetadata) -> Self {
        Self {
            name: OsString::from(name),
            metadata,
        }
    }
    pub fn get_name(&self) -> &OsStr {
        &self.name.as_os_str()
    }

    pub fn get_metadata(&self) -> &FileMetadata {
        &self.metadata
    }
}
