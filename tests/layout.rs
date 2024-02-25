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
    let pattern = "Goodbye world";
    let haystack = "Hello world";
    let mut captures = Captures2::default();
    captures.compile_regex(pattern);
    captures.collect_captures(haystack, true);

    let set_layout = layout::set_layout(haystack, &captures.matched_groups(), None);
    let expected_layout = LayoutJob {
        sections: vec![section(0..11)],
        text: haystack.to_string(),
        ..Default::default()
    };

    assert_eq!(set_layout, expected_layout);
}

#[test]
fn empty_pattern() {
    let pattern = "";
    let haystack = "Hello world";
    let mut captures = Captures2::default();
    captures.compile_regex(pattern);
    captures.collect_captures(haystack, true);

    let set_layout = layout::set_layout(haystack, &captures.matched_groups(), None);
    let expected_layout = LayoutJob {
        sections: vec![section(0..11)],
        text: haystack.to_string(),
        ..Default::default()
    };

    assert_eq!(set_layout, expected_layout);
}

#[test]
fn full_match() {
    let pattern = "Hello world";
    let haystack = "Hello world";
    let mut captures = Captures2::default();
    captures.compile_regex(pattern);
    captures.collect_captures(haystack, true);

    let set_layout = layout::set_layout(haystack, &captures.matched_groups(), None);
    let expected_layout = LayoutJob {
        sections: vec![section_colored(0..11)],
        text: haystack.to_string(),
        ..Default::default()
    };

    assert_eq!(set_layout, expected_layout);
}

#[test]
fn default_layout_when_empty_haystack() {
    let pattern = ".*";
    let haystack = "";
    let mut captures = Captures2::default();
    captures.compile_regex(pattern);
    captures.collect_captures(haystack, true);

    let set_layout = layout::set_layout(haystack, &captures.matched_groups(), None);
    let expected_layout = LayoutJob::default();

    assert_eq!(set_layout, expected_layout);
}

#[test]
fn single_group() {
    let pattern = "234";
    let haystack = "12345";
    let mut captures = Captures2::default();
    captures.compile_regex(pattern);
    captures.collect_captures(haystack, true);

    let set_layout = layout::set_layout(haystack, &captures.matched_groups(), None);

    let expected_layout = LayoutJob {
        sections: vec![section(0..1), section_colored(1..4), section(4..5)],
        text: haystack.to_string(),
        ..Default::default()
    };

    assert_eq!(set_layout, expected_layout);
}

#[test]
fn dot_pattern_non_iteratively() {
    let pattern = ".";
    let haystack = "abc";
    let mut captures = Captures2::default();
    captures.compile_regex(pattern);
    captures.collect_captures(haystack, false);

    let set_layout = layout::set_layout(haystack, &captures.matched_groups(), None);

    let expected_layout = LayoutJob {
        sections: vec![section_colored(0..1), section(1..3)],
        text: haystack.to_string(),
        ..Default::default()
    };

    assert_eq!(set_layout, expected_layout);
}

#[test]
fn dot_pattern_iteratively() {
    let pattern = ".";
    let haystack = "abc";
    let mut captures = Captures2::default();
    captures.compile_regex(pattern);
    captures.collect_captures(haystack, true);

    let set_layout = layout::set_layout(haystack, &captures.matched_groups(), None);

    let expected_layout = LayoutJob {
        sections: vec![
            section_colored(0..1),
            section_colored(1..2),
            section_colored(2..3),
        ],
        text: haystack.to_string(),
        ..Default::default()
    };

    assert_eq!(set_layout, expected_layout);
}

#[test]
fn named_groups_non_iteratively() {
    let pattern = r#"(?<y>\d{4})-(?<m>\d{2})-(?<d>\d{2})"#;
    let haystack = "1973-01-05, 1975-08-25 and 1980-10-18";
    let mut captures = Captures2::default();
    captures.compile_regex(pattern);
    captures.collect_captures(haystack, false);

    let set_layout = layout::set_layout(haystack, &captures.matched_groups(), None);

    let expected_layout = LayoutJob {
        sections: vec![section_colored(0..10), section(10..37)],
        text: haystack.to_string(),
        ..Default::default()
    };

    assert_eq!(set_layout, expected_layout);
}

#[test]
fn named_groups_iteratively() {
    let pattern = r#"(?<y>\d{4})-(?<m>\d{2})-(?<d>\d{2})"#;
    let haystack = "1973-01-05, 1975-08-25 and 1980-10-18";
    let mut captures = Captures2::default();
    captures.compile_regex(pattern);
    captures.collect_captures(haystack, true);

    let set_layout = layout::set_layout(haystack, &captures.matched_groups(), None);

    let expected_layout = LayoutJob {
        sections: vec![
            section_colored(0..10),
            section(10..12),
            section_colored(12..22),
            section(22..27),
            section_colored(27..37),
        ],
        text: haystack.to_string(),
        ..Default::default()
    };

    assert_eq!(set_layout, expected_layout);
}

// TODO: test with whitespace
