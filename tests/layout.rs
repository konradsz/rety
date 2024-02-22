use egui::{
    text::{LayoutJob, LayoutSection},
    Color32,
};
use regex_wasm::{captures::Captures2, layout};

#[test]
fn default_layout_when_empty_haystack() {
    let haystack = "";
    let mut captures = Captures2::default();
    captures.compile_regex(".*");
    captures.collect_captures_iteratively(haystack);

    let set_layout = layout::set_layout(haystack, &captures.matched_groups(), None);
    let expected_layout = LayoutJob::default();

    assert_eq!(set_layout, expected_layout);
}

#[test]
fn single_group() {
    let haystack = "12345";
    let mut captures = Captures2::default();
    captures.compile_regex("234");
    captures.collect_captures_iteratively(haystack);

    let set_layout = layout::set_layout(haystack, &captures.matched_groups(), None);

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
                byte_range: 4..5,
                format: egui::TextFormat {
                    ..Default::default()
                },
            },
        ],
        text: haystack.to_string(),
        ..Default::default()
    };

    assert_eq!(set_layout, expected_layout);
}
