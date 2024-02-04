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
    regex: Option<Regex>,
    #[serde(skip)]
    regex_field_color: egui::Color32,
    #[serde(skip)]
    matched_groups: Vec<MatchGroup>,
}

impl Default for App {
    fn default() -> Self {
        let regex_str = ".*"; // TODO: rethink
        Self {
            regex_str: regex_str.to_string(),
            text: "Hello world".to_string(),
            regex: None,
            regex_field_color: CORRECT_REGEX_COLOR,
            matched_groups: Vec::new(),
        }
    }
}

#[derive(Debug)]
struct MatchGroup {
    name: Option<String>,
    start: usize,
    end: usize,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    fn compile_regex(&mut self) {
        if let Ok(regex) = Regex::new(&self.regex_str) {
            self.regex = Some(regex);
            self.collect_captures();
            self.regex_field_color = CORRECT_REGEX_COLOR;
        } else {
            self.regex_field_color = INCORRECT_REGEX_COLOR;
        }
    }

    fn collect_captures(&self) {
        if let Some(regex) = &self.regex {
            println!("capture");
            let capture_names = regex.capture_names().collect::<Vec<_>>(); // TODO: do it when pattern changed

            let mut matched_groups = Vec::new();
            let mut locs = regex.capture_locations();
            if let Some(_) = regex.captures_read(&mut locs, &self.text) {
                for i in 0..locs.len() {
                    let prefix = capture_names[i].unwrap_or("none");
                    println!("{}: {:?}", prefix, locs.get(i));

                    let (start, end) = locs.get(i).unwrap();
                    matched_groups.push(MatchGroup {
                        name: Some(prefix.to_string()),
                        start,
                        end,
                    });
                }
            }

            println!("{:?}", matched_groups);
        }
    }

    fn my_layouter(ui: &egui::Ui, text: &str, _wrap_width: f32) -> Arc<egui::Galley> {
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

            // TODO: do not append at the end of the line
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
        if !self.regex_str.is_empty() && self.regex.is_none() {
            self.compile_regex();
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.monospace("Pattern:");
                ui.visuals_mut().extreme_bg_color = self.regex_field_color;
                if ui
                    .add(
                        TextEdit::singleline(&mut self.regex_str)
                            .font(TextStyle::Monospace)
                            .hint_text(".*"),
                    )
                    .changed()
                {
                    self.compile_regex();
                }
            });

            ui.add_space(10.0);

            ui.vertical_centered(|ui| {
                ui.monospace("Haystack: ");
                if ui
                    .add(
                        TextEdit::singleline(&mut self.text)
                            .font(TextStyle::Monospace)
                            .hint_text("Hello world")
                            .layouter(&mut Self::my_layouter),
                    )
                    .changed()
                {
                    self.collect_captures();
                }
            });
        });
    }
}
