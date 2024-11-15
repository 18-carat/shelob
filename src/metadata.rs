use boml::prelude::TomlTable;
use seahash::hash;

#[allow(dead_code)]
pub struct Metadata {
    pub id: u64,
    pub title: String,
    cover: String,
    author: String,
    artist: String,
    description: String,
    pub chapter_ids: Vec<u64>,
}

impl Metadata {
    pub fn new(title: String, toml: &TomlTable) -> Option<Self> {
        let cover = toml.get_string("cover").ok()?.to_string();
        let author = toml.get_string("author").ok()?.to_string();
        let artist = toml.get_string("artist").ok()?.to_string();
        let description = toml.get_string("description").ok()?.to_string();
        let id = hash(title.as_bytes());

        let chapter_ids = toml
            .get_array("chapter_ids")
            .ok()?
            .iter()
            .filter_map(|id| id.integer())
            .map(|id| id as u64)
            .collect();

        Some(Self {
            id,
            title,
            cover,
            author,
            artist,
            description,
            chapter_ids,
        })
    }
}
