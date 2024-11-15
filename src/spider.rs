use crate::chapter::Chapter;
use crate::downloader::download;
use crate::either::Either::Left;
use crate::metadata::Metadata;
use crate::series::Series;
use progression::{Bar, Config};
use regex_lite::Regex;

pub fn crawl_series(metadata: Metadata) -> Series {
    println!("Crawling chapters in {}", metadata.title);

    let bar = Bar::new(metadata.chapter_ids.len() as u64, Config::unicode());

    let chapters = metadata
        .chapter_ids
        .iter()
        .map(|id| format!("https://dto.to/chapter/{}", id))
        .fold(Vec::new(), |mut acc, url| {
            acc.push(crawl(&url));
            bar.inc(1);
            acc
        })
        .into_iter()
        .flatten()
        .collect();

    Series::new(metadata, chapters)
}

fn crawl(url: &str) -> Option<Chapter> {
    let bytes = download(url, Vec::new(), true)?;

    let html = match bytes {
        Left(bytes) => String::from_utf8(bytes).ok()?,
        _ => panic!("crawl() called download() and got Right case, expected Left"),
    };

    let html = html.split("const yourEmail").next()?;
    let pages = get_pages(html)?;
    let title = get_title(html)?;

    Some(Chapter::new(title, pages))
}

fn get_pages(html: &str) -> Option<Vec<String>> {
    let expr = Regex::new(r#"imgHttps = \[(.*)\]"#).ok()?;
    let img_https = expr.find(html)?.as_str();
    let pages = &img_https[0xD..img_https.len() - 2];

    Some(pages.split("\",\"").map(|s| s.to_string()).collect())
}

fn get_title(html: &str) -> Option<String> {
    let expr = Regex::new(r#"const local_text_epi = '(.*)'"#).ok()?;
    let var = expr.find(html)?.as_str();

    Some(var[0x18..var.len() - 1].to_string())
}
