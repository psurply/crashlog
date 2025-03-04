// Copyright (C) 2025 Intel Corporation
// SPDX-License-Identifier: MIT

#[cfg(target_os = "uefi")]
pub mod efi;
#[cfg(all(target_family = "windows", feature = "std"))]
pub mod event_log;
#[cfg(all(target_os = "linux", feature = "std"))]
pub mod sysfs;
