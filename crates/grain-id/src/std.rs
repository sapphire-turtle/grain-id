use std::time::{SystemTime, UNIX_EPOCH};

use super::GrainId;

impl GrainId {
    /// Creates a `GrainId` from a [`Duration`](std::time::Duration) from [`UNIX_EPOCH`] to `timestamp` based on decisecond precision.
    ///
    /// See [`GrainId::from_duration`] and [`GrainId::from_timestamp_since`] for more details.
    ///
    /// # Examples
    ///
    /// ```
    /// use chrono::Utc;
    /// # use grain_id::*;
    ///
    /// let timestamp = Utc::now();
    /// let id = GrainId::from_timestamp_unix(timestamp);
    /// assert_eq!(id.to_u64(), ((timestamp.timestamp_millis() / 100) as u64));
    /// ```
    pub fn from_timestamp_unix<T>(timestamp: T) -> Self
    where
        T: Into<SystemTime>,
    {
        Self::from_timestamp_since(timestamp, UNIX_EPOCH)
    }
    /// Creates a `GrainId` from a [`Duration`](std::time::Duration) from `base_time` to `timestamp` based on decisecond precision.
    ///
    /// See [`GrainId::from_duration`] for more details.
    ///
    /// # Behavior when `timestamp` is earlier than `base_time`
    ///
    /// If `timestamp` is earlier than `base_time`, Returns the value obtained by wrapping substracting the absolute value of the duration from [`NIL`](GrainId::NIL)
    ///
    /// # Examples
    ///
    /// ```
    /// use chrono::NaiveDate;
    /// # use grain_id::*;
    ///
    /// let base_time = NaiveDate::from_ymd_opt(2025, 1, 1)
    ///     .unwrap()
    ///     .and_hms_opt(0, 0, 0)
    ///     .unwrap()
    ///     .and_utc();
    /// let timestamp = NaiveDate::from_ymd_opt(2025, 11, 28)
    ///     .unwrap()
    ///     .and_hms_micro_opt(12, 34, 56, 789)
    ///     .unwrap()
    ///     .and_utc();
    /// let id = GrainId::from_timestamp_since(timestamp, base_time);
    /// assert_eq!(
    ///     id.to_u64(),
    ///     (((timestamp - base_time).num_milliseconds() / 100) as u64)
    /// );
    /// ```
    pub fn from_timestamp_since<T, U>(timestamp: T, base_time: U) -> Self
    where
        T: Into<SystemTime>,
        U: Into<SystemTime>,
    {
        match timestamp.into().duration_since(base_time.into()) {
            Ok(x) => Self::from_duration(x),
            Err(x) => Self::NIL.wrapping_sub(Self::from_duration(x.duration())),
        }
    }

    /// Creates a `GrainId` from a [`Duration`](std::time::Duration) from [`UNIX_EPOCH`] to [`SystemTime::now()`] based on decisecond precision.
    ///
    /// See [`GrainId::from_duration`] and [`GrainId::from_timestamp_since`] for more details.
    pub fn now_unix() -> Self {
        Self::from_timestamp_unix(SystemTime::now())
    }

    /// Creates a `GrainId` from a [`Duration`](std::time::Duration) from `base_time` to [`SystemTime::now()`] based on decisecond precision.
    ///
    /// See [`GrainId::from_duration`] and [`GrainId::from_timestamp_since`] for more details.
    pub fn now_since<T>(base_time: T) -> Self
    where
        T: Into<SystemTime>,
    {
        Self::from_timestamp_since(SystemTime::now(), base_time)
    }
}
