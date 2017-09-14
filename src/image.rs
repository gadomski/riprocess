//! Camera images.
//!
//! Images are sometimes inside the RiPROCESS project tree, in `04_CAM_RAW/03_IMG`, and sometimes
//! in an external folder.

use Result;
use regex::Regex;
use std::ffi::OsStr;
use std::path::PathBuf;

lazy_static! {
    static ref FILE_NAME_REGEX: Regex = Regex::new(r"^DSC(?P<image_number>\d{5}).JPG$").unwrap();
}

/// Configuration for a set of images.
#[derive(Debug, Default, Deserialize)]
pub struct Config {
    /// The directory that holds the images.
    pub path: PathBuf,
    /// The number of the first image to be used.
    ///
    /// If None, the first image in the directory is used.
    pub start: Option<usize>,
    /// The number of the last image to be used.
    ///
    /// If none, the last image in the directory is used.
    pub end: Option<usize>,
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
    /// The paths can be limited by the `start` and `end` attributes of the configuration. If the
    /// `start` or `end` values do not exist in the image directory, an error is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use riprocess::image::Config;
    /// let mut config = Config { path: "data/images".into(), ..Default::default() };
    /// let paths = config.paths().unwrap();
    /// config.start = Some(4242); // <- not a image number in the directory
    /// assert!(config.paths().is_err());
    /// ```
    pub fn paths(&self) -> Result<Vec<PathBuf>> {
        use Error;
        use std::fs::DirEntry;
        use std::io::Result;

        let mut image_numbers = Vec::new();
        let mut paths: Vec<PathBuf>;
        {
            let select_paths = |result: Result<DirEntry>| match result {
                Ok(dir_entry) => {
                    if let Some(image_number) = extract_image_number(&dir_entry.file_name()) {
                        image_numbers.push(image_number);
                        if self.image_number_is_in_range(image_number) {
                            return Some(dir_entry.path());
                        }
                    }
                    None
                }
                Err(_) => None,
            };
            paths = self.path
                .canonicalize()?
                .read_dir()?
                .filter_map(select_paths)
                .collect();
        }
        if let Some(start) = self.start {
            if !image_numbers.contains(&start) {
                return Err(Error::InvalidImageNumber(start));
            }
        }
        if let Some(end) = self.end {
            if !image_numbers.contains(&end) {
                return Err(Error::InvalidImageNumber(end));
            }
        }
        paths.sort();
        Ok(paths)
    }

    fn image_number_is_in_range(&self, image_number: usize) -> bool {
        self.start.map(|start| start <= image_number).unwrap_or(true) &&
        self.end.map(|end| end >= image_number).unwrap_or(true)
    }
}

fn extract_image_number(file_name: &OsStr) -> Option<usize> {
    file_name.to_str()
        .and_then(|file_name| FILE_NAME_REGEX.captures(file_name))
        .map(|captures| {
            captures.name("image_number")
                .expect("FILE_NAME_REGEX should have an image_number named pattern")
                .as_str()
                .parse()
                .expect("\\d{5} should always parse to a usize")
        })
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

    #[test]
    fn start() {
        let config = Config {
            path: "data/images".into(),
            start: Some(3522),
            end: None,
        };
        assert_eq!(6, config.paths().unwrap().len());
    }

    #[test]
    fn end() {
        let config = Config {
            path: "data/images".into(),
            start: None,
            end: Some(3522),
        };
        assert_eq!(2, config.paths().unwrap().len());
    }

    #[test]
    fn start_out_of_range() {
        let config = Config {
            path: "data/images".into(),
            start: Some(3520),
            end: None,
        };
        assert!(config.paths().is_err());
    }

    #[test]
    fn end_out_of_range() {
        let config = Config {
            path: "data/images".into(),
            start: None,
            end: Some(3428),
        };
        assert!(config.paths().is_err());
    }
}
