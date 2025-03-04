// Copyright (C) 2025 Intel Corporation
// SPDX-License-Identifier: MIT

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
#[cfg(not(feature = "std"))]
use core::mem;
#[cfg(feature = "std")]
use std::mem;
use uguid::Guid;

pub const RECORD_ID_CRASHLOG: Guid = uguid::guid!("8f87f311-c998-4d9e-a0c4-6065518c4f6d");

#[repr(C, packed)]
#[derive(Debug, Clone, Default)]
pub struct FirmwareErrorRecordHeader {
    pub error_type: u8,
    pub revision: u8,
    pub reserved: [u8; 6],
    pub record_identifier: u64,
    pub guid: Guid,
}

pub struct FirmwareErrorRecord {
    pub header: FirmwareErrorRecordHeader,
    pub payload: Vec<u8>,
}

impl FirmwareErrorRecordHeader {
    pub fn from_slice(s: &[u8]) -> Option<Self> {
        let revision = *s.get(1)?;
        Some(Self {
            error_type: *s.first()?,
            revision,
            reserved: s.get(2..8)?.try_into().ok()?,
            record_identifier: u64::from_le_bytes(s.get(8..16)?.try_into().ok()?),
            guid: if revision >= 2 {
                Guid::from_bytes(s.get(16..32)?.try_into().ok()?)
            } else {
                Guid::ZERO
            },
        })
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.push(self.error_type);
        bytes.push(self.revision);
        bytes.extend_from_slice(&self.reserved);
        bytes.extend_from_slice(&self.record_identifier.to_le_bytes());
        bytes.extend_from_slice(&self.guid.to_bytes());
        bytes
    }

    pub fn size(&self) -> usize {
        if self.revision >= 2 {
            mem::size_of::<Self>()
        } else {
            mem::size_of::<Self>() - mem::size_of::<u128>()
        }
    }
}

impl FirmwareErrorRecord {
    pub fn from_slice(s: &[u8]) -> Option<FirmwareErrorRecord> {
        let header = FirmwareErrorRecordHeader::from_slice(s)?;
        let payload = Vec::from(s.get(header.size()..)?);
        Some(Self { header, payload })
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = self.header.to_bytes();
        bytes.extend_from_slice(&self.payload);
        bytes
    }
}
