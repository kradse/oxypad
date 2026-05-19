use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{
    Application,
    ApplicationWindow,
};
use std::rc::Rc;
use std::cell::RefCell;
use crate::state::AppState;

pub mod header;
pub mod layout;
pub mod actions;

pub fn build_ui(app: &Application, state: Rc<RefCell<AppState>>) {

    let header = header::build_header();
    let layout = layout::build_layout();

    let window = ApplicationWindow::builder()
        .title("OxyPad")
        .default_width(1280)
        .default_height(720)
        .application(app)
        .build();

    actions::register_win_actions(&window);
    actions::register_app_actions(app, &state);

    window.set_titlebar(Some(&header));
    window.set_child(Some(&layout));
    window.present();
}
