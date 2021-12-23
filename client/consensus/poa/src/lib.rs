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
//! [`pallet_poa::Call::deposit`] as an example.  Furthermore, you need
//! to wrap the [`PurePoaBlockImport`] into your existing block import
//! pipeline. Refer to the [Substrate docs][1] for more information about
//! creating a nested `BlockImport`.
//!
//! [1]: https://substrate.dev/docs/en/knowledgebase/advanced/block-import
//! [`pallet_poa::Call::deposit`]: ../pallet_poa/pallet/enum.Call.html#variant.deposit

#![deny(missing_docs, unused_extern_crates)]

use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::Arc;

use codec::{Decode, Encode};
use thiserror::Error;

use sc_client_api::{backend::AuxStore, BlockBackend, BlockOf};
use sc_consensus::{BlockCheckParams, BlockImport, BlockImportParams, ImportResult};
use sp_api::ProvideRuntimeApi;
use sp_block_builder::BlockBuilder as BlockBuilderApi;
use sp_blockchain::{well_known_cache_keys::Id as CacheKeyId, HeaderBackend, ProvideCache};
use sp_consensus::{Error as ConsensusError, SelectChain};
use sp_runtime::{
    generic::BlockId,
    traits::{Block as BlockT, Header as HeaderT, NumberFor},
    DigestItem,
};

use canyon_primitives::{DataIndex, Depth, ExtrinsicIndex};
use cc_datastore::TransactionDataBackend as TransactionDataBackendT;
use cp_permastore::{PermastoreApi, CHUNK_SIZE};
use cp_poa::PoaApi;

mod chunk_proof;
mod inherent;
mod trie;
mod tx_proof;

pub use self::chunk_proof::{verify_chunk_proof, ChunkProofBuilder, ChunkProofVerifier};
pub use self::inherent::PoaInherentDataProvider;
pub use self::tx_proof::{build_extrinsic_proof, verify_extrinsic_proof, TxProofVerifier};

// Re-exports of the primitives of poa consensus.
pub use cp_consensus_poa::{
    ChunkProof, PoaConfiguration, PoaOutcome, PoaValidityError, ProofOfAccess, POA_ENGINE_ID,
};

/// Minimum depth of PoA.
const MIN_DEPTH: u32 = 1;

type Randomness = Vec<u8>;

/// Error type for poa consensus.
#[derive(Error, Debug)]
pub enum Error<Block: BlockT> {
    /// No PoA seal in the header.
    #[error("Header {0:?} has no PoA digest")]
    NoDigest(Block::Hash),
    /// Multiple PoA seals were found in the header.
    #[error("Header {0:?} has multiple PoA digests")]
    MultipleDigests(Block::Hash),
    /// Client error.
    #[error("Client error: {0}")]
    Client(sp_blockchain::Error),
    /// Codec error.
    #[error("Codec error: {0}")]
    Codec(#[from] codec::Error),
    /// Blockchain error.
    #[error("Blockchain error: {0}")]
    BlockchainError(#[from] sp_blockchain::Error),
    /// Invalid ProofOfAccess.
    #[error("Invalid ProofOfAccess: {0:?}")]
    InvalidPoa(PoaValidityError),
    /// Failed to verify the merkle proof.
    #[error("VerifyError error: {0:?}")]
    VerifyFailed(#[from] cp_permastore::VerifyError),
    /// Runtime api error.
    #[error(transparent)]
    ApiError(#[from] sp_api::ApiError),
    /// Chunk root not found.
    #[error("Chunk root not found for the recall extrinsic {0}#{1}")]
    ChunkRootNotFound(BlockId<Block>, ExtrinsicIndex),
    /// Block not found.
    #[error("Block {0} not found")]
    BlockNotFound(BlockId<Block>),
    /// Recall block not found.
    #[