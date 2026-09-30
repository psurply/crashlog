// Copyright (C) 2025 Intel Corporation
// SPDX-License-Identifier: MIT

//! Provides access to the records stored in a Crash Log region.

use crate::cper::section::{CperSectionBody, fer};
use crate::error::Error;
use crate::header::{Header, record_types};
use crate::record::Record;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

const MAX_NESTED_BOXES: usize = 4;

/// A container for one or several Crash Log records.
///
/// A Crash Log region refers to a region in an on-die memory that is allocated for storing
/// a contiguous sequence of Crash Log records.
#[derive(Default)]
pub struct Region {
    pub records: Vec<Record>,
    depth: usize,
}

impl Region {
    pub(crate) fn from_cper_section(section: &CperSectionBody) -> Option<Self> {
        match section {
            CperSectionBody::FirmwareErrorRecord(fer) => {
                let guid = fer.header.guid;
                if guid == fer::guids::RECORD_ID_CRASHLOG {
                    Region::from_slice(&fer.payload).ok()
                } else {
                    log::info!("Ignoring unknown Firmware Error Record: {}", guid);
                    None
                }
            }
            _ => None,
        }
    }

    pub(crate) fn get_children(&self) -> Vec<Self> {
        let mut children = Vec::new();

        if self.depth >= MAX_NESTED_BOXES {
            return children;
        }

        for record in self.records.iter() {
            let errata = record.header.version.errata();
            let is_box = record.header.version.record_type == record_types::BOX
                || errata.type0_legacy_server_box;

            if !is_box {
                continue;
            }

            let Some(payload) = record.data.get(record.header.header_size()..) else {
                log::error!("The Box record has an empty payload");
                continue;
            };

            match Region::from_slice(payload) {
                Ok(mut child) => {
                    for child_record in child.records.iter_mut() {
                        child_record.context.parent_header = Some(record.header.clone());
                    }

                    child.depth = self.depth + 1;

                    children.push(child)
                }
                Err(err) => log::warn!("Invalid region in Box record: {err}"),
            }
        }

        children
    }

    pub fn from_slice(bytes: &[u8]) -> Result<Self, Error> {
        let mut region = Region::default();
        let mut cursor = 0;

        while cursor < bytes.len() {
            let header = match Header::from_slice(&bytes[cursor..]) {
                Ok(Some(header)) => header,
                Ok(None) => {
                    log::debug!("Found termination marker at offset {cursor}");
                    break;
                }
                Err(err) => {
                    log::warn!("Cannot decode record header: {err}");
                    if region.records.is_empty() {
                        // Return the error if no record can be decoded
                        return Err(err);
                    }
                    break;
                }
            };

            log::debug!("Record version: {0}", header);

            let record_size = header.record_size();
            log::debug!("Record size: 0x{record_size:04x}");

            if record_size == 0 {
                log::warn!(
                    "{} record has an empty size. Skipping.",
                    header.record_type().unwrap_or("UNKNOWN")
                );
                break;
            }

            let limit = cursor + record_size;
            if limit > bytes.len() {
                log::warn!(
                    "Truncated record detected: record is expected to be {}B but is {}B",
                    record_size,
                    bytes.len() - cursor
                )
            }

            region.records.push(Record {
                header,
                data: bytes[cursor..limit.min(bytes.len())].into(),
                ..Default::default()
            });

            cursor += record_size;
        }

        if region.records.is_empty() {
            return Err(Error::EmptyRegion);
        }

        Ok(region)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        for record in self.records.iter() {
            bytes.append(&mut record.data.clone());
        }
        bytes
    }
}
