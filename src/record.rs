//! Records are internal RiPROCESS objects.
//!
//! We sometimes need to extract/use values from records. Maybe someday we'll populate this
//! information from the RiPROCESS XML itself, but for now we have to manually transcribe values.

use Result;

/// Confguration for records.
#[derive(Debug, Default, Deserialize)]
pub struct Config {
    /// The start time for each record.
    pub start_times: Vec<f64>,
}

impl Config {
    /// Adjust an array of timestamps, using the start times defined in this configurtion.
    ///
    /// Adjustment means:
    ///
    /// ```
    /// let record_start = 332978.669;
    /// let record_base = record_start - record_start % 100.;
    /// let first_timestamp_in_vector = 73779.899441;
    /// let timestamp_base = first_timestamp_in_vector - first_timestamp_in_vector % 100.;
    /// let timestamp = 73794.899136; // <- e.g.
    /// let adjusted_timestamp = timestamp - timestamp_base + record_base;
    /// ```
    ///
    /// Returns a flattened vector of all the adjusted timestamps.
    ///
    /// Returns an error if:
    ///
    /// - Any of the timestamp vectors are empty.
    /// - There is a mismatch between the size of the start times vector in this config and the
    ///   timestamp vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use riprocess::record::Config;
    /// let config = Config { start_times: vec![332978.669] };
    /// let timestamps = config.adjust_timestamps(&vec![vec![73779.899441]]).unwrap();
    /// assert_eq!(vec![332979.899441], timestamps);
    /// ```
    pub fn adjust_timestamps(&self, timestamps: &Vec<Vec<f64>>) -> Result<Vec<f64>> {
        use Error;

        if self.start_times.len() != timestamps.len() {
            return Err(Error::RecordCountMismatch {
                           timestamps: timestamps.len(),
                           records: self.start_times.len(),
                       });
        }
        let timestamp_bases = timestamps.iter()
            .map(|timestamps| if timestamps.is_empty() {
                     Err(Error::NoTimestamps)
                 } else {
                     Ok(timestamps[0] - timestamps[0] % 100.)
                 })
            .collect::<Result<Vec<_>>>()?;
        Ok(self.start_times
               .iter()
               .zip(timestamp_bases.into_iter())
               .zip(timestamps.iter())
               .flat_map(|((start_time, timestamp_base), timestamps)| {
                             timestamps.iter().map(move |timestamp| {
                                                       timestamp - timestamp_base + start_time -
                                                       start_time % 100.
                                                   })
                         })
               .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_reference() {
        let timestamps = vec![vec![73779.899441, 73781.419326], vec![73840.399224, 73842.018970]];
        let start_times = vec![332978.669, 333039.279];
        let config = Config { start_times: start_times };
        let timestamps = config.adjust_timestamps(&timestamps).unwrap();
        assert_eq!(4, timestamps.len());
        let expected = vec![332979.899441, 332981.419326, 333040.399224, 333042.018970];
        assert!(timestamps.iter().zip(expected.iter()).all(|(a, b)| (a - b).abs() < 1e-7),
                "actual={:?}, expected={:?}",
                timestamps,
                expected);
    }

    #[test]
    fn empty_timestamps() {
        let config = Config { start_times: vec![1.] };
        let timestamps = vec![vec![]];
        assert!(config.adjust_timestamps(&timestamps).is_err());
    }

    #[test]
    fn count_mismatch() {
        let config = Config { start_times: vec![1., 2.] };
        let timestamps = vec![vec![1.]];
        assert!(config.adjust_timestamps(&timestamps).is_err());
    }
}
