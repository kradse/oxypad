use iced::widget::text;
use iced::{Element, Task, Theme};

fn main() -> iced::Result {
    iced::application(OxyPad::boot, OxyPad::update, OxyPad::view)
        .title("Oxypad")
        .theme(|_: &OxyPad| Theme::Dark)
        .run()
}

#[derive(Default)]
struct OxyPad;

impl OxyPad {
    fn boot() -> (Self, Task<()>) {
        (Self::default(), Task::none())
    }

    fn update(&mut self, _message: ()) -> Task<()> {
        Task::none()
    }

    fn view(&self) -> Element<'_, ()> {
        text("Hello, world!").into()
    }
}
