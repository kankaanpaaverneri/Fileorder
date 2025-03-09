mod fileorder;
mod layouts;

use fileorder::App;

fn main() -> iced::Result {
    iced::application("File Order", App::update, App::view).run()
}
