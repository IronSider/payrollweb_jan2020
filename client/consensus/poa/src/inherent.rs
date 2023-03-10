
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

use std::sync::Arc;

use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::traits::{Block as BlockT, NumberFor};

use sc_client_api::BlockBackend;

use cc_datastore::TransactionDataBackend as TransactionDataBackendT;
use cp_consensus_poa::{PoaOutcome, POA_INHERENT_IDENTIFIER};

/// A type for creating the inherent data for pallet poa.
pub struct PoaInherentDataProvider {
    /// Outcome of creating a proof of access.
    pub poa_outcome: PoaOutcome,
}

impl PoaInherentDataProvider {
    /// Creates a new instance of [`PoaInherentDataProvider`].
    pub fn create<Block, Client, TransactionDataBackend>(
        client: Arc<Client>,
        parent: Block::Hash,
        transaction_data_backend: TransactionDataBackend,
    ) -> Result<Self, crate::Error<Block>>
    where
        Block: BlockT<Hash = canyon_primitives::Hash> + 'static,
        Client: BlockBackend<Block>
            + HeaderBackend<Block>
            + ProvideRuntimeApi<Block>
            + Send
            + Sync
            + 'static,
        Client::Api: cp_permastore::PermastoreApi<Block, NumberFor<Block>, u32, Block::Hash>
            + cp_poa::PoaApi<Block>,
        TransactionDataBackend: TransactionDataBackendT<Block>,
    {
        let poa_outcome = match crate::construct_poa(client, parent, transaction_data_backend) {
            Ok(outcome) => outcome,
            Err(e) => {
                log::error!(target: "poa", "Failed to construct poa: {:?}", e);
                return Err(e);
            }
        };
        Ok(Self { poa_outcome })
    }
}

#[async_trait::async_trait]
impl sp_inherents::InherentDataProvider for PoaInherentDataProvider {
    fn provide_inherent_data(
        &self,
        inherent_data: &mut sp_inherents::InherentData,
    ) -> Result<(), sp_inherents::Error> {
        inherent_data.put_data(POA_INHERENT_IDENTIFIER, &self.poa_outcome)
    }

    async fn try_handle_error(
        &self,
        _: &sp_inherents::InherentIdentifier,
        _: &[u8],
    ) -> Option<Result<(), sp_inherents::Error>> {
        // Inherent isn't checked and can not return any error
        None
    }
}