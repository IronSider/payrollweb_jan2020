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

#![deny(missing_docs, unused_extern_crates)]

//! This crate provides the feature of persistent storage for the transaction data
//! expected to exist indefinitely.
//!
//! Currently, it is implemented on the top of offchain storage, which is a persistent
//! local storage of each node.

#[cfg(test)]
mod tests;

use std::sync::Arc;

use codec::Encode;

use sc_client_db::offchain::LocalStorage;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{
    generic::BlockId,
    offchain::OffchainStorage,
    traits::{Block as BlockT, NumberFor},
};

use cp_permastore::{PermaStorage, PermastoreApi};

/// Permanent storage backed by offchain storage.
#[derive(Clone)]
pub struct PermanentStorage<C> {
    offchain_storage: LocalStorage,
    client: Arc<C>,
}

impl<C> PermanentStorage<C> {
    /// Creates new perma storage for tests.
    #[cfg(any(feature = "test-helpers", test))]
    pub fn new_test(client: Arc<C>) -> Self {
        Self {
            offchain_storage: LocalStorage::new_test(),
            client,
        }
    }

    /// Creates a new instance of [`PermaStorage`] backed by offchain storage.
    pub fn new(offchain_storage: LocalStorage, client: Arc<C>) -> Self {
        Self {
            offchain_storage,
        