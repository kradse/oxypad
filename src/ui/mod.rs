use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{
    Application, 
    ApplicationWindow,
};

pub mod header;
pub mod layout;

pub fn build_ui(app: &Application) {

    let header = header::build_header();
    let layout = layout::build_layout();

    let window = ApplicationWindow::builder()
        .title("OxyPad")
        .default_width(1280)
        .default_height(720)
        .application(app)
        .build();

    window.set_titlebar(Some(&header));
    window.set_child(Some(&layout));
    window.present();
}
