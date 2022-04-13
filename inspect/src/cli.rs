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

    #[allow(missing_do