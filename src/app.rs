use egui::{Color32, RichText, Stroke, TextEdit, TextStyle, Visuals};
use egui_extras::Column;

use crate::{
    colors::{self, COLORS},
    group_captures::{GroupCaptures, RegexState},
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
    captures: GroupCaptures,
}

impl Default for App {
    fn default() -> Self {
        let regex_str = ".*"; // TODO: rethink
        Self {
            regex_str: regex_str.to_string(),
            text: "Hello world!".to_string(),
            iteratively: false,
            hovered_group_index: None,
            captures: GroupCaptures::default(),
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
        let matched_groups_count = self
            .captures
            .matched_groups()
            .first()
            .map_or(0, |groups| groups.len());
        let mut transposed = vec![Vec::new(); matched_groups_count];
        for groups in self.captures.matched_groups() {
            for (idx, group) in groups.iter().enumerate() {
                transposed[idx].push(group);
            }
        }

        let hovered_row = if let Some(index) = self.hovered_group_index {
            ui.visuals_mut().widgets.hovered.bg_fill = COLORS[index % COLORS.len()];
            true
        } else {
            false
        };
        self.hovered_group_index = None;

        let mut table_builder = egui_extras::TableBuilder::new(ui)
            .striped(true)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(Column::exact(100.0))
            .column(Column::exact(200.0));
        table_builder = table_builder.sense(egui::Sense::click());
        let table = table_builder.header(20.0, |mut header| {
            header.col(|ui| {
                ui.label(RichText::new("Group name").monospace().strong())
                    .on_hover_text(
                        "The name of the group. If the group is unnamed, its index used instead.",
                    );
            });

            let matches_column_label = if self.iteratively { "Matches" } else { "Match" };
            header.col(|ui| {
                ui.label(RichText::new(matches_column_label).monospace().strong());
            });
        });
        table.body(|mut body| {
            for (idx, group) in transposed.iter().enumerate() {
                body.row(20.0, |mut row| {
                    row.col(|ui| {
                        ui.monospace(&group[0].name);
                    });
                    row.col(|ui| {
                        ui.spacing_mut().item_spacing.x = 1.0;

                        for (idx, g) in group.iter().enumerate() {
                            let text = if hovered_row {
                                RichText::new(g.capture.as_str())
                            } else {
                                RichText::new(g.capture.as_str())
                                    .background_color(Color32::from_gray(64))
                            };
                            ui.monospace(text);

                            if idx < group.len() - 1 {
                                ui.label(", ");
                            }
                        }
                    });

                    if row.response().hovered() {
                        self.hovered_group_index = Some(idx);
                    }
                });
            }
        });
    }

    fn get_visuals() -> Visuals {
        let mut visuals = Visuals::default();
        visuals.panel_fill = colors::BACKGROUND;
        visuals.extreme_bg_color = colors::TEXT_EDIT_BACKGROUND;

        visuals
    }
}

impl eframe::App for App {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(Self::get_visuals());

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
                        TextEdit::multiline(&mut self.text)
                            .hint_text("Hello world!")
                            .layouter(&mut layouter),
                    )
                    .changed()
                {
                    self.captures.collect_captures(&self.text, self.iteratively);
                }
            });

            ui.add_space(10.0);

            self.draw_matched_groups(ui);
        });
    }
}
