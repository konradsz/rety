use egui::{text::LayoutJob, Color32};

use crate::{colors::COLORS, MatchGroup};

pub fn set_layout(
    text: &str,
    matched_groups: &[MatchGroup],
    hovered_group_index: Option<usize>,
) -> LayoutJob {
    // TODO: memoize
    let mut layout_job = LayoutJob::default();

    if let Some(group) = matched_groups.get(0) {
        if text.len() >= group.end {
            layout_job.append(
                &text[0..group.start],
                0.0,
                egui::TextFormat {
                    ..Default::default()
                },
            );
            layout_job.append(
                &text[group.start..group.end],
                0.0,
                egui::TextFormat {
                    underline: egui::Stroke::new(2.0, Color32::DARK_RED),
                    ..Default::default()
                },
            );
            layout_job.append(
                &text[group.end..],
                0.0,
                egui::TextFormat {
                    ..Default::default()
                },
            );

            return layout_job;
        }
    }

    // default case, print the whole text
    layout_job.append(
        text,
        0.0,
        egui::TextFormat {
            color: COLORS[2],
            ..Default::default()
        },
    );

    layout_job
}
