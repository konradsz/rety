use egui::{text::LayoutJob, Color32};

use crate::{colors::COLORS, MatchGroup};

pub fn set_layout(
    text: &str,
    matched_groups: &[MatchGroup],
    hovered_group_index: Option<usize>,
) -> LayoutJob {
    // TODO: memoize
    let mut layout_job = LayoutJob::default();

    if let Some(group) = matched_groups.first() {
        if text.len() >= group.end {
            if let Some(hovered_group_index) = hovered_group_index {
                return highlight_group(
                    text,
                    &matched_groups[hovered_group_index],
                    Color32::DARK_BLUE,
                );
            } else {
                return highlight_group(text, group, Color32::DARK_BLUE);
            }
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

fn highlight_group(text: &str, group: &MatchGroup, color: Color32) -> LayoutJob {
    let mut layout_job = LayoutJob::default();

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
            background: color,
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

    layout_job
}
