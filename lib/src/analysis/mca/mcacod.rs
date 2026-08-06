// Copyright (C) 2026 Intel Corporation
// SPDX-License-Identifier: MIT

#[cfg(not(feature = "std"))]
use alloc::fmt;
#[cfg(feature = "std")]
use std::fmt;

/// SDM Vol. 3B 18.10.2.1
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CorrectionReportFiltering {
    Normal,
    Corrected,
}

impl CorrectionReportFiltering {
    pub fn from_u8(f: u8) -> Self {
        match f & 1 {
            1 => Self::Corrected,
            _ => Self::Normal,
        }
    }

    pub fn from_mcacod(mcacod: u16) -> Self {
        Self::from_u8(((mcacod >> 12) & 1) as u8)
    }
}

/// SDM Vol. 3B 18.10.2.3
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MemoryHierarchyLevel {
    Level0,
    Level1,
    Level2,
    Generic,
}

impl fmt::Display for MemoryHierarchyLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Level0 => write!(f, "L0"),
            Self::Level1 => write!(f, "L1"),
            Self::Level2 => write!(f, "L2"),
            Self::Generic => write!(f, "LG"),
        }
    }
}

impl MemoryHierarchyLevel {
    pub fn from_u8(ll: u8) -> Self {
        match ll & 0b11 {
            0b00 => Self::Level0,
            0b01 => Self::Level1,
            0b10 => Self::Level2,
            _ => Self::Generic,
        }
    }

    pub fn from_mcacod(mcacod: u16) -> Self {
        Self::from_u8((mcacod & 0b11) as u8)
    }
}

/// SDM Vol. 3B 18.10.2.2
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TransactionType {
    Instruction,
    Data,
    Generic,
    Reserved,
}

impl fmt::Display for TransactionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Instruction => write!(f, "I"),
            Self::Data => write!(f, "D"),
            Self::Generic => write!(f, "G"),
            Self::Reserved => write!(f, "X"),
        }
    }
}

impl TransactionType {
    pub fn from_u8(tt: u8) -> Self {
        match tt & 0b11 {
            0b00 => Self::Instruction,
            0b01 => Self::Data,
            0b10 => Self::Generic,
            _ => Self::Reserved,
        }
    }

    pub fn from_mcacod(mcacod: u16) -> Self {
        Self::from_u8(((mcacod >> 2) & 0b11) as u8)
    }
}

/// SDM Vol. 3B 18.10.2.4
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Request {
    Raw(u8),
    GenericError,
    GenericRead,
    GenericWrite,
    DataRead,
    DataWrite,
    InstructionFetch,
    Prefetch,
    Eviction,
    Snoop,
    PageWalk,
    EptPageWalk,
}

impl fmt::Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::GenericError => write!(f, "ERR"),
            Self::GenericRead => write!(f, "RD"),
            Self::GenericWrite => write!(f, "WR"),
            Self::DataRead => write!(f, "DRD"),
            Self::DataWrite => write!(f, "DWR"),
            Self::InstructionFetch => write!(f, "IRD"),
            Self::Prefetch => write!(f, "PREFETCH"),
            Self::Eviction => write!(f, "EVICT"),
            Self::Snoop => write!(f, "SNOOP"),
            Self::PageWalk => write!(f, "PW"),
            Self::EptPageWalk => write!(f, "EPW"),
            Self::Raw(rrrr) => write!(f, "{rrrr:X}H"),
        }
    }
}

impl Request {
    pub fn from_u8(rrrr: u8) -> Self {
        match rrrr & 0b1111 {
            0b0000 => Self::GenericError,
            0b0001 => Self::GenericRead,
            0b0010 => Self::GenericWrite,
            0b0011 => Self::DataRead,
            0b0100 => Self::DataWrite,
            0b0101 => Self::InstructionFetch,
            0b0110 => Self::Prefetch,
            0b0111 => Self::Eviction,
            0b1000 => Self::Snoop,
            0b1001 => Self::PageWalk,
            0b1010 => Self::EptPageWalk,
            rrrr => Self::Raw(rrrr),
        }
    }

    pub fn from_mcacod(mcacod: u16) -> Self {
        Self::from_u8(((mcacod >> 4) & 0b1111) as u8)
    }
}

/// SDM Vol. 3B 18.10.2.6
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Channel {
    Number(u8),
    NotSpecified,
}

impl fmt::Display for Channel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(cccc) => write!(f, "{cccc}"),
            Self::NotSpecified => write!(f, "X"),
        }
    }
}

impl Channel {
    pub fn from_u8(cccc: u8) -> Self {
        match cccc & 0b1111 {
            0b1111 => Self::NotSpecified,
            cccc => Self::Number(cccc),
        }
    }

    pub fn from_mcacod(mcacod: u16) -> Self {
        Self::from_u8((mcacod & 0b1111) as u8)
    }
}

/// SDM Vol. 3B 18.10.2.6
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MemoryControllerError {
    Raw(u8),
    GenericUndefinedRequest,
    MemoryReadError,
    MemoryWriteError,
    AddressCommandError,
    MemoryScrubbingError,
}

impl fmt::Display for MemoryControllerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::GenericUndefinedRequest => write!(f, "GEN"),
            Self::MemoryReadError => write!(f, "RD"),
            Self::MemoryWriteError => write!(f, "WR"),
            Self::AddressCommandError => write!(f, "AC"),
            Self::MemoryScrubbingError => write!(f, "MS"),
            Self::Raw(mmm) => write!(f, "{mmm:X}H"),
        }
    }
}

impl MemoryControllerError {
    pub fn from_u8(mmm: u8) -> Self {
        match mmm & 0b111 {
            0b000 => Self::GenericUndefinedRequest,
            0b001 => Self::MemoryReadError,
            0b010 => Self::MemoryWriteError,
            0b011 => Self::AddressCommandError,
            0b100 => Self::MemoryScrubbingError,
            mmm => Self::Raw(mmm),
        }
    }

    pub fn from_mcacod(mcacod: u16) -> Self {
        Self::from_u8(((mcacod >> 4) & 0b111) as u8)
    }
}

/// SDM Vol. 3B 18.10.2.5
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Participation {
    OriginatedRequest,
    RespondedToRequest,
    ObservedError,
    Generic,
}

impl fmt::Display for Participation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OriginatedRequest => write!(f, "SRC"),
            Self::RespondedToRequest => write!(f, "RES"),
            Self::ObservedError => write!(f, "OBS"),
            Self::Generic => write!(f, "GEN"),
        }
    }
}

impl Participation {
    pub fn from_u8(pp: u8) -> Self {
        match pp & 0b11 {
            0b00 => Self::OriginatedRequest,
            0b01 => Self::RespondedToRequest,
            0b10 => Self::ObservedError,
            _ => Self::Generic,
        }
    }

    pub fn from_mcacod(mcacod: u16) -> Self {
        Self::from_u8(((mcacod >> 9) & 0b11) as u8)
    }
}

/// SDM Vol. 3B 18.10.2.5
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Timeout {
    Timeout,
    NoTimeout,
}

impl fmt::Display for Timeout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Timeout => write!(f, "TIMEOUT"),
            Self::NoTimeout => write!(f, "NOTIMEOUT"),
        }
    }
}

impl Timeout {
    pub fn from_u8(t: u8) -> Self {
        match t & 1 {
            0 => Self::NoTimeout,
            _ => Self::Timeout,
        }
    }

    pub fn from_mcacod(mcacod: u16) -> Self {
        Self::from_u8(((mcacod >> 8) & 1) as u8)
    }
}

/// SDM Vol. 3B 18.10.2.5
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MemoryOrIo {
    MemoryAccess,
    Reserved,
    Io,
    OtherTransaction,
}

impl fmt::Display for MemoryOrIo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MemoryAccess => write!(f, "M"),
            Self::Reserved => write!(f, "X"),
            Self::Io => write!(f, "IO"),
            Self::OtherTransaction => write!(f, "O"),
        }
    }
}

impl MemoryOrIo {
    pub fn from_u8(ii: u8) -> Self {
        match ii & 0b11 {
            0b00 => Self::MemoryAccess,
            0b01 => Self::Reserved,
            0b10 => Self::Io,
            _ => Self::OtherTransaction,
        }
    }

    pub fn from_mcacod(mcacod: u16) -> Self {
        Self::from_u8(((mcacod >> 2) & 0b11) as u8)
    }
}

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
    GenericCacheHierarchy {
        f: CorrectionReportFiltering,
        ll: MemoryHierarchyLevel,
    },
    TlbErrors {
        f: CorrectionReportFiltering,
        ll: MemoryHierarchyLevel,
        tt: TransactionType,
    },
    MemoryControllerErrors {
        f: CorrectionReportFiltering,
        cccc: Channel,
        mmm: MemoryControllerError,
    },
    CacheHierarchyErrors {
        f: CorrectionReportFiltering,
        ll: MemoryHierarchyLevel,
        tt: TransactionType,
        rrrr: Request,
    },
    ExtendedMemoryErrors {
        f: CorrectionReportFiltering,
        cccc: Channel,
        mmm: MemoryControllerError,
    },
    BusAndInterconnectErrors {
        f: CorrectionReportFiltering,
        ll: MemoryHierarchyLevel,
        ii: MemoryOrIo,
        rrrr: Request,
        t: Timeout,
        pp: Participation,
    },
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
            Self::InternalUnclassified(x) => write!(f, "INTERNAL_UNCLASSIFIED_{x:03X}H"),
            Self::GenericCacheHierarchy { ll, .. } => {
                write!(f, "GENERIC_CACHE_HIERARCHY_ERROR_{ll}")
            }
            Self::TlbErrors { tt, ll, .. } => write!(f, "{tt}TLB{ll}_ERR"),
            Self::MemoryControllerErrors { cccc, mmm, .. } => write!(f, "{mmm}_CHANNEL{cccc}_ERR"),
            Self::CacheHierarchyErrors { ll, tt, rrrr, .. } => {
                write!(f, "{tt}CACHE{ll}_{rrrr}_ERR")
            }
            Self::ExtendedMemoryErrors { cccc, mmm, .. } => write!(f, "{mmm}_CHANNEL{cccc}_ERR"),
            Self::BusAndInterconnectErrors {
                pp,
                t,
                rrrr,
                ii,
                ll,
                ..
            } => {
                write!(f, "BUS{ll}_{pp}_{rrrr}_{ii}_{t}_ERR")
            }
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
            code if code & 0b1111_1100_0000_0000 == 0b0000_0100_0000_0000 => {
                Self::InternalUnclassified(code & 0x3FF)
            }
            code if code & 0b1110_1111_1111_1100 == 0b0000_0000_0000_1100 => {
                Self::GenericCacheHierarchy {
                    ll: MemoryHierarchyLevel::from_mcacod(code),
                    f: CorrectionReportFiltering::from_mcacod(code),
                }
            }
            code if code & 0b1110_1111_1111_0000 == 0b0000_0000_0001_0000 => Self::TlbErrors {
                ll: MemoryHierarchyLevel::from_mcacod(code),
                tt: TransactionType::from_mcacod(code),
                f: CorrectionReportFiltering::from_mcacod(code),
            },
            code if code & 0b1110_1111_1000_0000 == 0b0000_0000_1000_0000 => {
                Self::MemoryControllerErrors {
                    cccc: Channel::from_mcacod(code),
                    mmm: MemoryControllerError::from_mcacod(code),
                    f: CorrectionReportFiltering::from_mcacod(code),
                }
            }
            code if code & 0b1110_1111_0000_0000 == 0b0000_0001_0000_0000 => {
                Self::CacheHierarchyErrors {
                    ll: MemoryHierarchyLevel::from_mcacod(code),
                    tt: TransactionType::from_mcacod(code),
                    rrrr: Request::from_mcacod(code),
                    f: CorrectionReportFiltering::from_mcacod(code),
                }
            }
            code if code & 0b1110_1111_1000_0000 == 0b0000_0010_1000_0000 => {
                Self::ExtendedMemoryErrors {
                    cccc: Channel::from_mcacod(code),
                    mmm: MemoryControllerError::from_mcacod(code),
                    f: CorrectionReportFiltering::from_mcacod(code),
                }
            }
            code if code & 0b1110_1000_0000_0000 == 0b0000_1000_0000_0000 => {
                Self::BusAndInterconnectErrors {
                    ll: MemoryHierarchyLevel::from_mcacod(code),
                    ii: MemoryOrIo::from_mcacod(code),
                    rrrr: Request::from_mcacod(code),
                    t: Timeout::from_mcacod(code),
                    pp: Participation::from_mcacod(code),
                    f: CorrectionReportFiltering::from_mcacod(code),
                }
            }
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
            (0x0000, "NO_ERROR"),
            (0x0001, "UNCLASSIFIED"),
            (0x0002, "MICROCODE_ROM_PARITY_ERROR"),
            (0x0003, "EXTERNAL_ERROR"),
            (0x0004, "FRC_ERROR"),
            (0x0005, "INTERNAL_PARITY_ERROR"),
            (0x0006, "SMM_HANDLER_CODE_ACCESS_VIOLATION"),
            (0x0400, "INTERNAL_TIMER_ERROR"),
            (0x0E0B, "IO_ERROR"),
            (0x042A, "INTERNAL_UNCLASSIFIED_02AH"),
        ];

        for (value, error) in samples {
            assert_eq!(MachineCheckErrorCode::from_u16(value).to_string(), error);
        }
    }

    #[test]
    fn compound_error_codes() {
        let samples = [
            (0x000D, "GENERIC_CACHE_HIERARCHY_ERROR_L1"),
            (0x0012, "ITLBL2_ERR"),
            (0x00AA, "WR_CHANNEL10_ERR"),
            (0x0162, "ICACHEL2_PREFETCH_ERR"),
            (0x02AA, "WR_CHANNEL10_ERR"),
            (0x0962, "BUSL2_SRC_PREFETCH_M_TIMEOUT_ERR"),
        ];

        for (value, error) in samples {
            assert_eq!(MachineCheckErrorCode::from_u16(value).to_string(), error);
        }
    }
}
