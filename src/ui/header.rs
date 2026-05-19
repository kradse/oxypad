use gtk4 as gtk;
use gtk::{
    Label,
    HeaderBar,
    MenuButton,
    prelude::PopoverExt,
    prelude::WidgetExt,
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

    let section_file = gio::Menu::new();
    section_file.append(Some("New"), Some("win.file-new"));
    section_file.append(Some("Open…"), Some("win.file-open"));

    let section_save = gio::Menu::new();
    section_save.append(Some("Save"), Some("win.file-save"));
    section_save.append(Some("Save As…"), Some("win.file-save-as"));

    let section_quit = gio::Menu::new();
    section_quit.append(Some("Quit"), Some("app.quit"));

    let file_menu = gio::Menu::new();
    file_menu.append_section(None, &section_file);
    file_menu.append_section(None, &section_save);
    file_menu.append_section(None, &section_quit);

    let file_button = MenuButton::builder()
        .label("File")
        .menu_model(&file_menu)
        .build();

    if let Some(popover) = file_button.popover() {
        popover.set_halign(gtk::Align::Start);
        popover.set_has_arrow(false);
    }

    header.pack_start(&file_button);
    header
}