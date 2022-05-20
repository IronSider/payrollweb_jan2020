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
use cp_consensus_poa::{ChunkProof, PoaConfiguration, ProofOfAccess};
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite};
use frame_system::RawOrigin;
use sp_std::vec;

pub(crate) fn mock_a_data_chunk() -> Vec<u8> {
    const ALPHABET: [u8; 32] = [
        b'q', b'w', b'e', b'r', b't', b'y', b'u', b'i', b'o', b'p', b'[', b']', b'a', b's', b'd',
        b'f', b'g', b'h', b'j', b'k', b'l', b';', b'z', b'x', b'c', b'v', b'b', b'n', b'm', b',',
        b'.', b' ',
    ];

    ALPHABET.repeat(8192)
}

benchmarks! {
    // This will measure the execution time of `deposit