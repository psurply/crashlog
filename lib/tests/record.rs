// Copyright (C) 2025 Intel Corporation
// SPDX-License-Identifier: MIT

use intel_crashlog::prelude::*;
use std::fs;
use std::path::Path;

const COLLATERAL_TREE_PATH: &str = "tests/collateral";

#[test]
fn basic_decode() {
    let record = Record {
        header: Header::default(),
        data: vec![
            0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8A, 0x8B, 0x8C, 0x8D,
            0x8E, 0x8F,
        ],
    };

    let csv = "name;offset;size;description;bitfield
foo;0;128;;0
foo.bar.baz;4;64;;0
foo.bar;4;8;;0";

    let root = record.decode_with_csv(csv.as_bytes(), 0).unwrap();
    let section = root.get_by_path("foo").unwrap();
    assert_eq!(section.kind, NodeType::Section);
    let field = root.get_by_path("foo.bar").unwrap();
    assert_eq!(field.kind, NodeType::Field { value: 0x18 });
    let field = root.get_by_path("foo.bar.baz").unwrap();
    assert_eq!(
        field.kind,
        NodeType::Field {
            value: 0x8878685848382818
        }
    );
}

#[test]
fn decode() {
    let mut cm = CollateralManager::file_system_tree(Path::new(COLLATERAL_TREE_PATH)).unwrap();

    let data = fs::read("tests/samples/dummy_mca_rev1.crashlog").unwrap();
    let header = Header::from_slice(&data).unwrap().unwrap();
    let record = Record { header, data };

    let root = record.decode(&mut cm).unwrap();
    let version = root.get_by_path("mca.hdr.version.revision").unwrap();
    assert_eq!(version.kind, NodeType::Field { value: 1 });
}

#[test]
fn header_type6_decode() {
    let mut cm = CollateralManager::file_system_tree(Path::new(COLLATERAL_TREE_PATH)).unwrap();

    let data = fs::read("tests/samples/dummy_mca_rev2.crashlog").unwrap();
    let header = Header::from_slice(&data).unwrap().unwrap();
    let record = Record { header, data };

    let root = record.decode(&mut cm).unwrap();
    let version = root
        .get_by_path("processors.cpu0.die1.mca.hdr.version.revision")
        .unwrap();

    assert_eq!(version.kind, NodeType::Field { value: 2 });
}

#[test]
fn header_checksum() {
    let data = fs::read("tests/samples/three_strike_timeout.crashlog").unwrap();
    let region = Region::from_slice(&data).unwrap();

    for record in region.records.iter() {
        assert!(record.checksum().unwrap())
    }
}
