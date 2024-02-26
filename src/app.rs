use egui::{RichText, Vec2};

use crate::{generate_passsword, PasswdgenRequest};
/// We derive Deserialize/Serialize so we can persist app state on shutdown.
// #[derive(serde::Deserialize, serde::Serialize)]
// #[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct PasswdgenEgui {
    length: u32,
    remove_uppercase: bool,
    remove_lowercase: bool,
    remove_numbers: bool,
    add_special_characters: bool,
    remove_similar: bool,
    password: String,
}

impl Default for PasswdgenEgui {
    fn default() -> Self {
        Self {
            length: 16,
            remove_uppercase: false,
            remove_lowercase: false,
            remove_numbers: false,
            add_special_characters: true,
            remove_similar: true,
            password: generate_passsword(PasswdgenRequest::default()).unwrap(),
        }
    }
}

impl PasswdgenEgui {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        // if let Some(storage) = cc.storage {
        // return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        // }

        Default::default()
    }
}

impl eframe::App for PasswdgenEgui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.label(RichText::new("passwdgen").size(40.0));
                ui.label(RichText::new("Your password is:").size(20.0));
                ui.text_edit_singleline(&mut self.password);
                if ui
                    .add(egui::Button::new("Generate").min_size(Vec2::new(50.0, 25.0)))
                    .clicked()
                {
                    self.password = generate_passsword(PasswdgenRequest::new(
                        self.length,
                        self.remove_uppercase,
                        self.remove_lowercase,
                        self.remove_numbers,
                        self.add_special_characters,
                        self.remove_similar,
                    ))
                    .unwrap_or(String::from(
                        "Error: no characters to generate password from.",
                    ))
                }
                ui.add(
                    egui::widgets::DragValue::new(&mut self.length)
                        .prefix("Length: ")
                        .speed(0.05)
                        .clamp_range(1..=128),
                );
                ui.checkbox(&mut self.remove_lowercase, "Remove lowercase characters");
                ui.checkbox(&mut self.remove_uppercase, "Remove uppercase characters");
                ui.checkbox(&mut self.remove_numbers, "Remove numbers");
                ui.checkbox(&mut self.add_special_characters, "Add special characters");
                ui.checkbox(&mut self.remove_similar, "Remove similar characters")
            })
        });
    }
}
