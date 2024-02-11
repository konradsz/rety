use regex::Regex;

use crate::MatchGroup;

#[derive(Default)]
pub struct Captures2 {
    regex: Option<Regex>,
    matched_groups: Vec<MatchGroup>,
}

impl Captures2 {
    pub fn compile_regex(&mut self, regex_str: &str) {
        if let Ok(regex) = Regex::new(regex_str) {
            self.regex = Some(regex);
        } else {
            self.regex = None;
            self.matched_groups.clear();
        }
    }

    pub fn is_regex_valid(&self) -> bool {
        self.regex.is_some()
    }

    pub fn collect_captures(&mut self, text: &str) {
        if let Some(regex) = &self.regex {
            let capture_names = regex.capture_names().collect::<Vec<_>>(); // TODO: do it when pattern changed

            let mut matched_groups = Vec::new();
            let mut locs = regex.capture_locations();
            if regex.captures_read(&mut locs, text).is_some() {
                for (idx, capture_name) in capture_names.iter().enumerate() {
                    let name = capture_name
                        .map(str::to_string)
                        .unwrap_or_else(|| idx.to_string());

                    let (start, end) = locs.get(idx).unwrap();
                    matched_groups.push(MatchGroup { name, start, end });
                }
            }

            self.matched_groups = matched_groups;
        } else {
            // TODO: is it necessary? it is already cleared in compile_regex
            self.matched_groups.clear();
        }
    }

    pub fn matched_groups(&self) -> &[MatchGroup] {
        &self.matched_groups
    }
}
