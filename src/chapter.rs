use regex_lite::Regex;
use std::cmp::Ordering;
use std::cmp::Ordering::Equal;
use std::cmp::PartialOrd;

#[derive(Clone, PartialEq)]
pub struct Chapter {
    number: f64,
    pub pages: Vec<String>,
}

impl Chapter {
    pub fn new(name: String, pages: Vec<String>) -> Self {
        let number = Self::number(&name).unwrap_or(0.0);
        Self { number, pages }
    }

    pub fn name(&self) -> String {
        if self.number.fract() == 0.0 {
            (self.number as u32).to_string()
        } else {
            format!("{:.1}", self.number)
        }
    }

    fn number(name: &str) -> Option<f64> {
        let expr = Regex::new(r"-?\d+(\.\d+)?").unwrap();
        let match_ = expr.find(name);

        match_?.as_str().parse().ok()
    }
}

impl PartialOrd for Chapter {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Chapter {}

impl Ord for Chapter {
    fn cmp(&self, other: &Self) -> Ordering {
        self.number.partial_cmp(&other.number).unwrap_or(Equal)
    }
}
