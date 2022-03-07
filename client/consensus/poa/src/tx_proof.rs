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

use codec::Encode;

use sp_core::H256;
use sp_runtime::traits::Block as BlockT;

use canyon_primitives::ExtrinsicIndex;
use cp_consensus_poa::encode_index;
use cp_permastore::{TrieLayout, VerifyError};

use crate::trie::{prepare_trie_proof, TrieError};

/// Returns the calculated merkle proof given `extrinsic_index` and `extrinsics_root`.
///
/// # Panics
///
/// Panics if the calculated extrinsic root mismatches.
pub fn build_extrinsic_proof<Block: BlockT<Hash = canyon_primitives::Hash>>(
    extrinsic_index: ExtrinsicIndex,
    extrinsics_root: Block::Hash,
    extrinsics: Vec<Block::Extrinsic>,
) -> Result<Vec<Vec<u8>>, TrieError> {
    let l