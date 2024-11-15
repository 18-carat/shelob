#![forbid(unsafe_code)]

mod chapter;
mod downloader;
mod either;
mod mapper;
mod metadata;
mod series;
mod spider;

use crate::metadata::Metadata;
use crate::spider::crawl_series;
use boml::prelude::Toml;

fn main() {
    let toml_str = include_str!("../test.toml");
    let toml = Toml::parse(toml_str).unwrap();

    let series: Vec<_> = toml
        .iter()
        .map(|(key, val)| {
            let metadata = Metadata::new(key.to_string(), val.table().unwrap());
            crawl_series(metadata.unwrap())
        })
        .collect();

    series.iter().for_each(|s| s.download());
}
