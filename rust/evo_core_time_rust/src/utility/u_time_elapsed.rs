use crate::UTime;

/// A utility for measuring elapsed time in various units.
///
/// This struct provides methods to calculate elapsed time
/// from a given starting point measured in nanoseconds.
pub struct UTimeElapsed;

impl UTimeElapsed {
    /// Returns the elapsed time in nanoseconds.
    ///
    /// # Parameters
    /// - `time_start_ns`: The starting time in nanoseconds
    ///
    /// # Returns
    /// The elapsed time in nanoseconds as a u64.
    /// Uses saturating subtraction to prevent overflow.
    pub fn ns(time_start_ns: u64) -> u64 {
        UTime::time_ns().saturating_sub(time_start_ns)
    }

    /// Returns the elapsed time in milliseconds.
    ///
    /// # Parameters
    /// - `time_start_ns`: The starting time in nanoseconds
    ///
    /// # Returns
    /// The elapsed time in milliseconds as a floating-point value.
    /// Uses saturating subtraction to prevent overflow.
    pub fn ms(time_start_ns: u64) -> f64 {
        UTime::time_ns().saturating_sub(time_start_ns) as f64 / 1e6
    }

    /// Returns the elapsed time in seconds.
    ///
    /// # Parameters
    /// - `time_start_ns`: The starting time in nanoseconds
    ///
    /// # Returns
    /// The elapsed time in seconds as a floating-point value.
    /// Uses saturating subtraction to prevent overflow.
    pub fn s(time_start_ns: u64) -> f64 {
        UTime::time_ns().saturating_sub(time_start_ns) as f64 / 1e9
    }
}