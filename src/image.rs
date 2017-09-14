//! Camera images.
//!
//! Images are sometimes inside the RiPROCESS project tree, in `04_CAM_RAW/03_IMG`, and sometimes
//! in an external folder.

use Result;
use regex::Regex;
use std::path::PathBuf;

lazy_static! {
    static ref FILENAME_REGEX: Regex = Regex::new(r"^DSC\d{5}.JPG$").unwrap();
}

/// Configuration for a set of images.
#[derive(Debug, Default, Deserialize)]
pub struct Config {
    /// The directory that holds the images.
    pub path: PathBuf,
    /// The number of the first image to be used.
    ///
    /// If None, the first image in the directory is used.
    pub first: Option<usize>,
    /// The number of the last image to be used.
    ///
    /// If none, the last image in the directory is used.
    pub last: Option<usize>,
}

impl Config {
    /// Creates a new, default configuration.
    ///
    /// # Examples
    ///
    /// ```
    /// use riprocess::image::Config;
    /// let config = Config::new();
    /// ```
    pub fn new() -> Config {
        Default::default()
    }

    /// Returns the image paths for this configuration.
    ///
    /// The paths can be limited by the `first` and `last` attributes of the configuration. If the
    /// `first` or `last` values do not exist in the image directory, an error is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use riprocess::image::Config;
    /// let config = Config { path: "data/images".into(), ..Default::default() };
    /// let paths = config.paths().unwrap();
    /// ```
    pub fn paths(&self) -> Result<Vec<PathBuf>> {
        use std::fs::DirEntry;
        use std::io::Result;

        let select_paths = |result: Result<DirEntry>| match result {
            Ok(dir_entry) => {
                if FILENAME_REGEX.is_match(&dir_entry.file_name().to_string_lossy().into_owned()) {
                    Some(dir_entry.path())
                } else {
                    None
                }
            }
            Err(_) => None,
        };
        Ok(self.path
               .read_dir()?
               .filter_map(select_paths)
               .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_images() {
        let config = Config { path: "data".into(), ..Default::default() };
        assert!(config.paths().unwrap().is_empty());
    }

    #[test]
    fn all_images() {
        let config = Config { path: "data/images".into(), ..Default::default() };
        assert_eq!(7, config.paths().unwrap().len());
    }
}
