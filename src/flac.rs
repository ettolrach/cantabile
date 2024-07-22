// This file is part of Jukie.
//
// Jukie is free software: you can redistribute it and/or modify it under the terms of the GNU
// General Public License as published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// Jukie is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even
// the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General
// Public License for more details.
//
// You should have received a copy of the GNU General Public License along with Jukie. If not, see
// <https://www.gnu.org/licenses/>.

use crate::composers::{Composer, FAMOUS_COMPOSERS};

use std::{
    io,
    path::{Path, PathBuf},
};

use audiotags::Tag;

fn parse_album_artist(s: &str) -> (Vec<Composer>, Vec<String>) {
    let mut composers: Vec<Composer> = Vec::new();
    let mut artists: Vec<String> = Vec::new();

    for artist in s.split(';') {
        if let Some(composer) = Composer::from_famous(artist.trim()) {
            composers.push(composer);
        } else {
            artists.push(artist.to_owned());
        }
    }

    (composers, artists)
}

enum Artist {
    ClassicalComposer(Composer),
    Other(String),
}

impl From<Composer> for Artist {
    fn from(value: Composer) -> Self {
        Self::ClassicalComposer(value)
    }
}

/// The position of a track on a CD. So, the 4th track on a 13 track CD is `[4, 13]`.
type CdPosition = [u8; 2];

#[derive(thiserror::Error, Debug)]
enum Error {
    /// The directory specified in the config.toml file couldn't be found. This could be because
    /// the path is incorrect, or the program is missing read permissions.
    #[error("The directory couldn't be found! Is the supplied path in the config.toml correct?")]
    DirectoryMissing,
    /// Some [`io::Error`] occurred.
    #[error("{0}")]
    DirectoryReadError(#[from] io::Error),
    /// If a track doesn't have a title, artist, album, and genre, no [`Track`] can be constructed.
    #[error("Missing title, artist, album, or genre!")]
    NotEnoughInfo,
}

/// This is a FLAC struct.
///
/// This will assume that paths are valid.
#[derive(Default)]
pub struct Track {
    path: PathBuf,
    title: String,
    artists: Vec<Artist>,
    performer: Vec<String>,
    album: String,
    genre: String,
    year: Option<u16>,
    position: CdPosition,
}

impl TryFrom<PathBuf> for Track {
    type Error = self::Error;
    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        let mut tag = Tag::new()
            .read_from_path(value)
            .expect("File metadata invalid.");

        let mut artists: Vec<Artist> = Vec::new();

        let artist_string: String = tag.artist().ok_or(Self::Error::NotEnoughInfo)?.to_owned();
        let mut composer: Option<Composer> = Composer::from_famous(&artist_string);
        let mut performer: Vec<String> = Vec::new();

        if let Some(c) = composer {
            artists.push(Artist::ClassicalComposer(c));
            if let Some(album_artist_string) = tag.album_artist() {
                let (composer_vec, performer_vec) = parse_album_artist(album_artist_string);
                performer.append(&mut performer_vec);
                let mut composer_vec: Vec<Artist> = composer_vec
                    .into_iter()
                    .filter(|s| s == &c)
                    .map(From::from)
                    .collect();
                artists.append(&mut composer_vec);
            }
        } else {
            artists.
        }

        if let Some(s) = tag.album_artist() {

        }
        Ok(Track {
            path: value,
            title: tag.title().unwrap_or("").to_string(),
            artist: tag.artist().unwrap_or("").to_string(),
            album: tag.album().unwrap_or("").to_string(),
            genre: tag.genre().unwrap_or("").to_string(),
            year: tag.year().unwrap_or("").to_string(),
            position: tag.track_number().unwrap_or("").to_string(),
        })
    }
}

fn find_flac_paths(path: &Path) -> Result<Vec<PathBuf>, Error> {
    let mut to_return: Vec<PathBuf> = Vec::new();
    for entry in path.read_dir()? {
        let entry_path = entry?.path();
        if entry_path.is_file() {
            if let Some(s) = entry_path.extension() {
                if ["flac", "mp3"].contains(
                    &s.to_str()
                        .expect("File contains invalid UTF-8.")
                        .to_lowercase()
                        .as_str(),
                ) {
                    to_return.push(entry_path);
                }
            }
        } else {
            to_return.append(&mut find_flac_paths(&entry_path)?);
        }
    }
    Ok(to_return)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find() {
        let archive_path = PathBuf::from(r"/mnt/Archive_HDD/MusicRAW/");
        let flacs = find_flac_paths(&archive_path).unwrap();
        for f in flacs {
            println!("{f:?}");
        }
    }
}
