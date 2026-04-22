use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{
    Button,
    TextView,
    Application, 
    ApplicationWindow,
};

pub fn build_ui(app: &Application) {
    let button = Button::builder()
        .label("Press me!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    button.connect_clicked(|button| {
        button.set_label("Hello, world!");
    });

    let me = TextView::builder()
        .build();

    let window = ApplicationWindow::builder()
        .title("OxyPad")
        .default_width(1280)
        .default_height(720)
        .application(app)
        .child(&me)
        .build();

    window.present();
}
