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
    collections::HashSet, convert::Infallible, io, path::{Path, PathBuf}, str::FromStr
};

use audiotags::{Album, MimeType, Picture, Tag};

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Artists {
    composers: HashSet<Composer>,
    others: HashSet<String>,
}

impl Artists {
    // Constructors.

    /// Creates a new blank Artists struct, ready to be mutated with new values.
    pub fn new() -> Self {
        Artists { composers: HashSet::new(), others: HashSet::new() }
    }

    /// Takes an iterator of artist strings and parses each to create a new [`Artists`] struct.
    fn parse_strings<'a>(strings: impl IntoIterator<Item = &'a str>) -> Self {
        let mut to_return: Self = Self::new();
        for s in strings {
            for artist in s.replace("/", ";").replace("feat.", ";").split(';').map(str::trim) {
                to_return.add_string(artist);
            }

        }
        to_return
    }

    pub fn add_composer(&mut self, composer: Composer) {
        self.composers.insert(composer);
    }

    /// Adds a non-classical composer.
    pub fn add_other(&mut self, other: &str) {
        self.others.insert(other.to_owned());
    }

    /// Adds the string to the artists. This differs from [`Artist::add_other`] by checking if the
    /// input is a classical composer and adding it to the appropriate HashSet.
    pub fn add_string(&mut self, s: &str) {
        if let Some(c) = Composer::from_famous(s) {
            self.composers.insert(c);
        } else {
            self.others.insert(s.to_owned());
        }
    }
}

impl Default for Artists {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct Cover {
    pub data: Vec<u8>,
    pub mime_type: MimeType,
}

impl From<Picture<'_>> for Cover {
    fn from(value: Picture) -> Self {
        Self {
            data: value.data.to_owned(),
            mime_type: value.mime_type,
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// The directory specified in the config.toml file couldn't be found. This could be because
    /// the path is incorrect, or the program is missing read permissions.
    #[error("The directory couldn't be found! Is the supplied path in the config.toml correct?")]
    DirectoryMissing,
    /// Some [`io::Error`] occurred.
    #[error("{0}")]
    DirectoryReadError(#[from] io::Error),
    /// The year metadata is not a [`u16`] number (years for audio files should between 1800 and
    /// CURERNT_YEAR, and we're not quite in the year 65536 yet).
    #[error("Year is invalid! Got the number {0}.")]
    InvalidYear(i32),
    /// A track didn't have an album.
    #[error("Missing album!")]
    MissingAlbum,
    /// A track didn't have an artist.
    #[error("Missing artist!")]
    MissingArtist,
    /// A track didn't have a genre.
    #[error("Missing genre!")]
    MissingGenre,
    /// A track didn't have a position (a.k.a. track number).
    #[error("Missing position!")]
    MissingPosition,
    /// A track didn't have a title.
    #[error("Missing title!")]
    MissingTitle,
}

/// A Track which represents a music file with metadata and album information.
/// 
/// This struct should only be used in the process of creating a database. This will assume that
/// paths are valid.
/// 
/// If a track doesn't have a title, artist, album, and genre, no [`Track`] can be constructed.
#[derive(Clone)]
pub struct TrackMaxi {
    path: PathBuf,
    title: String,
    artists: Artists,
    album_name: String,
    album_cover: Option<Cover>,
    album_artists: Artists,
    genre: String,
    year: Option<u16>,
    position: u16,
}

impl TryFrom<PathBuf> for TrackMaxi {
    type Error = self::Error;
    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        let tag = Tag::new()
            .read_from_path(&value)
            .expect("File metadata invalid.");

        let artists = Artists::parse_strings([
            tag.artist().ok_or(Self::Error::MissingArtist)?,
            tag.album_artist().ok_or(Self::Error::MissingArtist)?,
        ]);

        let album_artists = Artists::parse_strings([
            tag.album_artist()
                .ok_or(Self::Error::MissingArtist)?
        ]);

        let year_i32: Option<i32> = tag.year();
        let year: Option<u16> = if let Some(x) = year_i32 {
            Some(u16::try_from(x).map_err(|_| Self::Error::InvalidYear(x))?)
        } else {
            None
        };

        let album = tag.album().ok_or(Self::Error::MissingAlbum)?;

        Ok(TrackMaxi {
            path: value,
            title: tag.title().ok_or(Self::Error::MissingTitle)?.to_owned(),
            artists,
            album_name: album.title.to_string(),
            album_cover: album.cover.map(Cover::from),
            album_artists,
            genre: tag.genre().unwrap_or("").to_string(),
            year,
            position: tag.track_number().ok_or(Self::Error::MissingPosition)?,
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
