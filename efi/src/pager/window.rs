// Copyright (C) 2025 Intel Corporation
// SPDX-License-Identifier: MIT

use uefi::proto::console::text::OutputMode;

pub(super) struct Window {
    pub line: usize,
    pub columns: usize,
    pub rows: usize,
}

impl Default for Window {
    fn default() -> Self {
        Self {
            line: 0,
            columns: 80,
            rows: 25,
        }
    }
}

impl Window {
    pub fn new(mode: &OutputMode) -> Self {
        Self {
            columns: mode.columns(),
            rows: mode.rows(),
            ..Self::default()
        }
    }
}
