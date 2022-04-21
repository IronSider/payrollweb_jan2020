// This file is part of Canyon.

// Copyright (C) 2021 Canyon Network.
// License: GPL-3.0

//! A CLI extension for substrate node, adding sub-command to pretty print debug info
//! about blocks and extrinsics.
//!
//! The blocks and extrinsics can either be retrieved from the database (on-chain),
//! or a raw SCALE-encoding can be provided.

#![warn(missing_docs)]

pub mod cli;
pub mod command;

use std::{fmt, fmt::Debug, marker::PhantomData, str::FromStr};

use codec::{Decode, Encode};

use sc_client_api::BlockBackend;
use sp_blockchain::HeaderBackend;
use sp_core::hexdisplay::HexDisplay;
use sp_runtime::{
    generic::BlockId,
    traits::{Block, Hash, HashFor, NumberFor},
};

/// A helper type for a generic block input.
pub type BlockAddressFor<TBlock> =
    BlockAddress<<HashFor<TBlock> as Hash>::Output, NumberFor<TBlock>>;

/// A Pretty formatter implementation.
pub trait PrettyPrinter<TBlock: Block> {
    /// Nicely format block.
    fn fmt_block(&self, fmt: &mut fmt::Formatter, block: &TBlock) -> fmt::Result;
    /// Nicely format extrinsic.
    fn fmt_extrinsic(&self, fmt: &mut fmt::Formatter, extrinsic: &TBlock::Extrinsic)
        -> fmt::Result;
}

/// Default dummy debug printer.
#[derive(Default)]
pub struct DebugPrinter;
impl<TBlock: Block> PrettyPrinter<TBlock> for DebugPrinter {
    fn fmt_block(&self, fmt: &mut fmt::Formatter, block: &TBlock) -> fmt::Result {
        writeln!(fmt, "Header:")?;
        writeln!(fmt, "{:?}", block.header())?;
        writeln!(fmt, "Block bytes: {:?}", HexDisplay::from(&block.encode()))?;
        writeln!(fmt, "Extrinsics ({})", block.extrinsics().len())?;
        for (idx, ex) in block.extrinsics().iter().enumerate() {
            writeln!(fmt, "- {}:", idx)?;
            <DebugPrinter as PrettyPrinter<TBlock>>::fmt_extrinsic(self, fmt, ex)?;
        }
        Ok(())
    }

    fn fmt_extrinsic(
        &self,
        fmt: &mut fmt::Formatter,
        extrinsic: &TBlock::Extrinsic,
    ) -> fmt::Result {
        writeln!(fmt, " {:#?}", extrinsic)?;
        writeln!(fmt, " Bytes: {:?}", HexDisplay::from(&extrinsic.encode()))?;
        Ok(())
    }
}

/// Aggregated error for `Inspector` operations.
#[derive(Debug, derive_more::From, derive_more::Display)]
pub enum Error {
    /// Could not decode Block or Extrinsic.
    Codec(codec::Error),
    /// Error accessing blockchain DB.
    Blockchain(sp_blockchain::Error),
    /// Given block has not been found.
    NotFound(String),
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Codec(ref e) => Some(e),
            Self::Blockchain(ref e) => Some(e),
            Self::NotFound(_) => None,
        }
    }
}

/// A helper trait to access block headers and bodies.
pub trait ChainAccess<TBlock: Block>: HeaderBackend<TBlock> 