use regex::Regex;

use crate::MatchGroup;

#[derive(Default)]
pub struct Captures2 {
    regex: Option<Regex>,
    matched_groups: Vec<Vec<MatchGroup>>,
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
            if regex.captures_read(&mut locs, &text[12..]).is_some() {
                for (idx, capture_name) in capture_names.iter().enumerate() {
                    let name = capture_name
                        .map(str::to_string)
                        .unwrap_or_else(|| idx.to_string());

                    let (start, end) = locs.get(idx).unwrap();
                    matched_groups.push(MatchGroup {
                        name,
                        start: start + 12,
                        end: end + 12,
                    });
                }
            }

            self.matched_groups.push(matched_groups);
        } else {
            // TODO: is it necessary? it is already cleared in compile_regex
            self.matched_groups.clear();
        }
    }

    pub fn collect_captures_iteratively(&mut self, text: &str) {
        // self.collect_captures(text);
        self.matched_groups.clear(); // TODO: needed here?

        if let Some(regex) = &self.regex {
            let capture_names = regex.capture_names().collect::<Vec<_>>(); // TODO: do it when pattern changed

            loop {
                let start_from = self
                    .matched_groups
                    .last()
                    .map_or(0, |g| g.first().unwrap().end); // TODO: unwrap

                let mut matched_groups = Vec::new();
                let mut locs = regex.capture_locations();
                if regex
                    .captures_read(&mut locs, &text[start_from..])
                    .is_some()
                {
                    for (idx, capture_name) in capture_names.iter().enumerate() {
                        let name = capture_name
                            .map(str::to_string)
                            .unwrap_or_else(|| idx.to_string());

                        let (start, end) = locs.get(idx).unwrap();
                        matched_groups.push(MatchGroup {
                            name,
                            start: start + start_from,
                            end: end + start_from,
                        });
                    }
                    self.matched_groups.push(matched_groups);
                } else {
                    break;
                }
            }
        } else {
            // TODO: is it necessary? it is already cleared in compile_regex
            self.matched_groups.clear();
        }
    }

    pub fn matched_groups(&self) -> &[Vec<MatchGroup>] {
        // if !self.matched_groups.is_empty() {
        //     &self.matched_groups[0]
        // } else {
        //     &[]
        // }
        &self.matched_groups
    }
}
