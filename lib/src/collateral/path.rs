// Copyright (C) 2025 Intel Corporation
// SPDX-License-Identifier: MIT

#[cfg(not(feature = "std"))]
use alloc::{fmt, str::FromStr, string::String, vec::Vec};
#[cfg(feature = "std")]
use std::{fmt, path::PathBuf, str::FromStr};

/// A path within a [`crate::collateral::PVSS`] to an item stored in the collateral tree.
#[derive(Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct ItemPath(Vec<String>);

impl ItemPath {
    pub fn new<const N: usize>(path: [&str; N]) -> Self {
        ItemPath(path.into_iter().map(String::from).collect())
    }

    pub(crate) fn push(&mut self, element: &str) {
        self.0.push(element.into())
    }
}

#[cfg(feature = "std")]
impl From<&ItemPath> for PathBuf {
    fn from(path: &ItemPath) -> Self {
        path.0.iter().collect()
    }
}

impl FromStr for ItemPath {
    type Err = ();

    fn from_str(path: &str) -> Result<Self, Self::Err> {
        Ok(Self(path.split('/').map(String::from).collect()))
    }
}

impl From<&str> for ItemPath {
    fn from(path: &str) -> Self {
        path.parse().unwrap()
    }
}

impl fmt::Display for ItemPath {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, path) in self.0.iter().enumerate() {
            write!(f, "{path}")?;
            if i < self.0.len() - 1 {
                write!(f, "/")?;
            }
        }
        Ok(())
    }
}
