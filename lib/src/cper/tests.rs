// Copyright (C) 2025 Intel Corporation
// SPDX-License-Identifier: MIT

use super::Cper;
use crate::CrashLog;

pub const FW_ERROR_RECORD_GUID: uguid::Guid = uguid::guid!("81212a96-09ed-4996-9471-8d729c8e69ed");

#[test]
fn from_slice() {
    let cper = Cper::from_slice(&std::fs::read("tests/samples/cper.whea").unwrap()).unwrap();

    let signature = cper.record_header.signature_start.to_le_bytes();
    assert_eq!(&signature, b"CPER");

    assert_eq!(cper.record_header.section_count, 5);

    assert_eq!(cper.sections.len(), 5);

    for section in cper.sections.iter() {
        assert_eq!(section.descriptor.section_type, FW_ERROR_RECORD_GUID);
    }
}

#[test]
fn cl_from_cper() {
    let cper = Cper::from_slice(&std::fs::read("tests/samples/cper.whea").unwrap()).unwrap();
    let crashlog = CrashLog::from_cper(cper);
    assert!(crashlog.is_ok());
    let crashlog = crashlog.unwrap();

    assert_eq!(crashlog.regions.len(), 3);

    let mut records = Vec::new();

    for region in crashlog.regions.iter() {
        for record in region.records.iter() {
            records.push(record);
        }
    }

    assert_eq!(records.len(), 3);
}
