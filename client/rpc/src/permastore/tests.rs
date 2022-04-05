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
        nonce,
        from: sender.into(),
        to: Default::default(),
    };
    tx.into_signed_tx()
}

type FullTransactionPool = BasicPool<FullChainApi<Client<Backend>, Block>, Block>;

type TestAuthor = Author<FullTransactionPool, Client<Backend>>;

struct TestSetup {
    pub client: Arc<Client<Backend>>,
    pub keystore: Arc<KeyStore>,
    pub pool: Arc<FullTransactionPool>,
}

impl Default for TestSetup {
    fn default() -> Self {
        let keystore = Arc::new(KeyStore::new());
        let client_builder = substrate_test_runtime_client::TestClientBuilder::new();
        let client = Arc::new(client_builder.set_keystore(keystore.clone()).build());

        let spawner = sp_core::testing::TaskExecutor::new();
        let pool = BasicPool::new_full(
            Default::default(),
            true.into(),
            None,
            spawner,
            client.clone(),
        );

        TestSetup {
            client,
            keystore,
            pool,
        }
    }
}

impl TestSetup {
    fn author(&self) -> TestAuthor {
        let subscriptions = SubscriptionManager::new(Arc::new(sc_rpc::testing::TaskExecutor));
        sc_rpc::author::Author::new(
            self.client.clone(),
            self.pool.clone(),
            subscriptions,
            self.keystore.clone(),
            DenyUnsafe::No,
        )
    }

    fn permastore(
        &self,
    ) -> Permastore<
        cc_datastore::PermanentStorage<Client<Backend>>,
        FullTransactionPool,
        TestAuthor,
        Block,
    > {
        Permastore {
            storage: Arc::new(RwLock::new(cc_datastore::PermanentStorage::new_test(
                self.client.clone(),
            ))),
            pool: self.pool.clone(),
            author: self.author(),
            deny_unsafe: DenyUnsafe::No,
            phatom: PhantomData::<Block>,
        }
    }
}

#[test]
fn submit_transaction_should_not_cause_error() {
    let p = TestSetup::default().permastore();
    let xt = uxt(AccountKeyring::Alice, 1).encode();
    let h: H256 = blake2_256(&xt).into();

    let data = b"mocked data".to_vec();
    assert_matches!(
        executor::block_on(PermastoreApi::submit_extrinsic(&p, xt.clone().into(), data.clone().into())),
        Ok(h2) if h == h2
    );
    assert!(
        executor::block_on(PermastoreApi::submit_extrinsic(&p, xt.into(), data.into())).is_err()
    );
}

#[test]
fn submit_rich_transaction_should_not_cause_error() {
    let p = TestSetup::default().permastore();
    let xt = uxt(AccountKeyring::Alice, 0).encode();
    let h: H256 = blake2_256(&xt).into();

    let data = b"mocked data".to_vec();
    assert_matches!(
        executor::block_on(PermastoreApi::submit_extrinsic(&p, xt.clone().into(), data.clone().into())),
        Ok(h2) if h == h2
    );
    assert!(
        executor::block_on(PermastoreApi::submit_extrinsic(&p, xt.into(), data.into())).is_err()
    );
}

#[test]
fn should_watch_extrinsic() {
    // given
    let setup = TestSetup::default();
    let p = setup.author();

    let (subscriber, id_rx, data) = jsonrpc_pubsub::typed::Subscriber::new_test("test");

    // when
    p.watch_extrinsic(
        Default::default(),
        subscriber,
        uxt(AccountKeyring::Alice, 0).encode().into(),
    );

    let id = executor::block_on(id_rx).unwrap().unwrap();
    assert_matches!(id, SubscriptionId::String(_));

    let id = match id {
        SubscriptionId::String(id) => id,
        _ => unreachable!(),
    };

    // check notifications
    let replacement = {
        let tx = Transfer {
            amount: 5,
            nonce: 0,
            fr