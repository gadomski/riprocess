use std::path::PathBuf;

/// Configuration for timestamps.
#[derive(Debug, Default, Deserialize)]
pub struct Config {
    /// The directory that holds the timestamp files.
    pub path: PathBuf,
    /// The name of the first timestamp file to be used.
    ///
    /// If None, uses the first file in the directory.
    pub first: Option<String>,
    /// The name of the last timestamp file to be used.
    ///
    /// If None, uses the last file in the directory.
    pub last: Option<String>,
}
