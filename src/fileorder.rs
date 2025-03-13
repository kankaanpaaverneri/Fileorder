use iced::Element;
use std::ffi::OsString;

use crate::{directory::Directory, layouts, util};

#[derive(Debug)]
pub struct App {
    id_stack: Vec<usize>,
    root: Directory,
    layout: layouts::Layout,
    current_path: OsString,
    directories_read: usize,
}

impl Default for App {
    fn default() -> Self {
        Self {
            id_stack: Vec::new(),
            root: Directory::new(),
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
                self.id_stack = Vec::new();
            }
            Message::TemplateLayout => {
                self.root.clear_directories();
                let mut index = 0;
                self.root
                    .write_directory_content(self.current_path.as_os_str(), &mut index);
                self.directories_read = index;
                self.layout = layouts::Layout::Templates
            }
            Message::In(selected_directory_id) => {
                self.id_stack.push(selected_directory_id);
                self.root.insert_new_sub_directory(
                    &mut self.id_stack,
                    &mut self.current_path,
                    &mut self.directories_read,
                    selected_directory_id,
                );
            }
            Message::Out => {
                if let Some(_) = self.id_stack.last() {
                    let directory = self.root.find_directory_by_id(&self.id_stack);
                    directory.get_mut_directories().clear();
                    self.current_path =
                        util::remove_directory_from_path(&self.current_path.as_os_str());
                    self.id_stack.pop();
                    println!("Directories read: {}", self.directories_read);
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
}
