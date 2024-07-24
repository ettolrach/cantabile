// This file is part of catabile.
//
// cantabile is free software: you can redistribute it and/or modify it under the terms of the GNU
// General Public License as published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// cantabile is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without
// even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with cantabile. If not,
// see <https://www.gnu.org/licenses/>.

/// Tuple of full name, short name.
pub const FAMOUS_COMPOSERS: [(&str, &str); 23] = [
    ("Bach", "Johann Sebastian Bach"),
    ("Beethoven", "Ludwig van Beethoven"),
    ("Bizet", "Georges Bizet"),
    ("Borodin", "Alexander Borodin"),
    ("Chopin", "Frédéric Chopin"),
    ("Dvořák", "Antonín Dvořák"),
    ("Elgar", "Edward Elgar"),
    ("Grieg", "Edvard Grieg"),
    ("Mendelssohn", "Felix Mendelssohn"),
    ("Mozart", "Wolfgang Amadeus Mozart"),
    ("Rachmaninoff", "Sergei Rachmaninoff"),
    ("Rossini", "Gioachino Rossini"),
    ("Schubert", "Franz Schubert"),
    ("Clara Schumann", "Clara Schumann"),
    ("Schumann", "Robert Schumann"),
    ("Shchedrin", "Rodrigo Shchedrin"),
    ("Shostakovich", "Dmitri Shostakovich"),
    ("Strauss I", "Johann Strauss I"),
    ("Strauss II", "Johann Strauss II"),
    ("R. Strauss", "Richard Strauss"),
    ("Sullivan", "Arthur Sullivan"),
    ("Tchaikovsky", "Pyotr Ilyich Tchaikovsky"),
    ("Vivaldi", "Antonio Vivaldi"),
];

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Composer {
    pub short_name: String,
    pub full_name: String,
}

impl Composer {
    #[must_use]
    pub fn from_famous(s: &str) -> Option<Composer> {
        for (short, long) in FAMOUS_COMPOSERS {
            if s == short.trim() || s == long.trim() {
                return Some(Composer { short_name: short.to_owned(), full_name: long.to_owned() })
            }
        }
        None
    }
}

