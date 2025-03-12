mod fileorder;
mod layouts;
mod util;

use fileorder::App;

fn main() -> iced::Result {
    iced::application("File Order", App::update, App::view).run()
}
