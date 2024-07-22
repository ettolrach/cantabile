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

/// Tuple of full name, short name.
pub const FAMOUS_COMPOSERS: [(&str, &str); 13] = [
    ("Bach", "Johann Sebastian Bach"),
    ("Beethoven", "Ludwig van Beethoven"),
    ("Bizet", "Georges Bizet"),
    ("Chopin", "Frédéric Chopin"),
    ("Dvořák", "Antonín Dvořák"),
    ("Grieg", "Edvard Grieg"),
    ("Mozart", "Wolfgang Amadeus Mozart"),
    ("Schubert", "Franz Schubert"),
    ("Schumann", "Robert Schumann"),
    ("Strauss I", "Johann Strauss I"),
    ("Strauss II", "Johann Strauss II"),
    ("R. Strauss", "Richard Strauss"),
    ("Tchaikovsky", "Pyotr Ilyich Tchaikovsky"),
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Composer {
    pub short_name: String,
    pub full_name: String,
}

impl Composer {
    pub fn from_famous(s: &str) -> Option<Composer> {
        for (short, long) in FAMOUS_COMPOSERS {
            if s == short.trim() || s == long.trim() {
                return Some(Composer { short_name: short.to_owned(), full_name: long.to_owned() })
            }
        }
        None
    }
}

