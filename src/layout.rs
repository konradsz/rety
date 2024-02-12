use egui::{text::LayoutJob, Color32};

use crate::{colors::COLORS, MatchGroup};

pub fn set_layout(
    text: &str,
    matched_groups: &[Vec<MatchGroup>],
    hovered_group_index: Option<usize>,
) -> LayoutJob {
    // TODO: memoize
    let mut layout_job = LayoutJob::default();

    // if let Some(group) = matched_groups.first() {
    //     let group = group.first().unwrap();
    //     if text.len() >= group.end {
    //         if let Some(hovered_group_index) = hovered_group_index {
    //             return highlight_group(
    //                 text,
    //                 &matched_groups[0][hovered_group_index],
    //                 Color32::DARK_BLUE,
    //             );
    //         } else {
    //             return highlight_group(text, group, Color32::DARK_BLUE);
    //         }
    //     }
    // }
    for group in matched_groups {
        if let Some(g) = group.first() {
            if text.len() >= g.end {
                if let Some(hovered_group_index) = hovered_group_index {
                    highlight_group2(
                        &mut layout_job,
                        &text[g.start..g.end],
                        0,
                        text.len(),
                        &group[hovered_group_index],
                        Color32::DARK_BLUE,
                    );
                } else {
                    highlight_group2(
                        &mut layout_job,
                        &text[g.start..g.end],
                        0,
                        text.len(),
                        g,
                        Color32::DARK_BLUE,
                    );
                }
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

// TODO: add tests
fn highlight_group2(
    layout_job: &mut LayoutJob,
    text: &str,
    start_index: usize,
    end_index: usize,
    group: &MatchGroup,
    color: Color32,
) {
    layout_job.append(
        &text[start_index..group.start],
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
        &text[group.end..end_index],
        0.0,
        egui::TextFormat {
            ..Default::default()
        },
    );
}
