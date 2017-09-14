//! Records are internal RiPROCESS objects.
//!
//! We sometimes need to extract/use values from records. Maybe someday we'll populate this
//! information from the RiPROCESS XML itself, but for now we have to manually transcribe values.

/// Confguration for records.
#[derive(Debug, Default, Deserialize)]
pub struct Config {
    /// The start time for each record.
    pub start_times: Vec<f64>,
}
