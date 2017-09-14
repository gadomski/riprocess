/// Confguration for records.
#[derive(Debug, Default, Deserialize)]
pub struct Config {
    /// The start time for each record.
    pub start_times: Vec<f64>,
}
