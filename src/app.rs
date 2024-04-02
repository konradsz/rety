use egui::{
    Color32, FontFamily, FontId, Label, RichText, Rounding, Stroke, TextEdit, Vec2, Visuals, Widget,
};
use egui_extras::Column;

use crate::{
    group_captures::{GroupCaptures, RegexState},
    layout,
    styles::{self, COLORS},
};

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
        Self {
            regex_str: ".*".to_string(),
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

    fn draw_pattern_textbox(&mut self, ui: &mut egui::Ui) {
        ui.scope(|ui| {
            Label::new(RichText::new("Pattern").monospace().strong())
                .selectable(false)
                .ui(ui);
            ui.add_space(3.0);

            let rounding = styles::TEXT_EDIT_ROUNDING;
            let (active_stroke, inactive_stroke) = match self.captures.get_regex_state() {
                RegexState::Empty => (
                    Stroke::new(1.0, styles::TEXT_EDIT_ACTIVE_STROKE),
                    Stroke::new(1.0, styles::TEXT_EDIT_INACTIVE_STROKE),
                ),
                RegexState::Valid(_) => (
                    Stroke::new(2.0, styles::CORRECT_PATTERN_ACTIVE_STROKE_COLOR),
                    Stroke::new(2.0, styles::CORRECT_PATTERN_INACTIVE_STROKE_COLOR),
                ),
                RegexState::Invalid => (
                    Stroke::new(2.0, styles::INCORRECT_PATTERN_ACTIVE_STROKE_COLOR),
                    Stroke::new(2.0, styles::INCORRECT_PATTERN_INACTIVE_STROKE_COLOR),
                ),
            };
            let text_edit_background = match self.captures.get_regex_state() {
                RegexState::Empty => styles::TEXT_EDIT_BACKGROUND,
                RegexState::Valid(_) => styles::CORRECT_PATTERN_BG_COLOR,
                RegexState::Invalid => styles::INCORRECT_PATTERN_BG_COLOR,
            };

            ui.visuals_mut().widgets.active.bg_stroke = active_stroke;
            ui.visuals_mut().widgets.active.rounding = rounding;
            ui.visuals_mut().widgets.active.expansion = 0.0;
            ui.visuals_mut().extreme_bg_color = text_edit_background;
            ui.visuals_mut().widgets.inactive.bg_stroke = inactive_stroke;
            ui.visuals_mut().widgets.inactive.rounding = rounding;
            ui.visuals_mut().widgets.hovered.bg_stroke = active_stroke;
            ui.visuals_mut().widgets.hovered.rounding = rounding;
            ui.visuals_mut().widgets.hovered.expansion = 0.0;
            ui.visuals_mut().selection.stroke = active_stroke;

            if ui
                .add(
                    TextEdit::singleline(&mut self.regex_str)
                        .font(FontId {
                            size: styles::FONT_SIZE,
                            family: FontFamily::Monospace,
                        })
                        .text_color(styles::TEXT_COLOR)
                        .desired_width(450.0)
                        .margin(Vec2::new(8.0, 5.0)),
                )
                .changed()
            {
                self.captures.compile_regex(&self.regex_str);
                self.captures.collect_captures(&self.text, self.iteratively);
            }

            ui.add_space(10.0);
        });
    }

    fn draw_haystack_textbox(&mut self, ui: &mut egui::Ui) {
        ui.scope(|ui| {
            Label::new(RichText::new("Haystack").monospace().strong())
                .selectable(false)
                .ui(ui);
            ui.add_space(3.0);

            let matched_groups = self.captures.matched_groups();
            let hovered_group_index = self.hovered_group_index;
            let mut layouter = move |ui: &egui::Ui, text: &str, wrap_width: f32| {
                let mut layout_job = layout::set_layout(text, matched_groups, hovered_group_index);
                layout_job.wrap.max_width = wrap_width;
                ui.fonts(|f| f.layout_job(layout_job))
            };

            let rounding = styles::TEXT_EDIT_ROUNDING;
            ui.visuals_mut().widgets.active.bg_stroke =
                Stroke::new(1.0, styles::TEXT_EDIT_ACTIVE_STROKE);
            ui.visuals_mut().widgets.active.rounding = rounding;
            ui.visuals_mut().widgets.active.expansion = 0.0;
            ui.visuals_mut().widgets.inactive.rounding = rounding;
            ui.visuals_mut().widgets.inactive.bg_stroke =
                Stroke::new(1.0, styles::TEXT_EDIT_INACTIVE_STROKE);
            ui.visuals_mut().widgets.hovered.bg_stroke =
                Stroke::new(1.0, styles::TEXT_EDIT_ACTIVE_STROKE);
            ui.visuals_mut().widgets.hovered.rounding = rounding;
            ui.visuals_mut().widgets.hovered.expansion = 0.0;
            ui.visuals_mut().selection.stroke = Stroke::new(1.0, styles::TEXT_EDIT_ACTIVE_STROKE);

            if ui
                .add(
                    TextEdit::multiline(&mut self.text)
                        .desired_width(450.0)
                        .desired_rows(5)
                        .margin(Vec2::new(8.0, 5.0))
                        .layouter(&mut layouter),
                )
                .changed()
            {
                self.captures.collect_captures(&self.text, self.iteratively);
            }

            ui.add_space(10.0);
        });
    }

    fn draw_search_iteratively_checkbox(&mut self, ui: &mut egui::Ui) {
        ui.scope(|ui| {
            ui.style_mut().spacing.icon_spacing = 5.0;
            ui.visuals_mut().widgets.active.rounding = Rounding::same(5.0);
            ui.visuals_mut().widgets.inactive.rounding = Rounding::same(5.0);
            ui.visuals_mut().widgets.hovered.rounding = Rounding::same(5.0);

            if ui
                .checkbox(
                    &mut self.iteratively,
                    RichText::new("Search iteratively").strong(),
                )
                .on_hover_text("Search successive non-overlapping matches in the haystack.")
                .changed()
            {
                self.captures.collect_captures(&self.text, self.iteratively);
            }

            ui.add_space(10.0);
        });
    }

    fn draw_matched_groups(&mut self, ui: &mut egui::Ui) {
        ui.scope(|ui| {
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
                .column(Column::exact(120.0))
                .column(Column::exact(358.0));
            table_builder = table_builder.sense(egui::Sense::click());
            let table = table_builder.header(20.0, |mut header| {
                header.col(|ui| {
                    ui.vertical_centered(|ui| {
                        ui.label(RichText::new("Group name").monospace().strong())
                            .on_hover_text(
                                "The name of the group. If the group is unnamed, its index used instead.",
                            );
                    });
                });

                let matches_column_label = if self.iteratively { "Matches" } else { "Match" };
                header.col(|ui| {
                    ui.vertical_centered(|ui| {
                        ui.label(RichText::new(matches_column_label).monospace().strong());
                    });
                });
            });
            
            table.body(|mut body| {
                for (idx, group) in transposed.iter().enumerate() {
                    body.row(20.0, |mut row| {
                        row.col(|ui| {
                            Label::new(&group[0].name).selectable(false).ui(ui);
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
                                Label::new(text).selectable(false).ui(ui);

                                if idx < group.len() - 1 {
                                    Label::new(", ").selectable(false).ui(ui);
                                }
                            }
                        });

                        if row.response().hovered() {
                            self.hovered_group_index = Some(idx);
                        }
                    });
                }
            });
        });
    }

    fn get_visuals() -> Visuals {
        egui::Visuals {
            panel_fill: styles::BACKGROUND,
            extreme_bg_color: styles::TEXT_EDIT_BACKGROUND,
            ..Default::default()
        }
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
                self.draw_pattern_textbox(ui);
                self.draw_haystack_textbox(ui);
                self.draw_search_iteratively_checkbox(ui);
                self.draw_matched_groups(ui);
            });
        });
    }
}
