use gtk4::{self as gtk, prelude::BoxExt};
use gtk::{
    Box,
    TextView,
    ScrolledWindow
};

pub fn build_layout() -> Box {
    let text_view = TextView::new();
    let scroller = ScrolledWindow::builder()
        .child(&text_view)
        .vexpand(true)
        .hexpand(true)
        .min_content_width(1)
        .min_content_height(1)
        .build();

    let layout = Box::builder()
        .orientation(gtk4::Orientation::Vertical)
        .build();

    layout.append(&scroller);
    return layout;
}