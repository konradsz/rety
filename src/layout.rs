use egui::{text::LayoutJob, Color32};

use crate::MatchGroup;

pub fn set_layout(
    text: &str,
    _matched_groups: &[MatchGroup],
    _hovered_group_index: Option<usize>,
) -> LayoutJob {
    // TODO: memoize
    let mut layout_job = LayoutJob::default();

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
        layout_job.append(
            word,
            0.0,
            egui::TextFormat {
                color: COLORS[index % COLORS.len()],
                ..Default::default()
            },
        );

        // TODO: do not append at the end of the line
        layout_job.append(
            " ",
            0.0,
            egui::TextFormat {
                color: COLORS[index % COLORS.len()],
                ..Default::default()
            },
        );
    }

    layout_job
}
