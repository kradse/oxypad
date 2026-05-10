use gtk4::gio::{Menu, MenuModel};
use gtk4 as gtk;
use gtk::{
    Label,
    HeaderBar,
    MenuButton,
};
use gtk::gio;

pub fn build_header() -> HeaderBar {
    let title_widget = Label::builder()
        .label("OxyPad")
        .build();

    let header = HeaderBar::builder()
        .title_widget(&title_widget)
        .show_title_buttons(true)
        .build();

        
    let file_menu = gio::Menu::new();
    file_menu.append(Some("New"), Some("win.file-new"));
    file_menu.append(Some("Open…"), Some("win.file-open"));
    file_menu.append(Some("Save"), Some("win.file-save"));
    file_menu.append(Some("Save As…"), Some("win.file-save-as"));

    let file_button = MenuButton::builder()
        .label("File")
        .menu_model(&file_menu)
        .build();

    header.pack_start(&file_button);

    return header;
}