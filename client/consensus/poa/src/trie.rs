
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

use sp_trie::{empty_trie_root, MemoryDB, TrieDBMut, TrieMut};

use cp_consensus_poa::encode_index;
use cp_permastore::{Hasher, TrieLayout};

/// Error type for building a trie proof.
#[derive(Debug, thiserror::Error)]
pub enum TrieError {
    /// Trie error.
    #[error(transparent)]
    Trie(#[from] Box<dyn std::error::Error + Send + Sync>),
}

/// Prepares the components for building a trie proof given the final leaf nodes.
///
/// # Panics
///
/// Panics if the insertion of trie node failed.
pub fn prepare_trie_proof(leaves: Vec<Vec<u8>>) -> (MemoryDB<Hasher>, sp_core::H256) {
    let mut db = MemoryDB::<Hasher>::default();
    let mut root = empty_trie_root::<TrieLayout>();

    {
        let mut trie = TrieDBMut::<TrieLayout>::new(&mut db, &mut root);

        for (index, leaf) in leaves.iter().enumerate() {
            trie.insert(&encode_index(index as u32), leaf)
                .unwrap_or_else(|e| {
                    panic!("Failed to insert the trie node: {:?}, index: {}", e, index)
                });
        }

        trie.commit();
    }

    (db, root)
}