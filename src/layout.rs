use egui::{text::LayoutJob, Color32};

use crate::MatchGroup;

pub fn set_layout(
    haystack: &str,
    matched_groups: &[Vec<MatchGroup>],
    hovered_group_index: Option<usize>,
) -> LayoutJob {
    if haystack.is_empty() {
        return LayoutJob::default();
    }

    // TODO: memoize
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
                        Color32::DARK_BLUE,
                        starting_index,
                    );

                    layout_job.append(
                        &haystack[hovered_group.start + hovered_group.capture.len()..g.end],
                        0.0,
                        egui::TextFormat {
                            ..Default::default()
                        },
                    );
                } else {
                    highlight_group(
                        &mut layout_job,
                        haystack,
                        g,
                        Color32::DARK_BLUE,
                        starting_index,
                    );
                }

                starting_index = g.end;
            }
        }
    }

    // print the rest of the text or the whole text if no groups were found
    layout_job.append(
        &haystack[starting_index..],
        0.0,
        egui::TextFormat {
            ..Default::default()
        },
    );

    layout_job
}

fn highlight_group(
    layout_job: &mut LayoutJob,
    text: &str,
    group: &MatchGroup,
    color: Color32,
    start_index: usize,
) {
    layout_job.append(
        &text[start_index..group.start],
        0.0,
        egui::TextFormat {
            ..Default::default()
        },
    );
    layout_job.append(
        &group.capture,
        0.0,
        egui::TextFormat {
            background: color,
            ..Default::default()
        },
    );
}
