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
//!     - Currently, the randome byte is determined by hashing
//!       the parent header hash for N times(see [`calculate_challenge_byte`]),
//!       which will be replaced with another strategy in SPoRA.
//!
//! 2. Locate the extrinsic in which the random byte is included.
//!
//! 3. Check if the data of extrinsic located in Step 2 exists in
//!    the local storage.
//!
//!     - If the data does exist locally, create the two merkle proofs
//!       of extrinsic and data chunks respectively.
//!     - If not, repeat from Step 1 by choosing another random byte
//!       with N+1 hashing.
//!
//! ## Usage
//!
//! Technically, PoA needs to be used with other traditional consensus
//! algorithems like PoW or PoS together as it's not typically designed
//! for solving the problem of selecting one from a set of validators
//! to create next block in an unpredictable or fair way. In another word,
//! PoA is not intended for resolving the leader election problem, and
//! is usually exploited as a precondition for PoW or PoS in order to
//! encourage the miners to store more data locally.
//!
//! This crate implements the core algorithem of Proof of Access in
//! [`construct_poa`] and provides the inherent data provider via
//! [`PoaInherentDataProvider`]. [`PurePoaBlockImport`] implements the
//! `BlockImport` trait, thus can be wrapped in another block importer.
//!
//! To use this engine, you can create an inhehrent extrinsic using the
//! data provided by [`PoaInherentDataProvider`] in a pallet, refer to
//! [`pallet_poa::Call::deposit`] as an 