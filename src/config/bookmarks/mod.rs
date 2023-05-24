pub mod bookmarks_raw;

pub use self::bookmarks_raw::*;

use std::collections::HashMap;

use crossterm::event::Event;

use crate::config::TomlConfigFile;
use crate::util::keyparse;

use super::parse_config_or_default;

pub type Bookmarks = HashMap<crossterm::event::KeyCode, String>;

impl From<BookmarksRaw> for Bookmarks {
    fn from(raw: BookmarksRaw) -> Self {
        let mut raw = raw;
        let map: Bookmarks = raw
            .bookmark
            .drain(..)
            .filter_map(|bookmark| match keyparse::str_to_event(&bookmark.key) {
                Some(event) => Some((event, bookmark.path)),
                None => None,
            })
            .collect();
        map
    }
}

impl TomlConfigFile for Bookmarks {
    fn get_config(file_name: &str) -> Self {
        parse_config_or_default::<BookmarksRaw, Bookmarks>(file_name)
    }
}
