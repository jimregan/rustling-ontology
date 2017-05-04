//! General purpose ontology based on rustling.
//!
//! Contains detectors for various entities, like numbers, temperatures, dates
//! in french, english, ...
//!
//! ```
//! extern crate rustling;
//! extern crate rustling_ontology;
//!
//! fn main() {
//!     use rustling_ontology::IntegerValue;
//!     use rustling::AttemptTo;
//!
//!     let parser = rustling_ontology::build_parser(rustling_ontology::Lang::EN).unwrap();
//!     let result = parser.parse("twenty-one").unwrap();
//!
//!     assert_eq!(result.len(), 1);
//!     let int:i64 = result[0].value.attempt_to().unwrap();
//!     assert_eq!(21, int);
//!
//!     let int:IntegerValue = result[0].value.attempt_to().unwrap();
//!     assert_eq!(21, int.value);
//! }
//! ```

#[macro_use]
extern crate rustling;
extern crate rustling_ontology_moment as moment;

use std::result;

#[macro_use]
mod macros;
#[allow(dead_code)]
mod helpers;
pub mod dimension;
pub mod en;
pub mod fr;
pub mod es;

/// Enumerates all language supported for the general purpose ontology.
#[derive(Copy,Clone,Debug)]
pub enum Lang {
    /// English
    EN,
    /// French
    FR,
    /// Spanish
    ES,
}

impl std::str::FromStr for Lang {
    type Err = String;
    fn from_str(it: &str) -> result::Result<Lang, Self::Err> {
        match &*it.to_lowercase() {
            "en" => Ok(Lang::EN),
            "fr" => Ok(Lang::FR),
            "es" => Ok(Lang::ES),
            _ => Err(format!("Unknown language {}", it)),
        }
    }
}

impl ::std::string::ToString for Lang {
    fn to_string(&self) -> String {
        match self {
            &Lang::EN => "en".to_string(),
            &Lang::FR => "fr".to_string(),
            &Lang::ES => "es".to_string(),
        }
    }
}

