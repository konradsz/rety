use egui::{Color32, RichText, Stroke, TextEdit, TextStyle};

use crate::{captures::Captures2, colors::COLORS, layout};

const CORRECT_REGEX_COLOR: egui::Color32 = egui::Color32::DARK_GREEN;
const INCORRECT_REGEX_COLOR: egui::Color32 = egui::Color32::DARK_RED;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct App {
    regex_str: String,
    text: String,
    #[serde(skip)]
    hovered_group_index: Option<usize>,
    #[serde(skip)]
    captures: Captures2,
}

impl Default for App {
    fn default() -> Self {
        let regex_str = ".*"; // TODO: rethink
        Self {
            regex_str: regex_str.to_string(),
            text: "Hello world".to_string(),
            hovered_group_index: None,
            captures: Captures2::default(),
        }
    }
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            let mut app: App = eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();

            app.captures.compile_regex(&app.regex_str);
            app.captures.collect_captures(&app.text, true);

            return app;
        }

        Default::default()
    }

    fn draw_matched_groups(&mut self, ui: &mut egui::Ui) {
        self.hovered_group_index = None;

        // transpose the matched groups
        let s = self
            .captures
            .matched_groups()
            .first()
            .map_or(0, |gs| gs.len());
        let mut transposed = vec![Vec::new(); s];
        for groups in self.captures.matched_groups() {
            for (idx, group) in groups.iter().enumerate() {
                transposed[idx].push(group);
            }
        }

        for (idx, group) in transposed.iter().enumerate() {
            let text = RichText::new(format!(
                "{}: {}",
                group[0].name,
                group
                    .iter()
                    .map(|g| g.capture.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ))
            .monospace()
            .color(COLORS[idx % COLORS.len()]);

            // TODO: not working :(
            ui.visuals_mut().widgets.hovered.fg_stroke.color = Color32::WHITE;
            ui.visuals_mut().widgets.hovered.bg_fill = Color32::WHITE;

            if ui.label(text).hovered() {
                ui.visuals_mut().widgets.hovered.bg_fill = Color32::WHITE;
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
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.monospace("Pattern:");

                let regex_field_color = if self.captures.is_regex_valid() {
                    CORRECT_REGEX_COLOR
                } else {
                    INCORRECT_REGEX_COLOR
                };

                let stroke = Stroke::new(2.0, regex_field_color);
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
                    self.captures.compile_regex(&self.regex_str);
                    self.captures.collect_captures(&self.text, true);
                }
            });

            ui.add_space(10.0);

            ui.vertical_centered(|ui| {
                ui.monospace("Haystack: ");

                let matched_groups = self.captures.matched_groups();
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
                    self.captures.collect_captures(&self.text, true);
                }
            });

            self.draw_matched_groups(ui);
        });
    }
}
