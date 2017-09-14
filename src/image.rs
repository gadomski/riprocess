use std::path::PathBuf;

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
