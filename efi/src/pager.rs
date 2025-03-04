// Copyright (C) 2025 Intel Corporation
// SPDX-License-Identifier: MIT

mod input;
mod keys;
mod render;
mod update;
mod window;

use alloc::string::String;
use alloc::vec::Vec;
use uefi::boot::ScopedProtocol;
use uefi::proto::console::text::{Input, Output};
use window::Window;

#[derive(Default)]
enum PagerMode {
    #[default]
    View,
    Search,
}

pub struct Pager {
    lines: Vec<String>,
    window: Window,
    mode: PagerMode,
    output: ScopedProtocol<Output>,
    input: ScopedProtocol<Input>,
    search_pattern: String,
}

impl Pager {
    fn new(text: &str) -> Result<Self, uefi::Error> {
        let handle = uefi::boot::get_handle_for_protocol::<Output>()?;
        let output = uefi::boot::open_protocol_exclusive::<Output>(handle)
            .inspect_err(|err| log::warn!("Failed to open Output protocol: {err}"))?;

        let handle = uefi::boot::get_handle_for_protocol::<Input>()?;
        let input = uefi::boot::open_protocol_exclusive::<Input>(handle)
            .inspect_err(|err| log::warn!("Failed to open Input protocol: {err}"))?;

        Ok(Self {
            lines: text.lines().map(String::from).collect(),
            window: output
                .current_mode()?
                .as_ref()
                .map(Window::new)
                .unwrap_or_else(Window::default),
            output,
            input,
            mode: PagerMode::default(),
            search_pattern: String::new(),
        })
    }

    pub fn display(text: &str) -> Result<(), uefi::Error> {
        let mut pager = Pager::new(text)?;
        pager.run()
    }
}
