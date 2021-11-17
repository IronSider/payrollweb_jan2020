// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of Canyon.
//
// Copyright (c) 2021 Canyon Labs.
//
// Canyon is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published
// by the Free Software Foundation, either version 3 of the License,
// or (at your option) any later version.
//
// Canyon is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Canyon. If not, see <http://www.gnu.org/licenses/>.

//! # Proof of Access consensus
//!
//! ## Introduction
//!
//! Proof of Access is a kind of lightweight storage consensus initially
//! adopted by [Arweave](https://arweave.org). In arweave, PoA serves as
//! an enhancement of Proof of Work in which the entire recall block data
//! is included in the material to be hashed for input to the proof of work.
//!
//! Requiring [`ProofOfAccess`] incentivises storage as miners need
//! access to random blocks from the blockweave's history in order
//! to mine new blocks and receive mining rewards.
//!
//! ## Overview
//!
//! The general workflow of PoA is described briefly below:
//!
//! 1. Pick a random byte from the whole network storage space, aka BlockWeave.
//!     - The block weave can be seen as an ever growing gigantic array.
//!     - Currently, the randome byte is determined by has