//! Timestamps for camera images.
//!
//! Timestamps are contained in `.eif` files, usually residing in `04_CAM_RAW/01_EIF`.

use Result;
use regex::Regex;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

lazy_static! {
    static ref FILE_NAME_REGEX: Regex = Regex::new(r"^\d{6}_\d{6}.eif$").unwrap();
}

/// Configuration for timestamps.
#[derive(Debug, Default, Deserialize)]
pub struct Config {
    /// The directory that holds the timestamp files.
    pub path: PathBuf,
    /// The name of the first timestamp file to be used.
    ///
    /// If None, uses the first file in the directory.
    pub start: Option<String>,
    /// The name of the last timestamp file to be used.
    ///
    /// If None, uses the last file in the directory.
    pub end: Option<String>,
}

impl Config {
    /// Returns all timestamp file paths for this config.
    ///
    /// # Examples
    ///
    /// ```
    /// use riprocess::timestamp::Config;
    /// let config = Config { path: "data/timestamps".into(), ..Default::default() };
    /// let paths = config.paths().unwrap();
    /// ```
    pub fn paths(&self) -> Result<Vec<PathBuf>> {
        use Error;
        use std::fs::DirEntry;
        use std::io::Result;

        let mut file_names = Vec::new();
        let mut paths: Vec<PathBuf>;
        {
            let select_paths = |result: Result<DirEntry>| match result {
                Ok(dir_entry) => {
                    let file_name = dir_entry.file_name();
                    if file_name_is_match(&file_name) && self.file_name_is_in_range(&file_name) {
                        file_names.push(file_name.to_string_lossy().into_owned());
                        Some(dir_entry.path())
                    } else {
                        None
                    }
                }
                Err(_) => None,
            };
            paths = self.path
                .read_dir()?
                .filter_map(select_paths)
                .collect();
        }
        if let Some(start) = self.start.as_ref() {
            if !file_names.contains(start) {
                return Err(Error::InvalidTimestampFileName(start.clone()));
            }
        }
        if let Some(end) = self.end.as_ref() {
            if !file_names.contains(&end) {
                return Err(Error::InvalidTimestampFileName(end.clone()));
            }
        }
        paths.sort();
        Ok(paths)
    }

    /// Returns the timestamps in each configured timestamp file.
    ///
    /// # Examples
    ///
    /// ```
    /// use riprocess::timestamp::Config;
    /// let config = Config { path: "data/timestamps".into(), ..Default::default() };
    /// let timestamps = config.timestamps().unwrap();
    /// ```
    pub fn timestamps(&self) -> Result<Vec<Vec<f64>>> {
        self.paths().and_then(|paths| paths.into_iter().map(read_timestamps).collect())
    }

    fn file_name_is_in_range(&self, file_name: &OsStr) -> bool {
        file_name.to_str()
            .map(|file_name| {
                self.start
                    .as_ref()
                    .map(|start| start.as_str() <= file_name)
                    .unwrap_or(true) &&
                self.end
                    .as_ref()
                    .map(|end| end.as_str() >= file_name)
                    .unwrap_or(true)
            })
            .unwrap_or(false)
    }
}

fn file_name_is_match(file_name: &OsStr) -> bool {
    file_name.to_str().map(|file_name| FILE_NAME_REGEX.is_match(file_name)).unwrap_or(false)
}

fn read_timestamps<P: AsRef<Path>>(path: P) -> Result<Vec<f64>> {
    use Error;
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let file = File::open(path)?;
    BufReader::new(file)
        .lines()
        .map(|result| {
                 result.map_err(Error::from).and_then(|line| line.parse().map_err(Error::from))
             })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_paths() {
        let config = Config {
            path: "data".into(),
            start: None,
            end: None,
        };
        assert!(config.paths().unwrap().is_empty());
    }

    #[test]
    fn all_paths() {
        let config = Config {
            path: "data/timestamps".into(),
            start: None,
            end: None,
        };
        assert_eq!(4, config.paths().unwrap().len());
    }

    #[test]
    fn start() {
        let config = Config {
            path: "data/timestamps".into(),
            start: Some("170621_202939.eif".to_string()),
            end: None,
        };
        assert_eq!(3, config.paths().unwrap().len());
    }

    #[test]
    fn end() {
        let config = Config {
            path: "data/timestamps".into(),
            start: None,
            end: Some("170621_202939.eif".to_string()),
        };
        assert_eq!(2, config.paths().unwrap().len());
    }

    #[test]
    fn invalid_start() {
        let config = Config {
            path: "data/timestamps".into(),
            start: Some("not a timestamp file".to_string()),
            end: None,
        };
        assert!(config.paths().is_err());
    }

    #[test]
    fn invalid_end() {
        let config = Config {
            path: "data/timestamps".into(),
            start: None,
            end: Some("not a timestamp file".to_string()),
        };
        assert!(config.paths().is_err());
    }

    #[test]
    fn timestamps() {
        let config = Config {
            path: "data/timestamps".into(),
            start: None,
            end: Some("170621_202939.eif".to_string()),
        };
        let timestamps = config.timestamps().unwrap();
        assert_eq!(vec![vec![73779.899441, 73781.419326, 73782.899381],
                        vec![73779.899441, 73781.419326]],
                   timestamps);
    }
}
