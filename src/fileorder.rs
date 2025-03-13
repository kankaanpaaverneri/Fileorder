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
                self.insert_new_sub_directory(selected_directory_id);
            }
            Message::Out => {
                if let Some(_) = self.id_stack.last() {
                    let directory = self.find_directory_by_id();
                    directory.get_mut_directories().clear();
                    self.current_path =
                        util::remove_directory_from_path(&self.current_path.as_os_str());
                    self.id_stack.pop();
                    println!("Directories read: {}", self.directories_read);
                }
            }
        }
    }

    fn insert_new_sub_directory(&mut self, selected_directory_id: usize) {
        let mut current_dir = &mut self.root;
        for i in 0..self.id_stack.len() {
            let result = util::find_directory_index_by_id(&mut current_dir, selected_directory_id);
            if let Some(index) = result {
                self.current_path.push("/");
                self.current_path
                    .push(current_dir.get_directories()[index].get_name());

                current_dir.get_mut_directories()[index].write_directory_content(
                    &self.current_path.as_os_str(),
                    &mut self.directories_read,
                );
                break;
            }

            let result: Option<usize> =
                util::find_directory_index_by_id(current_dir, self.id_stack[i]);

            if let Some(selected) = result {
                current_dir = &mut current_dir.get_mut_directories()[selected];
            }
        }
    }

    pub fn find_directory_by_id(&mut self) -> &mut Directory {
        let mut current_dir = &mut self.root;
        let mut iter = self.id_stack.iter();

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

    pub fn get_root(&self) -> &Directory {
        &self.root
    }

    pub fn get_id_stack(&self) -> &Vec<usize> {
        &self.id_stack
    }
}
