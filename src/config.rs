use std::iter::Zip;
use std::path::{PathBuf, Path};
use std::vec::IntoIter;
use {Result, Error};

/// Configuration for a RiPROCESS setup.
///
/// # Examples
///
/// ```
/// # use riprocess::Config;
/// let config = Config::from_path("data/config.toml").unwrap();
/// ```
#[derive(Debug, Default, Deserialize)]
pub struct Config {
    /// Image file configuration.
    pub images: ImageConfig,
    /// Timestamp configuration.
    pub timestamps: TimestampConfig,
    /// Record configuration.
    pub records: RecordConfig,
}

/// Configuration for a set of images.
#[derive(Debug, Default, Deserialize)]
pub struct ImageConfig {
    /// The directory that holds the images.
    pub path: PathBuf,
    /// The number of the first image to be used.
    ///
    /// If None, the first image in the directory is used.
    pub first_image_number: Option<usize>,
    /// The number of the last image to be used.
    ///
    /// If none, the last image in the directory is used.
    pub last_image_number: Option<usize>,
}

/// Confguration for timestamps.
#[derive(Debug, Default, Deserialize)]
pub struct TimestampConfig {
    /// The directory that holds the timestamp files.
    pub path: PathBuf,
    /// The name of the first timestamp file to be used.
    ///
    /// If None, uses the first file in the directory.
    pub first_timestamp_file_name: Option<String>,
    /// The name of the last timestamp file to be used.
    ///
    /// If None, uses the last file in the directory.
    pub last_timestamp_file_name: Option<String>,
}

/// Confguration for records.
#[derive(Debug, Default, Deserialize)]
pub struct RecordConfig {
    /// The start time for each record.
    pub start_times: Vec<f64>,
}

/// An iterator over timestamps and images.
#[derive(Debug)]
pub struct ImageList {
    iter: Zip<IntoIter<PathBuf>, IntoIter<f64>>,
}

/// An image record.
#[derive(Debug, PartialEq)]
pub struct Image {
    /// The path to the image.
    pub path: PathBuf,
    /// The timestamp of the image.
    pub timestamp: f64,
}

impl Config {
    /// Creates a configuration from a TOML file at the provided path.
    ///
    /// # Examples
    ///
    /// ```
    /// # use riprocess::Config;
    /// let config = Config::from_path("data/config.toml").unwrap();
    /// ```
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Config> {
        use std::fs::File;
        use std::io::Read;
        use toml;
        let mut contents = String::new();
        let mut file = File::open(path)?;
        file.read_to_string(&mut contents)?;
        toml::from_str(&contents).map_err(Error::from)
    }

    /// Creates a new, default configuration.
    ///
    /// # Examples
    ///
    /// ```
    /// use riprocess::Config;
    /// let config = Config::new();
    /// ```
    pub fn new() -> Config {
        Default::default()
    }

    /// Returns a vector of all image paths for this configuration.
    ///
    /// # Examples
    ///
    /// ```
    /// # use riprocess::Config;
    /// let mut config = Config::new();
    /// config.images.path = "data/images".into();
    /// let image_paths = config.image_paths().unwrap();
    /// ```
    pub fn image_paths(&self) -> Result<Vec<PathBuf>> {
        use regex::Regex;
        use std::fs::DirEntry;
        use std::io;

        lazy_static! {
            static ref RE: Regex = Regex::new(r"^DSC(?P<image_number>\d{5}).JPG").unwrap();
        }

        let select_paths = |result: io::Result<DirEntry>| match result {
            Ok(dir_entry) => {
                if let Some(captures) = RE.captures(&dir_entry.file_name().to_string_lossy()) {
                    match captures.name("image_number")
                              .unwrap()
                              .as_str()
                              .parse::<usize>() {
                        Ok(image_number) => {
                            if self.images
                                   .first_image_number
                                   .map(|n| n <= image_number)
                                   .unwrap_or(true) &&
                               self.images
                                   .last_image_number
                                   .map(|n| n >= image_number)
                                   .unwrap_or(true) {
                                Some(Ok(dir_entry.path()))
                            } else {
                                None
                            }
                        }
                        Err(err) => Some(Err(Error::from(err))),
                    }
                } else {
                    None
                }
            }
            Err(err) => Some(Err(Error::from(err))),
        };

        self.images
            .path
            .read_dir()
            .map_err(Error::from)
            .and_then(|read_dir| read_dir.filter_map(select_paths).collect())
    }

    /// Returns a vector of all paths to timestamp (.eif) files, as configured.
    ///
    /// # Examples
    ///
    /// ```
    /// use riprocess::Config;
    /// let config = Config::from_path("data/config.toml").unwrap();
    /// let timestamp_paths = config.timestamp_paths().unwrap();
    /// ```
    pub fn timestamp_paths(&self) -> Result<Vec<PathBuf>> {
        Ok(Vec::new())
    }

    /// Returns an iterator over timestamp+path pairs for each configued image.
    ///
    /// Errors occur when the number of timestamp files doesn't match the number of records or the
    /// number of images doesn't match the number of timestamps.
    ///
    /// # Examples
    ///
    /// ```
    /// # use riprocess::Config;
    /// let config = Config::from_path("data/config.toml").unwrap();
    /// let image_list = config.image_list().unwrap().collect::<Vec<_>>();
    /// ```
    pub fn image_list(&self) -> Result<ImageList> {
        unimplemented!()
    }
}

impl Iterator for ImageList {
    type Item = Image;
    fn next(&mut self) -> Option<Image> {
        self.iter.next().map(|(path, timestamp)| {
                                 Image {
                                     path: path,
                                     timestamp: timestamp,
                                 }
                             })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn image_list() {
        let config = Config::from_path("data/config.toml").unwrap();
        let images = config.image_list().unwrap().collect::<Vec<_>>();

        let file_names =
            images.iter().map(|image| image.path.file_name().unwrap()).collect::<Vec<_>>();
        assert_eq!(vec!["DSC03522.JPG", "DSC03523.JPG", "DSC03524.JPG", "DSC03525.JPG"],
                   file_names);

        let expected_timestamps = vec![332979.899441, 332981.419326, 333040.399224, 333042.018970];
        let deltas = images.iter()
            .zip(expected_timestamps)
            .map(|(image, expected)| (expected - image.timestamp).abs())
            .collect::<Vec<_>>();
        assert!(deltas.iter().all(|&delta| delta < 1e-7), "{:?}", deltas);
    }

    #[test]
    fn record_count_mismatch() {
        let mut config = Config::from_path("data/config.toml").unwrap();
        config.records.start_times = vec![1.];
        assert!(config.image_list().is_err());
    }

    #[test]
    fn image_count_mismatch() {
        let mut config = Config::from_path("data/config.toml").unwrap();
        config.images.last_image_number = None;
        assert!(config.image_list().is_err());
    }

    #[test]
    fn no_images() {
        let mut config = Config::from_path("data/config.toml").unwrap();
        config.images.path = "data".into();
        config.images.first_image_number = None;
        config.images.last_image_number = None;
        assert!(config.image_list().is_err());
    }

    #[test]
    fn image_paths() {
        let mut config = Config::new();
        config.images.path = "data".into();
        assert!(config.image_paths().unwrap().is_empty());

        config.images.path = "data/images".into();
        assert_eq!(7, config.image_paths().unwrap().len());

        config.images.first_image_number = Some(3522);
        assert_eq!(6, config.image_paths().unwrap().len());

        config.images.last_image_number = Some(3526);
        assert_eq!(5, config.image_paths().unwrap().len());

        config.images.first_image_number = Some(42);
        config.images.last_image_number = None;
        assert!(config.image_paths().is_err());

        config.images.first_image_number = None;
        config.images.last_image_number = Some(42);
        assert!(config.image_paths().is_err());
    }

    #[test]
    #[ignore]
    fn timestamp_paths() {
        let mut config = Config::new();
        config.timestamps.path = "data".into();
        assert!(config.timestamp_paths().unwrap().is_empty());

        config.timestamps.path = "data/timestamps".into();
        assert_eq!(4, config.timestamp_paths().unwrap().len());

        config.timestamps.first_timestamp_file_name = Some("170621_202939.eif".to_string());
        assert_eq!(3, config.timestamp_paths().unwrap().len());

        config.timestamps.last_timestamp_file_name = Some("170621_203040.eif".to_string());
        assert_eq!(2, config.timestamp_paths().unwrap().len());

        config.timestamps.first_timestamp_file_name = Some("not a timestamp file".to_string());
        config.timestamps.last_timestamp_file_name = None;
        assert!(config.timestamp_paths().is_err());

        config.timestamps.first_timestamp_file_name = None;
        config.timestamps.last_timestamp_file_name = Some("not a timestamp file".to_string());
        assert!(config.timestamp_paths().is_err());
    }
}
