// This file is part of Canyon.

// Copyright (C) 2021 Canyon Network.
// License: GPL-3.0

//! Structs to easily compose inspect sub-command for CLI.

use std::fmt::Debug;

use structopt::StructOpt;

use sc_cli::{ImportParams, SharedParams};

/// The `inspect` command used to print decoded chain data.
#[derive(Debug, StructOpt)]
pub struct InspectCmd {
    #[allow(missing_docs)]
    #[structopt(flatten)]
    pub command: InspectSubCmd,

    #[allow(missing_docs)]
    #[structopt(flatten)]
    pub shared_params: SharedParams,

    #[allow(missing_docs)]
    #[structopt(flatten)]
    pub import_params: ImportParams,
}

/// A possible inspect sub-commands.
#[derive(Debug, StructOpt)]
pub enum InspectSubCmd {
    /// Decode block with native version of runtime and print out the details.
    Block {
        /// Address of the block to print out.
        ///
        /// Can be either a block hash (no 0x prefix) or a number to retrieve existing bl