use egui::{text::LayoutJob, Color32, FontFamily, FontId};

use crate::{
    styles::{COLORS, DEFAULT_MATCH_COLOR, FONT_SIZE, TEXT_COLOR, TEXT_COLOR_MATCHED},
    MatchGroup,
};

pub fn set_layout(
    haystack: &str,
    matched_groups: &[Vec<MatchGroup>],
    hovered_group_index: Option<usize>,
) -> LayoutJob {
    if haystack.is_empty() {
        return LayoutJob::default();
    }

    let mut layout_job = LayoutJob::default();

    let mut starting_index = 0;
    for group in matched_groups {
        if let Some(g) = group.first() {
            if haystack.len() >= g.end {
                if let Some(hovered_group_index) = hovered_group_index {
                    let hovered_group = &group[hovered_group_index];

                    highlight_group(
                        &mut layout_job,
                        haystack,
                        hovered_group,
                        COLORS[hovered_group_index % COLORS.len()],
                        starting_index,
                    );

                    layout_job.append(
                        &haystack[hovered_group.start + hovered_group.capture.len()..g.end],
                        0.0,
                        egui::TextFormat {
                            font_id: FontId {
                                family: FontFamily::Monospace,
                                size: FONT_SIZE,
                            },
                            color: TEXT_COLOR,
                            ..Default::default()
                        },
                    );
                } else {
                    highlight_group(
                        &mut layout_job,
                        haystack,
                        g,
                        DEFAULT_MATCH_COLOR,
                        starting_index,
                    );
                }

                starting_index = g.end;
            }
        }
    }

    // print the rest of the text or the whole text if no groups were found
    if starting_index < haystack.len() {
        layout_job.append(
            &haystack[starting_index..],
            0.0,
            egui::TextFormat {
                font_id: FontId {
                    family: FontFamily::Monospace,
                    size: FONT_SIZE,
                },
                color: TEXT_COLOR,
                ..Default::default()
            },
        );
    }

    layout_job
}

fn highlight_group(
    layout_job: &mut LayoutJob,
    text: &str,
    group: &MatchGroup,
    color: Color32,
    start_index: usize,
) {
    if start_index < group.start {
        layout_job.append(
            &text[start_index..group.start],
            0.0,
            egui::TextFormat {
                font_id: FontId {
                    family: FontFamily::Monospace,
                    size: FONT_SIZE,
                },
                color: TEXT_COLOR,
                ..Default::default()
            },
        );
    }

    layout_job.append(
        &group.capture,
        0.0,
        egui::TextFormat {
            font_id: FontId {
                family: FontFamily::Monospace,
                size: FONT_SIZE,
            },
            color: TEXT_COLOR_MATCHED,
            background: color,
            ..Default::default()
        },
    );
}
