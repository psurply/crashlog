// Copyright (C) 2025 Intel Corporation
// SPDX-License-Identifier: MIT

//! Information extracted alongside the Crash Log records.

#[cfg(not(feature = "std"))]
use alloc::{fmt, string::String};
#[cfg(feature = "std")]
use std::fmt;

/// Crash Log Metadata
#[derive(Default)]
pub struct Metadata {
    pub computer: Option<String>,
    pub time: Option<Time>,
}

/// Crash Log Extraction Time
pub struct Time {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
}

impl fmt::Display for Metadata {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match (self.computer.as_ref(), self.time.as_ref()) {
            (Some(computer), Some(time)) => write!(f, "{computer}-{time}"),
            (None, Some(time)) => write!(f, "{time}"),
            (Some(computer), None) => write!(f, "{computer}"),
            (None, None) => write!(f, "unnamed"),
        }
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:04}-{:02}-{:02}-{:02}-{:02}",
            self.year, self.month, self.day, self.hour, self.minute
        )
    }
}
