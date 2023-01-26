// Copyright 2021 Parity Technologies (UK) Ltd.
// This file is part of Cumulus.

// Cumulus is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Cumulus is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Cumulus.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for `cumulus_pallet_xcmp_queue`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-01-25, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `runner-b3zmxxc-project-238-concurrent-0`, CPU: `Intel(R) Xeon(R) CPU @ 2.60GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("bridge-hub-kusama-dev"), DB CACHE: 1024

// Executed Command:
// ./artifacts/polkadot-parachain
// benchmark
// pallet
// --chain=bridge-hub-kusama-dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=cumulus_pallet_xcmp_queue
// --extrinsic=*
// --steps=50
// --repeat=20
// --json
// --header=./file_header.txt
// --output=./parachains/runtimes/bridge-hubs/bridge-hub-kusama/src/weights/cumulus_pallet_xcmp_queue.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `cumulus_pallet_xcmp_queue`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> cumulus_pallet_xcmp_queue::WeightInfo for WeightInfo<T> {
	// Storage: XcmpQueue QueueConfig (r:1 w:1)
	fn set_config_with_u32() -> Weight {
		// Minimum execution time: 6_738 nanoseconds.
		Weight::from_ref_time(7_394_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: XcmpQueue QueueConfig (r:1 w:1)
	fn set_config_with_weight() -> Weight {
		// Minimum execution time: 6_921 nanoseconds.
		Weight::from_ref_time(7_291_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
