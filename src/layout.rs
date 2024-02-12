use egui::{text::LayoutJob, Color32};

use crate::MatchGroup;

pub fn set_layout(
    text: &str,
    matched_groups: &[Vec<MatchGroup>],
    hovered_group_index: Option<usize>,
) -> LayoutJob {
    // TODO: memoize
    let mut layout_job = LayoutJob::default();

    let mut starting_index = 0;
    for group in matched_groups {
        if let Some(g) = group.first() {
            if text.len() >= g.end {
                if let Some(hovered_group_index) = hovered_group_index {
                    let hovered_group = &group[hovered_group_index];

                    highlight_group(
                        &mut layout_job,
                        text,
                        hovered_group,
                        Color32::DARK_BLUE,
                        starting_index,
                    );

                    layout_job.append(
                        &text[hovered_group.start + hovered_group.capture.len()..g.end],
                        0.0,
                        egui::TextFormat {
                            ..Default::default()
                        },
                    );
                } else {
                    highlight_group(&mut layout_job, text, g, Color32::DARK_BLUE, starting_index);
                }

                starting_index = g.end;
            }
        }
    }

    // print the rest of the text or the whole text if no groups were found
    layout_job.append(
        &text[starting_index..],
        0.0,
        egui::TextFormat {
            ..Default::default()
        },
    );

    layout_job
}

// TODO: add tests
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
