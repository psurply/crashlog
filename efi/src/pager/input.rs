// Copyright (C) 2025 Intel Corporation
// SPDX-License-Identifier: MIT

use super::keys;
use super::{Pager, PagerMode};
use uefi::proto::console::text::Key;
use uefi::proto::console::text::ScanCode;

pub enum Command {
    Down,
    Up,
    PageDown,
    PageUp,
    Top,
    Bottom,
    Quit,
    Refresh,
    Next,
    Previous,
}

impl Pager {
    fn search_mode(&mut self, key: Key) -> Option<Command> {
        match key {
            Key::Printable(k) => match char::from(k) {
                '\n' | '\r' => {
                    self.mode = PagerMode::View;
                    Some(Command::Next)
                }
                k => {
                    self.search_pattern.push(k);
                    Some(Command::Refresh)
                }
            },
            Key::Special(ScanCode::DELETE) => {
                let _ = self.search_pattern.pop();
                Some(Command::Refresh)
            }
            _ => None,
        }
    }

    fn view_mode(&mut self, key: Key) -> Option<Command> {
        Some(match key {
            Key::Printable(k) => match char::from(k) {
                keys::UP => Command::Up,
                '\n' | '\r' | keys::DOWN => Command::Down,
                keys::QUIT => Command::Quit,
                keys::PAGE_DOWN => Command::PageDown,
                keys::TOP => Command::Top,
                keys::BOTTOM => Command::Bottom,
                keys::NEXT => Command::Next,
                keys::PREV => Command::Previous,
                keys::SEARCH => {
                    self.search_pattern.clear();
                    self.mode = PagerMode::Search;
                    Command::Refresh
                }
                _ => return None,
            },
            Key::Special(code) => match code {
                ScanCode::UP => Command::Up,
                ScanCode::DOWN => Command::Down,
                ScanCode::HOME => Command::Top,
                ScanCode::END => Command::Bottom,
                ScanCode::PAGE_DOWN => Command::PageDown,
                ScanCode::PAGE_UP => Command::PageUp,
                ScanCode::ESCAPE => Command::Quit,
                _ => return None,
            },
        })
    }

    pub(super) fn next_command(&mut self) -> Result<Command, uefi::Error> {
        loop {
            let event = self
                .input
                .wait_for_key_event()
                .expect("Cannot get keyboard events");
            let _ = uefi::boot::wait_for_event(&mut [event]).expect("Cannot wait for key event");

            let Some(key) = self.input.read_key()? else {
                continue;
            };

            let cmd = match self.mode {
                PagerMode::View => self.view_mode(key),
                PagerMode::Search => self.search_mode(key),
            };

            if let Some(cmd) = cmd {
                return Ok(cmd);
            }
        }
    }
}
