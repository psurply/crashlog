// Copyright (C) 2026 Intel Corporation
// SPDX-License-Identifier: MIT

#[cfg(not(feature = "std"))]
use alloc::fmt;
#[cfg(feature = "std")]
use std::fmt;

#[derive(Debug, Eq, PartialEq)]
pub enum MachineCheckErrorCode {
    Raw(u16),
    NoError,
    Unclassified,
    MicrocodeRomParityError,
    ExternalError,
    FrcError,
    InternalParityError,
    SmmHandlerCodeAccessViolation,
    InternalTimerError,
    IoError,
    InternalUnclassified(u16),
}

impl fmt::Display for MachineCheckErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Raw(mcacod) => write!(f, "MCACOD_{mcacod:04X}H"),
            Self::NoError => write!(f, "NO_ERROR"),
            Self::Unclassified => write!(f, "UNCLASSIFIED"),
            Self::MicrocodeRomParityError => write!(f, "MICROCODE_ROM_PARITY_ERROR"),
            Self::ExternalError => write!(f, "EXTERNAL_ERROR"),
            Self::FrcError => write!(f, "FRC_ERROR"),
            Self::InternalParityError => write!(f, "INTERNAL_PARITY_ERROR"),
            Self::SmmHandlerCodeAccessViolation => write!(f, "SMM_HANDLER_CODE_ACCESS_VIOLATION"),
            Self::InternalTimerError => write!(f, "INTERNAL_TIMER_ERROR"),
            Self::IoError => write!(f, "IO_ERROR"),
            Self::InternalUnclassified(x) => write!(f, "INTERNAL_UNCLASSIFIED_{x:04X}H"),
        }
    }
}

impl MachineCheckErrorCode {
    pub fn from_u16(code: u16) -> Self {
        match code {
            0b0000_0000_0000_0000 => Self::NoError,
            0b0000_0000_0000_0001 => Self::Unclassified,
            0b0000_0000_0000_0010 => Self::MicrocodeRomParityError,
            0b0000_0000_0000_0011 => Self::ExternalError,
            0b0000_0000_0000_0100 => Self::FrcError,
            0b0000_0000_0000_0101 => Self::InternalParityError,
            0b0000_0000_0000_0110 => Self::SmmHandlerCodeAccessViolation,
            0b0000_0100_0000_0000 => Self::InternalTimerError,
            0b0000_1110_0000_1011 => Self::IoError,
            code if code >> 10 == 1 => Self::InternalUnclassified(code & 0x3FF),
            _ => Self::Raw(code),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_error_codes() {
        let samples = [
            (0b0000_0000_0000_0000, MachineCheckErrorCode::NoError),
            (0b0000_0000_0000_0001, MachineCheckErrorCode::Unclassified),
            (
                0b0000_0000_0000_0010,
                MachineCheckErrorCode::MicrocodeRomParityError,
            ),
            (0b0000_0000_0000_0011, MachineCheckErrorCode::ExternalError),
            (0b0000_0000_0000_0100, MachineCheckErrorCode::FrcError),
            (
                0b0000_0000_0000_0101,
                MachineCheckErrorCode::InternalParityError,
            ),
            (
                0b0000_0000_0000_0110,
                MachineCheckErrorCode::SmmHandlerCodeAccessViolation,
            ),
            (
                0b0000_0100_0000_0000,
                MachineCheckErrorCode::InternalTimerError,
            ),
            (0b0000_1110_0000_1011, MachineCheckErrorCode::IoError),
            (
                0b0000_0100_0010_1010,
                MachineCheckErrorCode::InternalUnclassified(42),
            ),
        ];

        for (value, error) in samples {
            assert_eq!(MachineCheckErrorCode::from_u16(value), error);
        }
    }
}
