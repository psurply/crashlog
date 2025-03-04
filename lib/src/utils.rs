// Copyright (C) 2025 Intel Corporation
// SPDX-License-Identifier: MIT

#[cfg(not(feature = "std"))]
use alloc::collections::BTreeMap;
#[cfg(feature = "std")]
use std::collections::HashMap;

#[cfg(feature = "std")]
pub type Map<K, T> = HashMap<K, T>;
#[cfg(not(feature = "std"))]
pub type Map<K, T> = BTreeMap<K, T>;
