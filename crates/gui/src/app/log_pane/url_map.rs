use std::collections::HashMap;

use bookmark_library::Bookmark;

use super::IntoMetricValue;

#[derive(Clone, Debug, Default)]
pub struct UrlMap {
    pub occurance_count: HashMap<String, usize>,
    pub parse_issues: Vec<(Bookmark, url::ParseError)>,
}

#[derive(Clone, Copy, Debug)]
pub struct Tally {
    pub parse_issues: usize,
    pub no_domain: usize,
    pub domains: usize,
}

impl UrlMap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tally(&self) -> Tally {
        let no_domain = self.occurance_count.get("<no domain>").copied();
        let domain_count = self.occurance_count.keys().len() - usize::from(no_domain.is_some());

        Tally {
            parse_issues: self.parse_issues.len(),
            no_domain: no_domain.unwrap_or(0),
            domains: domain_count,
        }
    }
}

impl IntoMetricValue for UrlMap {
    fn into_metric_value(self) -> super::MetricValue {
        super::MetricValue::UrlMap(self)
    }
}
