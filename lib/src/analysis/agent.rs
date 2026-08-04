// Copyright (C) 2026 Intel Corporation
// SPDX-License-Identifier: MIT

use super::Analyzer;
use super::tag::Tag;
use crate::node::Node;
#[cfg(not(feature = "std"))]
use alloc::fmt;
#[cfg(feature = "std")]
use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CrashLogAgentStatus {
    Success,
    InvalidOpcode,
    RecipeOverflow,
    StorageOverflow,
    InvalidXtor,
    CollectionError,
    NestedCall,
    UnexpectedRet,
    MismatchingAgentId,
    MismatchingStaticConfigRevision,
    IncompleteRecipe,
    RecipeNotLoaded,
    UnsupportedHeaderType,
    CompletionStatusOverflow,
    InvalidTxCode,
    InProgress,
}

impl fmt::Display for CrashLogAgentStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Success => write!(f, "SUCCESS"),
            Self::InvalidOpcode => write!(f, "INVALID_OPCODE"),
            Self::RecipeOverflow => write!(f, "RECIPE_OVERFLOW"),
            Self::StorageOverflow => write!(f, "STORAGE_OVERFLOW"),
            Self::InvalidXtor => write!(f, "INVALID_XTOR"),
            Self::CollectionError => write!(f, "COLLECTION_ERROR"),
            Self::NestedCall => write!(f, "NESTED_CALL"),
            Self::UnexpectedRet => write!(f, "UNEXPECTED_RET"),
            Self::MismatchingAgentId => write!(f, "MISMATCHING_AGENT_ID"),
            Self::MismatchingStaticConfigRevision => {
                write!(f, "MISMATCHING_STATIC_CONFIG_REVISION")
            }
            Self::IncompleteRecipe => write!(f, "INCOMPLETE_RECIPE"),
            Self::RecipeNotLoaded => write!(f, "RECIPE_NOT_LOADED"),
            Self::UnsupportedHeaderType => write!(f, "UNSUPPORTED_HEADER_TYPE"),
            Self::CompletionStatusOverflow => write!(f, "COMPLETION_STATUS_OVERFLOW"),
            Self::InvalidTxCode => write!(f, "INVALID_TX_CODE"),
            Self::InProgress => write!(f, "IN_PROGRESS"),
        }
    }
}

impl CrashLogAgentStatus {
    pub fn from_u64(status: u64) -> Option<Self> {
        match status {
            0 => Some(Self::Success),
            1 => Some(Self::InvalidOpcode),
            2 => Some(Self::RecipeOverflow),
            3 => Some(Self::StorageOverflow),
            4 => Some(Self::InvalidXtor),
            5 => Some(Self::CollectionError),
            6 => Some(Self::NestedCall),
            7 => Some(Self::UnexpectedRet),
            8 => Some(Self::MismatchingAgentId),
            9 => Some(Self::MismatchingStaticConfigRevision),
            10 => Some(Self::IncompleteRecipe),
            11 => Some(Self::RecipeNotLoaded),
            12 => Some(Self::UnsupportedHeaderType),
            13 => Some(Self::CompletionStatusOverflow),
            14 => Some(Self::InvalidTxCode),
            15 => Some(Self::InProgress),
            _ => None,
        }
    }

    fn is_error(&self) -> bool {
        !matches!(self, Self::Success | Self::InProgress)
    }
}

impl Analyzer<'_> {
    pub(super) fn analyze_crashlog_agent(&mut self, agent: &Node) {
        let Some(status) = agent
            .get("status")
            .and_then(|node| node.value())
            .and_then(CrashLogAgentStatus::from_u64)
        else {
            return;
        };

        let Some(ip) = agent
            .get("instruction_pointer")
            .and_then(|node| node.value())
        else {
            return;
        };

        if status.is_error() {
            self.tag(Tag::CrashLogAgentError {
                status,
                instruction_pointer: ip as u32,
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crashlog_agent_record() {
        let mut record = Node::record("crashlog_agent");
        record.add(Node::field("status", 0x5));
        record.add(Node::field("instruction_pointer", 0x2A));

        let mut root = Node::root();
        root.add(record);

        let tags = Analyzer::default().with_input(&root).analyze().tags;

        assert!(
            tags.iter()
                .any(|t| t.to_string() == "CRASHLOG_AGENT_ERROR.COLLECTION_ERROR.IP_2AH")
        );
    }
}
