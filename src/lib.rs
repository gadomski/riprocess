//! Helper tools for RiPROCESS.
//!
//! RiPROCESS, by [Riegl](http://riegl.com/), is used to process airborne and UAS LiDAR data. This
//! library provides utilities for working with RiPROCESS.
//!
//! Specifically, the camera data import wizard in RiPROCESS makes mistakes with our RiCOPTER data.
//! This library includes tools for generating timestamp+image lists that can be imported into
//! RiPROCESS in order to bypass the built-in camera wizard.

#![deny(missing_docs, missing_debug_implementations, missing_copy_implementations, trivial_casts,
        trivial_numeric_casts, unsafe_code, unstable_features, unused_import_braces,
        unused_qualifications)]

#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate toml;

mod config;
pub mod image;
pub mod record;
pub mod timestamp;

pub use config::Config;

/// Our custom error enum.
#[derive(Debug)]
pub enum Error {
    /// The image number, as provided in configuration, is invalid.
    ///
    /// Usually means that there wasn't a file with that image number.
    InvalidImageNumber(usize),
    /// Wrapper around `std::io::Error`.
    Io(std::io::Error),
    /// The are no images with the expected naming structure in the provided path.
    NoImages(std::path::PathBuf),
    /// Wrapper around `std::num::ParseFloatError`.
    ParseFloat(std::num::ParseFloatError),
    /// Wrapper around `std::num::ParseIntError`.
    ParseInt(std::num::ParseIntError),
    /// The timestamp and record counts don't match.
    RecordCountMismatch {
        /// The number of timestamp files.
        timestamps: usize,
        /// The number of records.
        records: usize,
    },
    /// The timestamp and image counts don't match.
    TimestampCountMismatch {
        /// The number of timestamps.
        timestamps: usize,
        /// The number of images.
        images: usize,
    },
    /// Wrapper around `toml::de::Error`.
    TomlDe(toml::de::Error),
}

/// Our custom result type.
pub type Result<T> = std::result::Result<T, Error>;

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<std::num::ParseFloatError> for Error {
    fn from(err: std::num::ParseFloatError) -> Error {
        Error::ParseFloat(err)
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(err: std::num::ParseIntError) -> Error {
        Error::ParseInt(err)
    }
}

impl From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Error {
        Error::TomlDe(err)
    }
}
