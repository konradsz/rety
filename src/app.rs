use egui::{Stroke, TextEdit, TextStyle};
use regex::Regex;

use crate::{layout, MatchGroup};

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
    #[serde(skip)]
    hovered_group_index: Option<usize>,
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
            hovered_group_index: None,
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

    fn compile_regex(&mut self) {
        if let Ok(regex) = Regex::new(&self.regex_str) {
            self.regex = Some(regex);
            self.collect_captures();
            self.regex_field_color = CORRECT_REGEX_COLOR;
        } else {
            self.regex_field_color = INCORRECT_REGEX_COLOR;
        }
    }

    fn collect_captures(&mut self) {
        if let Some(regex) = &self.regex {
            let capture_names = regex.capture_names().collect::<Vec<_>>(); // TODO: do it when pattern changed

            let mut matched_groups = Vec::new();
            let mut locs = regex.capture_locations();
            if regex.captures_read(&mut locs, &self.text).is_some() {
                for (idx, capture_name) in capture_names.iter().enumerate() {
                    let name = capture_name
                        .map(str::to_string)
                        .unwrap_or_else(|| idx.to_string());

                    let (start, end) = locs.get(idx).unwrap();
                    matched_groups.push(MatchGroup { name, start, end });
                }
            }

            self.matched_groups = matched_groups;
        }
    }

    fn draw_matched_groups(&mut self, ui: &mut egui::Ui) {
        self.hovered_group_index = None;

        for (idx, group) in self.matched_groups.iter().enumerate() {
            if ui
                .monospace(format!(
                    "{}: {}",
                    group.name,
                    &self.text[group.start..group.end]
                ))
                .hovered()
            {
                self.hovered_group_index = Some(idx);
            }
        }
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

                let stroke = Stroke::new(2.0, self.regex_field_color);
                ui.visuals_mut().widgets.inactive.bg_stroke = stroke;
                ui.visuals_mut().widgets.hovered.bg_stroke = stroke;
                ui.visuals_mut().selection.stroke = stroke;

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

                let matched_groups = &self.matched_groups;
                let hovered_group_index = self.hovered_group_index;
                let mut layouter = move |ui: &egui::Ui, text: &str, wrap_width: f32| {
                    let mut layout_job =
                        layout::set_layout(text, matched_groups, hovered_group_index);
                    layout_job.wrap.max_width = wrap_width;
                    ui.fonts(|f| f.layout_job(layout_job))
                };

                if ui
                    .add(
                        TextEdit::singleline(&mut self.text)
                            .font(TextStyle::Monospace)
                            .hint_text("Hello world")
                            .layouter(&mut layouter),
                    )
                    .changed()
                {
                    self.collect_captures();
                }
            });

            self.draw_matched_groups(ui);
        });
    }
}
