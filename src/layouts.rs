use iced::{
    alignment::Vertical,
    widget::{button, column, container, row, text, Column},
    Element,
};

use crate::fileorder::{App, Directory, Message};

#[derive(Debug)]
pub enum Layout {
    Home,
    Templates,
}

pub fn home_layout<'a>() -> Element<'a, Message> {
    container(column![
        container(text("File order").center().size(50)).center(iced::Fill),
        row![
            button("Select location to organize").on_press(Message::TemplateLayout),
            button("Create file order")
        ]
    ])
    .align_y(Vertical::Center)
    .center(iced::Fill)
    .into()
}

pub fn templates_layout<'a>(app: &'a App) -> Element<'a, Message> {
    container(column![
        text("Select directory to order").size(25).center(),
        get_directory_buttons(app),
        button("Go back").on_press(Message::HomeLayout)
    ])
    .center(iced::Fill)
    .into()
}

fn get_directory_buttons(app: &App) -> Column<Message> {
    let mut column = Column::new();
    let mut root = app.get_root();
    let mut position = app.get_position().iter();

    // Select current directory to display
    root = find_current_directory(&mut position, root);
    column = column.push(button("Back up").on_press(Message::Out));

    // Display directories
    for dir in root.get_directories() {
        let dir_name = dir.get_name().to_str();
        if let Some(name) = dir_name {
            column = column.push(button(name).on_press(Message::In(dir.get_directory_id())));
        }
    }

    // Display files
    for file in root.get_files() {
        let file_name = file.to_str();
        if let Some(name) = file_name {
            column = column.push(text(name));
        }
    }
    column
}

fn find_current_directory<'a>(
    position: &mut impl Iterator<Item = &'a usize>,
    mut root: &'a Directory,
) -> &'a Directory {
    if let Some(id) = position.next() {
        for directory in root.get_directories() {
            if *id == directory.get_directory_id() {
                root = directory;
                root = find_current_directory(position, root);
            }
        }
    }
    root
}
