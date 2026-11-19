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
    /// use jiff::Timestamp;
    /// use std::time::SystemTime;
    /// # use grain_id::*;
    ///
    /// let timestamp = Timestamp::now();
    /// let id = GrainId::from_timestamp_unix(SystemTime::try_from(timestamp).unwrap());
    /// assert_eq!(id.to_u64(), ((timestamp.as_millisecond() / 100) as u64));
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
    /// use jiff::Timestamp;
    /// use std::time::SystemTime;
    /// # use grain_id::*;
    ///
    /// let base_time: Timestamp = "2025-01-01T00:00:00Z".parse().unwrap();
    /// let timestamp: Timestamp = "2025-11-28T12:34:56.000789Z".parse().unwrap();
    /// let id = GrainId::from_timestamp_since(
    ///     SystemTime::try_from(timestamp).unwrap(),
    ///     SystemTime::try_from(base_time).unwrap(),
    /// );
    /// assert_eq!(
    ///     id.to_u64(),
    ///     ((timestamp.duration_since(base_time).as_millis() / 100) as u64)
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
