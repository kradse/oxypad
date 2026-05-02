use gtk4 as gtk;
use gtk::prelude::*;
use gtk::Application;
use gtk::glib;

use std::rc::Rc;
use std::cell::RefCell;

use state::AppState;

mod ui;
mod state;

const APP_ID: &str = "io.github.kradse.OxyPad";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(move |app| {
        let state = Rc::new(RefCell::new(AppState::new()));
        ui::build_ui(app, state);
    });

    app.run()
}
