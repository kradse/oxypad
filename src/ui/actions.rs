use gtk4 as gtk;
use gtk::{Application, ApplicationWindow, glib};
use gtk4::gio::SimpleAction;
use gtk::prelude::*;

use std::rc::Rc;
use std::cell::RefCell;
use crate::state::AppState;

pub fn register_win_actions(window: &ApplicationWindow) {
    let action_new = SimpleAction::new("file-new", None);
    action_new.connect_activate(|_, _| {
        println!("New file");
    });
    window.add_action(&action_new);

    let action_open = SimpleAction::new("file-open", None);
    action_open.connect_activate(|_, _| {
        println!("Open file");
    });
    window.add_action(&action_open);

    let action_save = SimpleAction::new("file-save", None);
    action_save.connect_activate(|_, _| {
        println!("Save file");
    });
    window.add_action(&action_save);

    let action_save_as = SimpleAction::new("file-save-as", None);
    action_save_as.connect_activate(|_, _| {
        println!("Save file as");
    });
    window.add_action(&action_save_as);
}

pub fn register_app_actions(app: &Application, state: &Rc<RefCell<AppState>>) {
    let action_quit = SimpleAction::new("quit", None);
    action_quit.connect_activate(glib::clone!(
        #[weak] app,
        move |_, _| {
            app.quit();
        }
    ));
    app.add_action(&action_quit);
}
