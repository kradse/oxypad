use gtk4 as gtk;
use gtk::prelude::*;
use gtk::Application;
use gtk::glib;

mod ui;

const APP_ID: &str = "org.oxypaint.OxyPad";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(ui::build_ui);
    app.run()
}
