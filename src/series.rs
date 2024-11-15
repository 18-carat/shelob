use crate::chapter::Chapter;
use crate::downloader::download;
use crate::metadata::Metadata;
use progression::{Bar, Config};
use std::fs::{create_dir_all, File};

pub struct Series {
    metadata: Metadata,
    chapters: Vec<Chapter>,
}

impl Series {
    pub fn new(metadata: Metadata, chapters: Vec<Chapter>) -> Self {
        Self { metadata, chapters }
    }

    fn sorted_chapters(&self) -> Vec<Chapter> {
        let mut chapters = self.chapters.clone();
        chapters.sort();
        chapters
    }

    pub fn download(&self) {
        println!("Downloading pages for {}", self.metadata.title);

        let hash = format!("{:x}", self.metadata.id);
        let page_count: usize = self.sorted_chapters().iter().map(|c| c.pages.len()).sum();
        let bar = Bar::new(page_count as u64, Config::unicode());

        self.sorted_chapters().iter().for_each(|chapter| {
            chapter.pages.iter().enumerate().for_each(|(i, url)| {
                create_dir_all(format!("{}/{}", hash, chapter.name())).unwrap();

                let file_name = format!("{}/{}/{}.webp", hash, chapter.name(), i);
                let file = File::create(file_name).unwrap();

                download(url, file, false);
                bar.inc(1);
            });
        });
    }
}
