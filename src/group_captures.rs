use regex::Regex;

use crate::MatchGroup;

#[derive(Default)]
pub enum RegexState {
    #[default]
    Empty,
    Valid(Regex),
    Invalid,
}

#[derive(Default)]
pub struct GroupCaptures {
    regex_state: RegexState,
    matched_groups: Vec<Vec<MatchGroup>>,
}

impl GroupCaptures {
    pub fn compile_regex(&mut self, pattern: &str) {
        self.matched_groups.clear();

        if pattern.is_empty() {
            self.regex_state = RegexState::Empty;
        } else if let Ok(regex) = Regex::new(pattern) {
            self.regex_state = RegexState::Valid(regex);
        } else {
            self.regex_state = RegexState::Invalid;
        }
    }

    pub fn get_regex_state(&self) -> &RegexState {
        &self.regex_state
    }

    pub fn collect_captures(&mut self, haystack: &str, iteratively: bool) {
        self.matched_groups.clear();

        if let RegexState::Valid(regex) = &self.regex_state {
            let capture_names = regex.capture_names().collect::<Vec<_>>();

            loop {
                let start_from = self
                    .matched_groups
                    .last()
                    .map_or(0, |g| g.first().unwrap().end);

                let haystack = &haystack[start_from..];
                if haystack.is_empty() {
                    break;
                }

                let mut matched_groups = Vec::new();
                let mut locs = regex.capture_locations();
                if regex.captures_read(&mut locs, haystack).is_some() {
                    for (idx, capture_name) in capture_names.iter().enumerate() {
                        let name = capture_name
                            .map(str::to_string)
                            .unwrap_or_else(|| idx.to_string());

                        let (start, end) = locs.get(idx).unwrap();
                        matched_groups.push(MatchGroup::new(
                            name,
                            haystack[start..end].to_string(),
                            start + start_from,
                            end + start_from,
                        ));
                    }
                    self.matched_groups.push(matched_groups);
                } else {
                    break;
                }

                if !iteratively {
                    break;
                }
            }
        }
    }

    pub fn matched_groups(&self) -> &[Vec<MatchGroup>] {
        &self.matched_groups
    }
}
