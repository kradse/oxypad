use eframe::egui;
use egui::{Id, TextEdit, Vec2};
use rfd::FileDialog;
use std::{f32, path::PathBuf};

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder {
            inner_size: Some(Vec2::new(1440., 1080.)),
            resizable: Some(true),
            ..Default::default()
        },
        ..Default::default()
    };

    eframe::run_native(
        "OxyPad",
        options,
        Box::new(|cc| Ok(Box::new(OxyPad::new(cc)))),
    )
}

struct OxyPad {
    text_content: String,
    curr_file: String,
    show_about: bool,
}

impl OxyPad {
    fn new(_creation_context: &eframe::CreationContext<'_>) -> Self {
        Self {
            ..Default::default()
        }
    }
}

impl Default for OxyPad {
    fn default() -> Self {
        Self {
            text_content: "".to_string(),
            curr_file: "".to_string(),
            show_about: false,
        }
    }
}

impl eframe::App for OxyPad {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut style = (*ctx.style()).clone();
        style.spacing.scroll.floating_width = 4.;
        ctx.set_style(style);

        if ctx.input_mut(|i| i.consume_shortcut(&egui::KeyboardShortcut::new(egui::Modifiers::CTRL, egui::Key::S))) {
            if self.curr_file == "" {
                save_as(self);
            } else {
                let _ = std::fs::write(self.curr_file.clone(), self.text_content.clone());
            }
        }

        if ctx.input_mut(|i| i.consume_shortcut(&egui::KeyboardShortcut::new(egui::Modifiers::CTRL | egui::Modifiers::SHIFT, egui::Key::S))) {
            save_as(self);
        }

        if ctx.input_mut(|i| i.consume_shortcut(&egui::KeyboardShortcut::new(egui::Modifiers::CTRL, egui::Key::N))) {
            self.text_content = "".to_string();
            self.curr_file = "".to_string();
        }

        if ctx.input_mut(|i| i.consume_shortcut(&egui::KeyboardShortcut::new(egui::Modifiers::CTRL, egui::Key::O))) {
            if let Some(file) = FileDialog::new().set_directory("/").pick_file() {
                if let Ok(content) = std::fs::read_to_string(file.as_path()) {
                    self.text_content = content;
                    self.curr_file = file.as_path().to_str().unwrap_or("").to_string();
                }
            }
        }

        egui::TopBottomPanel::top("top_menu").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.add(egui::Button::new("New").shortcut_text("Ctrl+N")).clicked() {
                        self.text_content = "".to_string();
                        self.curr_file = "".to_string();
                    };
                    if ui.add(egui::Button::new("Load").shortcut_text("Ctrl+O")).clicked() {
                        if let Some(file) = FileDialog::new().set_directory("/").pick_file() {
                            if let Ok(content) = std::fs::read_to_string(file.as_path()) {
                                self.text_content = content;
                                self.curr_file = file.as_path().to_str().unwrap_or("").to_string();
                            }
                        }
                    };
                    if ui.add(egui::Button::new("Save").shortcut_text("Ctrl+S")).clicked() {
                        if self.curr_file == "" {
                            save_as(self);
                        } else {
                            let _ = std::fs::write(self.curr_file.clone(), self.text_content.clone());
                        }
                    };
                    if ui.add(egui::Button::new("Save as...").shortcut_text("Ctrl+Shift+S")).clicked() {
                        save_as(self);
                    };
                    ui.separator();
                    if ui.button("Quit").clicked() {
                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                    };
                });
                ui.menu_button("Edit", |ui| {
                    if ui.button("Placeholder").clicked() {};
                });
                ui.menu_button("Help", |ui| {
                    if ui.button("About").clicked() {
                        self.show_about = true;
                        ui.close();
                    };
                });
            });
        });

        if self.show_about {
            egui::Modal::new(Id::new("About OxyPad")).show(ctx, |ui| {
                ui.set_min_size(Vec2::new(240.0, 0.0));
                ui.vertical_centered(|ui| {
                    ui.heading("OxyPad");
                });
                ui.separator();
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    ui.add_space(8.0);
                    ui.vertical(|ui| {
                        ui.label("Version:");
                        ui.label("Date:");
                        ui.add_space(4.0);
                        ui.label("OS:");
                    });
                    ui.vertical(|ui| {
                        ui.label("0.1");
                        ui.label("2025/12/22");
                        ui.add_space(4.0);
                        ui.label("TODO");
                    });
                });
                ui.add_space(16.0);
                ui.vertical_centered(|ui| {
                    ui.horizontal(|ui| {
                        if ui
                            .add_sized([120.0, 32.0], egui::Button::new("Ok"))
                            .clicked()
                        {
                            self.show_about = false;
                        }
                        if ui
                            .add_sized([120.0, 32.0], egui::Button::new("Copy"))
                            .clicked()
                        {
                            self.show_about = false;
                        }
                    });
                });
            });
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                let response = ui.add_sized(
                    ui.available_size(),
                    TextEdit::multiline(&mut self.text_content)
                        .font(egui::TextStyle::Monospace)
                        .desired_width(f32::INFINITY)
                        .frame(false),
                );
                response.request_focus();
            });
        });
        egui::TopBottomPanel::bottom("buttom_panel").show(ctx, |ui| {
            ui.heading("Hello, world");
        });
    }
}

fn save_as(oxypad: &mut OxyPad) {
    let mut path = PathBuf::from("/");
    if oxypad.curr_file.is_empty() {
        path = PathBuf::from(&oxypad.curr_file);
        if let Some(parent) = path.parent() {
            path = parent.to_path_buf();
        }
    }

    let Some(file) = FileDialog::new()
        .set_directory(path)
        .save_file()
    else {
        return;
    };

    if let Ok(_) = std::fs::write(file.as_path(), oxypad.text_content.clone()) {
        oxypad.curr_file = file.as_path().to_str().unwrap_or("").to_string();
    }
}
