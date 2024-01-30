use std::sync::Arc;

use egui::{TextEdit, TextStyle};
use regex::Regex;

const CORRECT_REGEX_COLOR: egui::Color32 = egui::Color32::DARK_GREEN;
const INCORRECT_REGEX_COLOR: egui::Color32 = egui::Color32::DARK_RED;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct App {
    regex_str: String,
    text: String,
    #[serde(skip)]
    regex: Regex,
    #[serde(skip)]
    regex_field_color: egui::Color32,
}

impl Default for App {
    fn default() -> Self {
        let regex_str = ".*";
        Self {
            regex_str: regex_str.to_string(),
            text: "Hello world".to_string(),
            regex: Regex::new(regex_str).unwrap(),
            regex_field_color: CORRECT_REGEX_COLOR,
        }
    }
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    fn capture(&self) {
        for name in self.regex.capture_names() {
            // log::info!("{name:?}");
        }
        for c in self.regex.captures_iter(&self.text) {
            // c.
        }
    }

    fn my_layouter(ui: &egui::Ui, text: &str, wrap_width: f32) -> Arc<egui::Galley> {
        use egui::text::LayoutJob;
        use egui::Color32;
        let mut job = LayoutJob::default();

        const COLORS: [Color32; 10] = [
            Color32::LIGHT_BLUE,
            Color32::LIGHT_RED,
            Color32::LIGHT_GREEN,
            Color32::LIGHT_YELLOW,
            Color32::LIGHT_GRAY,
            Color32::DARK_BLUE,
            Color32::DARK_RED,
            Color32::DARK_GREEN,
            Color32::BROWN,
            Color32::GOLD,
        ];

        for (index, word) in text.split_whitespace().enumerate() {
            job.append(
                word,
                0.0,
                egui::TextFormat {
                    color: COLORS[index % COLORS.len()],
                    ..Default::default()
                },
            );

            job.append(
                " ",
                0.0,
                egui::TextFormat {
                    color: COLORS[index % COLORS.len()],
                    ..Default::default()
                },
            );
        }

        ui.fonts(|f| f.layout_job(job))
    }
}

impl eframe::App for App {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.monospace("Regex:");
                ui.visuals_mut().extreme_bg_color = self.regex_field_color;
                if ui
                    .add(
                        TextEdit::singleline(&mut self.regex_str)
                            .font(TextStyle::Monospace)
                            .hint_text(".*"),
                    )
                    .changed()
                {
                    if let Ok(regex) = Regex::new(&self.regex_str) {
                        self.regex = regex;
                        // self.capture();
                        self.regex_field_color = CORRECT_REGEX_COLOR;
                    } else {
                        self.regex_field_color = INCORRECT_REGEX_COLOR;
                    }
                }
            });

            ui.add_space(10.0);

            ui.vertical_centered(|ui| {
                ui.monospace("Text: ");
                ui.add(
                    TextEdit::multiline(&mut self.text)
                        .font(TextStyle::Monospace)
                        .hint_text("Hello world")
                        .layouter(&mut Self::my_layouter),
                );
            });
        });
    }
}
