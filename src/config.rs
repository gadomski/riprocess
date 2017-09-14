use {Error, Result, image, record, timestamp};
use std::iter::Zip;
use std::path::{Path, PathBuf};
use std::vec::IntoIter;

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
    pub images: image::Config,
    /// Timestamp configuration.
    pub timestamps: timestamp::Config,
    /// Record configuration.
    pub records: record::Config,
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
    #[ignore]
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
    #[ignore]
    fn record_count_mismatch() {
        let mut config = Config::from_path("data/config.toml").unwrap();
        config.records.start_times = vec![1.];
        assert!(config.image_list().is_err());
    }

    #[test]
    #[ignore]
    fn image_count_mismatch() {
        let mut config = Config::from_path("data/config.toml").unwrap();
        config.images.last = None;
        assert!(config.image_list().is_err());
    }

    #[test]
    #[ignore]
    fn no_images() {
        let mut config = Config::from_path("data/config.toml").unwrap();
        config.images.path = "data".into();
        config.images.first = None;
        config.images.last = None;
        assert!(config.image_list().is_err());
    }
}
