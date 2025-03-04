// Copyright (C) 2025 Intel Corporation
// SPDX-License-Identifier: MIT

use crate::bert::Berr;
use crate::error::Error;

const BERR_PATH: &str = "/sys/firmware/acpi/tables/data/BERT";

pub(crate) fn read_berr_from_sysfs() -> Result<Berr, Error> {
    std::fs::read(BERR_PATH)
        .map_err(|err| match err.kind() {
            std::io::ErrorKind::NotFound => Error::NoCrashLogFound,
            _ => Error::from(err),
        })
        .and_then(|berr| Berr::from_slice(&berr).ok_or(Error::InvalidBootErrorRecordRegion))
}
