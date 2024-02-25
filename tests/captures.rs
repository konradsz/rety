use regex_wasm::{captures::Captures2, MatchGroup};

#[test]
fn named_groups_non_iteratively() {
    let pattern = r#"(?<y>\d{4})-(?<m>\d{2})-(?<d>\d{2})"#;
    let haystack = "1973-01-05, 1975-08-25 and 1980-10-18";
    let mut captures = Captures2::default();
    captures.compile_regex(pattern);
    captures.collect_captures(haystack, false);

    let expected_groups = [[
        MatchGroup::new("0".into(), "1973-01-05".into(), 0, 10),
        MatchGroup::new("y".into(), "1973".into(), 0, 4),
        MatchGroup::new("m".into(), "01".into(), 5, 7),
        MatchGroup::new("d".into(), "05".into(), 8, 10),
    ]];

    assert_eq!(captures.matched_groups(), expected_groups);
}

#[test]
fn named_groups_iteratively() {
    let pattern = r#"(?<y>\d{4})-(?<m>\d{2})-(?<d>\d{2})"#;
    let haystack = "1973-01-05, 1975-08-25 and 1980-10-18";
    let mut captures = Captures2::default();
    captures.compile_regex(pattern);
    captures.collect_captures(haystack, true);

    let expected_groups = [
        [
            MatchGroup::new("0".into(), "1973-01-05".into(), 0, 10),
            MatchGroup::new("y".into(), "1973".into(), 0, 4),
            MatchGroup::new("m".into(), "01".into(), 5, 7),
            MatchGroup::new("d".into(), "05".into(), 8, 10),
        ],
        [
            MatchGroup::new("0".into(), "1975-08-25".into(), 12, 22),
            MatchGroup::new("y".into(), "1975".into(), 12, 16),
            MatchGroup::new("m".into(), "08".into(), 17, 19),
            MatchGroup::new("d".into(), "25".into(), 20, 22),
        ],
        [
            MatchGroup::new("0".into(), "1980-10-18".into(), 27, 37),
            MatchGroup::new("y".into(), "1980".into(), 27, 31),
            MatchGroup::new("m".into(), "10".into(), 32, 34),
            MatchGroup::new("d".into(), "18".into(), 35, 37),
        ],
    ];

    assert_eq!(captures.matched_groups(), expected_groups);
}
