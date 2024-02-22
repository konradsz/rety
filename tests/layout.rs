use std::ops::Range;

use egui::{
    text::{LayoutJob, LayoutSection},
    Color32,
};
use regex_wasm::{captures::Captures2, layout};

fn section(range: Range<usize>) -> LayoutSection {
    LayoutSection {
        leading_space: 0.0,
        byte_range: range,
        format: egui::TextFormat {
            ..Default::default()
        },
    }
}

fn section_colored(range: Range<usize>) -> LayoutSection {
    LayoutSection {
        leading_space: 0.0,
        byte_range: range,
        format: egui::TextFormat {
            background: Color32::DARK_BLUE,
            ..Default::default()
        },
    }
}

#[test]
fn no_match() {
    let haystack = "Hello world";
    let mut captures = Captures2::default();
    captures.compile_regex("Goodbye world");
    captures.collect_captures_iteratively(haystack);

    let set_layout = layout::set_layout(haystack, &captures.matched_groups(), None);
    let expected_layout = LayoutJob {
        sections: vec![section(0..11)],
        text: haystack.to_string(),
        ..Default::default()
    };

    assert_eq!(set_layout, expected_layout);
}

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
        sections: vec![section(0..1), section_colored(1..4), section(4..5)],
        text: haystack.to_string(),
        ..Default::default()
    };

    assert_eq!(set_layout, expected_layout);
}
