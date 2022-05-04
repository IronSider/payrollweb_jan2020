
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

//! # Permastore Pallet
//!
//! The Permastore pallet provides the interfaces for storing data
//! onto the network. It also records some information necessary for
//! the PoA consensus on chain.
//!
//! ## Interface
//!
//! ### Dispatchable Functions
//!
//! * `store`: Make an order of storing data.
//! * `forget`: Unimplemented.
//!
//! ### Public Functions
//!
//! See the [`Pallet`] for details of publicly available functions.
//!
//! ### Signed Extensions
//!
//! The Permastore pallet defines the [`CheckStore`] extension which
//! checks the data has been stored in the node and user has sufficient
//! balance to pay the perpetual storage fee.

// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]
#![deny(rustdoc::broken_intra_doc_links)]

use codec::{Decode, Encode};
use scale_info::TypeInfo;

use sp_runtime::{
    traits::{AccountIdConversion, DispatchInfoOf, SaturatedConversion, SignedExtension},
    transaction_validity::{InvalidTransaction, TransactionValidity, TransactionValidityError},
};
use sp_std::{marker::PhantomData, prelude::*};

use frame_support::{
    ensure,
    traits::{Currency, ExistenceRequirement, Get, IsSubType},
    weights::Weight,
};
use frame_system::ensure_signed;

#[cfg(any(feature = "runtime-benchmarks", test))]
mod benchmarking;
#[cfg(all(feature = "std", test))]
mod mock;
#[cfg(all(feature = "std", test))]
mod tests;
pub mod weights;

pub use self::weights::WeightInfo;
// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

/// The balance type of this module.
pub type BalanceOf<T> =
    <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

type ExtrinsicIndex = u32;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::{
        pallet_prelude::*,
        traits::{Currency, Get},
        PalletId,
    };
    use frame_system::pallet_prelude::{BlockNumberFor, OriginFor};

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// The native currency.
        type Currency: Currency<Self::AccountId>;

        /// The treasury pallet id.
        type TreasuryPalletId: Get<PalletId>;

        /// Maximum of a transaction data in bytes.
        type MaxDataSize: Get<u32>;

        /// Weight information for extrinsics in this pallet.
        type WeightInfo: WeightInfo;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]