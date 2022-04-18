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
    BlockAddress<<HashFor<TBlock> as