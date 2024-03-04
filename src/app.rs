use egui::{RichText, Stroke, TextEdit, TextStyle};
use egui_extras::Column;

use crate::{
    captures::{Captures2, RegexState},
    colors::COLORS,
    layout,
};

const CORRECT_REGEX_COLOR: egui::Color32 = egui::Color32::DARK_GREEN;
const INCORRECT_REGEX_COLOR: egui::Color32 = egui::Color32::DARK_RED;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct App {
    regex_str: String,
    text: String,
    iteratively: bool,
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
            iteratively: false,
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
            app.captures.collect_captures(&app.text, app.iteratively);

            return app;
        }

        Default::default()
    }

    fn draw_matched_groups(&mut self, ui: &mut egui::Ui) {
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

        if let Some(ii) = self.hovered_group_index {
            ui.visuals_mut().widgets.hovered.bg_fill = COLORS[ii % COLORS.len()];
        }
        self.hovered_group_index = None;

        let mut table_builder = egui_extras::TableBuilder::new(ui)
            .striped(true)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(Column::exact(100.0))
            .column(Column::exact(200.0));
        table_builder = table_builder.sense(egui::Sense::click());
        let table = table_builder.header(20.0, |mut header| {
            header.col(|ui| {
                ui.label(RichText::new("name").monospace().strong());
            });
            header.col(|ui| {
                ui.label(RichText::new("match").monospace().strong());
            });
        });
        table.body(|mut body| {
            for (idx, group) in transposed.iter().enumerate() {
                body.row(20.0, |mut row| {
                    row.col(|ui| {
                        ui.label(&group[0].name);
                    });
                    row.col(|ui| {
                        let matches = group
                            .iter()
                            .map(|g| g.capture.as_str())
                            .collect::<Vec<_>>()
                            .join(", ");
                        ui.label(matches);
                    });

                    if row.response().hovered() {
                        self.hovered_group_index = Some(idx);
                    }
                });
            }
        });
    }
}

impl eframe::App for App {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                if ui
                    .checkbox(&mut self.iteratively, "Search iteratively")
                    .changed()
                {
                    self.captures.collect_captures(&self.text, self.iteratively);
                }

                ui.label(RichText::new("Pattern:").monospace().strong());

                match self.captures.get_regex_state() {
                    RegexState::Empty => (),
                    RegexState::Valid(_) => {
                        let stroke = Stroke::new(2.0, CORRECT_REGEX_COLOR);
                        ui.visuals_mut().widgets.inactive.bg_stroke = stroke;
                        ui.visuals_mut().widgets.hovered.bg_stroke = stroke;
                        ui.visuals_mut().selection.stroke = stroke;
                    }
                    RegexState::Invalid => {
                        let stroke = Stroke::new(2.0, INCORRECT_REGEX_COLOR);
                        ui.visuals_mut().widgets.inactive.bg_stroke = stroke;
                        ui.visuals_mut().widgets.hovered.bg_stroke = stroke;
                        ui.visuals_mut().selection.stroke = stroke;
                    }
                };

                if ui
                    .add(
                        TextEdit::singleline(&mut self.regex_str)
                            .font(TextStyle::Monospace)
                            .hint_text(".*"),
                    )
                    .changed()
                {
                    self.captures.compile_regex(&self.regex_str);
                    self.captures.collect_captures(&self.text, self.iteratively);
                }
            });

            ui.add_space(10.0);

            ui.vertical_centered(|ui| {
                ui.label(RichText::new("Haystack:").monospace().strong());

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
                    self.captures.collect_captures(&self.text, self.iteratively);
                }
            });

            self.draw_matched_groups(ui);
        });
    }
}
