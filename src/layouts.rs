use chrono::{DateTime, Datelike, Local, Timelike};

use iced::{
    alignment::{Horizontal, Vertical},
    widget::{button, column, container, row, scrollable, text, Column, Container},
    Element, Theme,
};

use crate::directory::Directory;
use crate::file::FileMetadata;
use crate::fileorder::{App, Message};

pub struct FormattedDates {
    pub created: String,
    pub modified: String,
    pub accessed: String,
}

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

pub fn error_layout<'a>(_: &'a App, error_message: &'a str) -> Element<'a, Message> {
    container(text(error_message)).center(iced::Fill).into()
}

pub fn templates_layout<'a>(app: &'a App) -> Element<'a, Message> {
    container(
        column![
            text("Select directory to order").size(25).center(),
            scrollable(get_directory_buttons(app).padding(50)).height(500),
            button("Home")
                .on_press(Message::HomeLayout)
                .style(|theme: &Theme, status| {
                    // Do stuff
                    let palette = theme.extended_palette();
                    match status {
                        button::Status::Active => {
                            // Do something
                            button::Style::default().with_background(palette.secondary.strong.color)
                        }
                        _ => button::primary(theme, status),
                    }
                })
        ]
        .spacing(10),
    )
    .center(iced::Fill)
    .into()
}

fn get_directory_buttons(app: &App) -> Container<Message> {
    let mut column = Column::new();
    let mut root = app.get_root();
    let mut position = app.get_id_stack().iter();

    // Select current directory to display
    root = find_current_directory(&mut position, root);
    column = column
        .push(button(text("..").center().size(15)).on_press(Message::Out))
        .spacing(5);

    column = display_head(column);

    column = display_directories(column, root);
    column = display_files(column, root);

    let container = Container::new(column);
    container
}

fn find_current_directory<'a>(
    position: &mut impl Iterator<Item = &'a usize>,
    mut root: &'a Directory,
) -> &'a Directory {
    while let Some(id) = position.next() {
        for directory in root.get_directories() {
            if *id == directory.get_directory_id() {
                root = directory;
                break;
            }
        }
    }
    root
}

fn format_datetime(datetime: DateTime<Local>) -> String {
    format!(
        "{}-{}-{} {}:{}:{}",
        datetime.year(),
        datetime.month(),
        datetime.day(),
        datetime.hour(),
        datetime.minute(),
        datetime.second()
    )
}

fn get_formatted_metadata(metadata: &FileMetadata) -> FormattedDates {
    let mut formatted_creation_date = String::from("No creation date");
    let mut formatted_modified_date = String::from("No modified date");
    let mut formatted_accessed_date = String::from("No accessed date");
    if let Some(created) = metadata.get_created() {
        formatted_creation_date = format_datetime(created);
    }

    if let Some(modified) = metadata.get_modified() {
        formatted_modified_date = format_datetime(modified);
    }

    if let Some(accessed) = metadata.get_accessed() {
        formatted_accessed_date = format_datetime(accessed);
    }
    let formatted_dates = FormattedDates {
        created: formatted_creation_date,
        modified: formatted_modified_date,
        accessed: formatted_accessed_date,
    };
    formatted_dates
}

fn display_head<'a>(mut column: Column<'a, Message>) -> Column<'a, Message> {
    column = column.push(
        row![
            text("Name").size(15).width(iced::FillPortion(1)),
            text("Created").size(15).width(iced::FillPortion(1)),
            text("Modified").size(15).width(iced::FillPortion(1)),
            text("Accessed").size(15).width(iced::FillPortion(1)),
        ]
        .padding(5)
        .spacing(5),
    );
    column
}

fn display_directories<'a>(
    mut column: Column<'a, Message>,
    root: &'a Directory,
) -> Column<'a, Message> {
    for dir in root.get_directories() {
        let dir_name = dir.get_name().to_str();
        let directory_metadata = dir.get_metadata();
        let formatted_dates = get_formatted_metadata(directory_metadata);
        if let Some(name) = dir_name {
            column = column.push(
                button(row![
                    text(name)
                        .center()
                        .size(15)
                        .align_x(Horizontal::Left)
                        .width(iced::FillPortion(1)),
                    text(formatted_dates.created)
                        .center()
                        .size(15)
                        .align_x(Horizontal::Left)
                        .width(iced::FillPortion(1)),
                    text(formatted_dates.modified)
                        .center()
                        .size(15)
                        .align_x(Horizontal::Left)
                        .width(iced::FillPortion(1)),
                    text(formatted_dates.accessed)
                        .center()
                        .size(15)
                        .align_x(Horizontal::Left)
                        .width(iced::FillPortion(1)),
                ])
                .on_press(Message::In(dir.get_directory_id())),
            );
        }
    }
    column
}

fn display_files<'a>(mut column: Column<'a, Message>, root: &'a Directory) -> Column<'a, Message> {
    for file in root.get_files() {
        let file_name = file.get_name();
        let file_metadata = file.get_metadata();

        let formatted_dates = get_formatted_metadata(file_metadata);

        if let Some(name) = file_name.to_str() {
            column = column.push(
                row![
                    text(name).size(15).width(iced::FillPortion(1)),
                    text(formatted_dates.created)
                        .center()
                        .align_x(Horizontal::Left)
                        .size(15)
                        .width(iced::FillPortion(1)),
                    text(formatted_dates.modified)
                        .center()
                        .align_x(Horizontal::Left)
                        .size(15)
                        .width(iced::FillPortion(1)),
                    text(formatted_dates.accessed)
                        .center()
                        .align_x(Horizontal::Left)
                        .size(15)
                        .width(iced::FillPortion(1))
                ]
                .spacing(5)
                .padding(5),
            );
        }
    }
    column
}
