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

use super::*;

use std::sync::Arc;

use assert_matches::assert_matches;
use codec::Encode;
use futures::{executor, StreamExt};
use jsonrpc_pubsub::{manager::SubscriptionManager, SubscriptionId};
use parking_lot::RwLock;

use sc_rpc::author::Author;
use sc_rpc_api::{author::hash::ExtrinsicOrHash, DenyUnsafe};
use sc_transaction_pool::{BasicPool, FullChainApi};
use sp_core::{blake2_256, hexdisplay::HexDisplay, H256};
use sp_keystore::testing::KeyStore;
use substrate_test_runtime_client::{
    self,
    runtime::{Block, Extrinsic, Transfer},
    AccountKeyring, Backend, Client, DefaultTestClientBuilderExt, TestClientBuilderExt,
};

fn uxt(sender: AccountKeyring, nonce: u64) -> Extrinsic {
    let tx = Transfer {
        amount: Default::default(),
 