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

use std::{
    io,
    path::{Path, PathBuf},
};

use serde::Deserialize;
use thiserror::Error;

#[derive(Deserialize)]
struct SpotifySecrets {
    access_token: String,
    client_id: String,
}

#[derive(Deserialize)]
struct PathsToml {
    paths: Vec<String>,
}

#[derive(Deserialize)]
struct Config {
    spotify: SpotifySecrets,
    local: PathsToml,
}

// struct Directories(Vec<PathBuf>);

// impl Directories {
//     /// Checks if the path exists and is readable.
//     fn verify_path(&self) -> Result<(), Error> {
//         self.0.iter().try_for_each(|path| {
//             path.try_exists().map_or_else(
//                 |io_error| Err(Error::from(io_error)),
//                 |b| {
//                     if !b {
//                         Err(Error::DirectoryMissing)
//                     } else {
//                         Ok(())
//                     }
//                 },
//             )
//         })
//     }
// }
