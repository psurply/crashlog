// Copyright (C) 2025 Intel Corporation
// SPDX-License-Identifier: MIT

#![allow(dead_code)]

pub mod fer;
#[cfg(test)]
mod tests;

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
use fer::FirmwareErrorRecord;
use uguid::Guid;

pub const FW_ERROR_RECORD_GUID: Guid = uguid::guid!("81212a96-09ed-4996-9471-8d729c8e69ed");
const RECORD_HEADER_SIZE: usize = 128;
const SECTION_DESCRIPTOR_SIZE: usize = 72;

pub enum CperSection {
    FirmwareErrorRecord(FirmwareErrorRecord),
    Unknown(Vec<u8>),
}

impl CperSection {
    pub(super) fn from_slice(guid: uguid::Guid, s: &[u8]) -> Option<CperSection> {
        Some(match guid {
            FW_ERROR_RECORD_GUID => {
                CperSection::FirmwareErrorRecord(fer::FirmwareErrorRecord::from_slice(s)?)
            }
            _ => CperSection::Unknown(Vec::from(s)),
        })
    }

    pub(super) fn to_bytes(&self) -> Vec<u8> {
        match self {
            CperSection::FirmwareErrorRecord(fer) => fer.to_bytes(),
            CperSection::Unknown(data) => data.clone(),
        }
    }
}

pub struct Revision {
    pub minor: u8,
    pub major: u8,
}

pub struct CperSectionDescriptor {
    pub section_offset: u32,
    pub section_length: u32,
    pub revision: Revision,
    pub validation_bits: u8,
    pub reserved: u8,
    pub flags: u32,
    pub section_type: Guid,
    pub fru_id: Guid,
    pub section_severity: u32,
    pub fru_text: [u8; 20],
}

impl CperSectionDescriptor {
    fn from_slice(s: &[u8]) -> Option<Self> {
        Some(CperSectionDescriptor {
            section_offset: u32::from_le_bytes(s.get(0..4)?.try_into().ok()?),
            section_length: u32::from_le_bytes(s.get(4..8)?.try_into().ok()?),
            revision: Revision {
                minor: *s.get(8)?,
                major: *s.get(9)?,
            },
            validation_bits: *s.get(10)?,
            reserved: *s.get(11)?,
            flags: u32::from_le_bytes(s.get(12..16)?.try_into().ok()?),
            section_type: Guid::from_bytes(s.get(16..32)?.try_into().ok()?),
            fru_id: Guid::from_bytes(s.get(32..48)?.try_into().ok()?),
            section_severity: u32::from_le_bytes(s.get(48..52)?.try_into().ok()?),
            fru_text: s.get(52..72)?.try_into().ok()?,
        })
    }
}

pub struct Section {
    pub descriptor: CperSectionDescriptor,
    pub section: CperSection,
}

pub struct CperHeader {
    pub signature_start: u32,
    pub revision: Revision,
    pub signature_end: u32,
    pub section_count: u16,
    pub error_severity: u32,
    pub validation_bits: u32,
    pub record_length: u32,
    pub timestamp: u64,
    pub platform_id: Guid,
    pub partition_id: Guid,
    pub creator_id: Guid,
    pub notification_type: Guid,
    pub record_id: u64,
    pub flags: u32,
    pub persistence_information: u64,
    pub reserved: [u8; 12],
}

impl CperHeader {
    fn from_slice(s: &[u8]) -> Option<Self> {
        if s.starts_with(b"CPER") {
            Some(CperHeader {
                signature_start: u32::from_le_bytes(s.get(0..4)?.try_into().ok()?),
                revision: Revision {
                    minor: *s.get(4)?,
                    major: *s.get(5)?,
                },
                signature_end: u32::from_le_bytes(s.get(6..10)?.try_into().ok()?),
                section_count: u16::from_le_bytes(s.get(10..12)?.try_into().ok()?),
                error_severity: u32::from_le_bytes(s.get(12..16)?.try_into().ok()?),
                validation_bits: u32::from_le_bytes(s.get(16..20)?.try_into().ok()?),
                record_length: u32::from_le_bytes(s.get(20..24)?.try_into().ok()?),
                timestamp: u64::from_le_bytes(s.get(24..32)?.try_into().ok()?),
                platform_id: Guid::from_bytes(s.get(32..48)?.try_into().ok()?),
                partition_id: Guid::from_bytes(s.get(48..64)?.try_into().ok()?),
                creator_id: Guid::from_bytes(s.get(64..80)?.try_into().ok()?),
                notification_type: Guid::from_bytes(s.get(80..96)?.try_into().ok()?),
                record_id: u64::from_le_bytes(s.get(96..104)?.try_into().ok()?),
                flags: u32::from_le_bytes(s.get(104..108)?.try_into().ok()?),
                persistence_information: u64::from_le_bytes(s.get(108..116)?.try_into().ok()?),
                reserved: s.get(116..128)?.try_into().ok()?,
            })
        } else {
            None
        }
    }
}

pub struct Cper {
    /// CPER Record Header
    pub record_header: CperHeader,
    /// CPER Sections
    pub sections: Vec<Section>,
}

impl Cper {
    /// Decodes the CPER stored in a byte slice
    pub fn from_slice(slice: &[u8]) -> Option<Self> {
        let record_header = CperHeader::from_slice(slice.get(0..RECORD_HEADER_SIZE)?)?;

        let sections = (0..record_header.section_count)
            .map(|i| {
                let index = RECORD_HEADER_SIZE + (i as usize * SECTION_DESCRIPTOR_SIZE);
                let descriptor = CperSectionDescriptor::from_slice(slice.get(index..)?)?;
                let offset = descriptor.section_offset as usize;
                let end_offset = offset + descriptor.section_length as usize;
                let section = CperSection::from_slice(
                    descriptor.section_type,
                    slice.get(offset..end_offset)?,
                )?;
                Some(Section {
                    descriptor,
                    section,
                })
            })
            .collect::<Option<Vec<Section>>>()?;

        Some(Cper {
            record_header,
            sections,
        })
    }
}
