use egui::{text::LayoutJob, Color32};

use crate::MatchGroup;

pub fn set_layout(
    text: &str,
    matched_groups: &[Vec<MatchGroup>],
    hovered_group_index: Option<usize>,
) -> LayoutJob {
    if text.is_empty() {
        return LayoutJob::default();
    }

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

#[cfg(test)]
mod tests {
    use egui::text::LayoutSection;

    use super::*;

    // fn layouts_equal(lhs: &LayoutJob, rhs: &LayoutJob) -> bool {
    //     lhs.sections == rhs.sections
    // }

    #[test]
    fn default_layout_when_empty_text() {
        let matched_groups = vec![vec![]];
        let set_layout = set_layout("", &matched_groups, None);
        let expected_layout = LayoutJob::default();

        assert_eq!(set_layout, expected_layout);
    }

    #[test]
    fn single_group() {
        let matched_groups = vec![vec![MatchGroup {
            name: String::from("0"),
            capture: String::from("234"),
            start: 1,
            end: 3,
        }]];
        let set_layout = set_layout("12345", &matched_groups, None);
        let expected_layout = LayoutJob {
            sections: vec![
                LayoutSection {
                    leading_space: 0.0,
                    byte_range: 0..1,
                    format: egui::TextFormat {
                        ..Default::default()
                    },
                },
                LayoutSection {
                    leading_space: 0.0,
                    byte_range: 1..4,
                    format: egui::TextFormat {
                        background: Color32::DARK_BLUE,
                        ..Default::default()
                    },
                },
                LayoutSection {
                    leading_space: 0.0,
                    byte_range: 4..6,
                    format: egui::TextFormat {
                        ..Default::default()
                    },
                },
            ],

            ..Default::default()
        };

        assert_eq!(set_layout.sections, expected_layout.sections);
    }
}
