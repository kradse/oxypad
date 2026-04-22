use gtk4 as gtk;
use gtk::{
    Label,
    HeaderBar,
};

pub fn build_header() -> HeaderBar {
    let title_widget = Label::builder()
        .label("Untitled.txt")
        .build();

    let header = HeaderBar::builder()
        .title_widget(&title_widget)
        .show_title_buttons(true)
        .build();

    return header;
}