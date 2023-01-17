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

//! Autogenerated weights for `pallet_assets`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-01-13, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `bm5`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("statemint-dev"), DB CACHE: 1024

// Executed Command:
// ./artifacts/polkadot-parachain
// benchmark
// pallet
// --chain=statemint-dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=pallet_assets
// --extrinsic=*
// --steps=50
// --repeat=20
// --json
// --header=./file_header.txt
// --output=./parachains/runtimes/assets/statemint/src/weights/pallet_assets.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_assets`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_assets::WeightInfo for WeightInfo<T> {
	// Storage: Assets Asset (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn create() -> Weight {
		// Minimum execution time: 28_357 nanoseconds.
		Weight::from_ref_time(28_737_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Assets Asset (r:1 w:1)
	fn force_create() -> Weight {
		// Minimum execution time: 17_524 nanoseconds.
		Weight::from_ref_time(17_866_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Assets Asset (r:1 w:1)
	fn start_destroy() -> Weight {
		// Minimum execution time: 19_424 nanoseconds.
		Weight::from_ref_time(19_887_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Account (r:1 w:0)
	// Storage: System Account (r:20 w:20)
	/// The range of component `c` is `[0, 1000]`.
	fn destroy_accounts(c: u32, ) -> Weight {
		// Minimum execution time: 21_736 nanoseconds.
		Weight::from_ref_time(22_148_000)
			// Standard Error: 13_283
			.saturating_add(Weight::from_ref_time(13_160_503).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(c.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(c.into())))
	}
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Approvals (r:1 w:0)
	/// The range of component `a` is `[0, 1000]`.
	fn destroy_approvals(a: u32, ) -> Weight {
		// Minimum execution time: 22_250 nanoseconds.
		Weight::from_ref_time(22_578_000)
			// Standard Error: 7_877
			.saturating_add(Weight::from_ref_time(12_812_179).saturating_mul(a.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(a.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(a.into())))
	}
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Metadata (r:1 w:0)
	fn finish_destroy() -> Weight {
		// Minimum execution time: 18_779 nanoseconds.
		Weight::from_ref_time(19_302_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Account (r:1 w:1)
	fn mint() -> Weight {
		// Minimum execution time: 30_348 nanoseconds.
		Weight::from_ref_time(31_228_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Account (r:1 w:1)
	fn burn() -> Weight {
		// Minimum execution time: 36_329 nanoseconds.
		Weight::from_ref_time(37_238_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Account (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	fn transfer() -> Weight {
		// Minimum execution time: 47_537 nanoseconds.
		Weight::from_ref_time(48_293_000)
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Account (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	fn transfer_keep_alive() -> Weight {
		// Minimum execution time: 40_899 nanoseconds.
		Weight::from_ref_time(41_737_000)
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Account (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	fn force_transfer() -> Weight {
		// Minimum execution time: 47_595 nanoseconds.
		Weight::from_ref_time(48_737_000)
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	// Storage: Assets Asset (r:1 w:0)
	// Storage: Assets Account (r:1 w:1)
	fn freeze() -> Weight {
		// Minimum execution time: 22_270 nanoseconds.
		Weight::from_ref_time(22_884_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Assets Asset (r:1 w:0)
	// Storage: Assets Account (r:1 w:1)
	fn thaw() -> Weight {
		// Minimum execution time: 22_029 nanoseconds.
		Weight::from_ref_time(22_594_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Assets Asset (r:1 w:1)
	fn freeze_asset() -> Weight {
		// Minimum execution time: 18_807 nanoseconds.
		Weight::from_ref_time(19_126_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Assets Asset (r:1 w:1)
	fn thaw_asset() -> Weight {
		// Minimum execution time: 18_932 nanoseconds.
		Weight::from_ref_time(19_243_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Metadata (r:1 w:0)
	fn transfer_ownership() -> Weight {
		// Minimum execution time: 19_889 nanoseconds.
		Weight::from_ref_time(20_121_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Assets Asset (r:1 w:1)
	fn set_team() -> Weight {
		// Minimum execution time: 18_520 nanoseconds.
		Weight::from_ref_time(18_809_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Assets Asset (r:1 w:0)
	// Storage: Assets Metadata (r:1 w:1)
	/// The range of component `n` is `[0, 50]`.
	/// The range of component `s` is `[0, 50]`.
	fn set_metadata(n: u32, s: u32, ) -> Weight {
		// Minimum execution time: 30_414 nanoseconds.
		Weight::from_ref_time(31_856_176)
			// Standard Error: 808
			.saturating_add(Weight::from_ref_time(1_658).saturating_mul(n.into()))
			// Standard Error: 808
			.saturating_add(Weight::from_ref_time(484).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Assets Asset (r:1 w:0)
	// Storage: Assets Metadata (r:1 w:1)
	fn clear_metadata() -> Weight {
		// Minimum execution time: 32_277 nanoseconds.
		Weight::from_ref_time(32_847_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Assets Asset (r:1 w:0)
	// Storage: Assets Metadata (r:1 w:1)
	/// The range of component `n` is `[0, 50]`.
	/// The range of component `s` is `[0, 50]`.
	fn force_set_metadata(n: u32, s: u32, ) -> Weight {
		// Minimum execution time: 18_916 nanoseconds.
		Weight::from_ref_time(19_686_888)
			// Standard Error: 586
			.saturating_add(Weight::from_ref_time(524).saturating_mul(n.into()))
			// Standard Error: 586
			.saturating_add(Weight::from_ref_time(273).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Assets Asset (r:1 w:0)
	// Storage: Assets Metadata (r:1 w:1)
	fn force_clear_metadata() -> Weight {
		// Minimum execution time: 31_531 nanoseconds.
		Weight::from_ref_time(32_676_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Assets Asset (r:1 w:1)
	fn force_asset_status() -> Weight {
		// Minimum execution time: 17_551 nanoseconds.
		Weight::from_ref_time(17_953_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Approvals (r:1 w:1)
	fn approve_transfer() -> Weight {
		// Minimum execution time: 34_741 nanoseconds.
		Weight::from_ref_time(35_219_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Approvals (r:1 w:1)
	// Storage: Assets Account (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	fn transfer_approved() -> Weight {
		// Minimum execution time: 62_217 nanoseconds.
		Weight::from_ref_time(62_794_000)
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Approvals (r:1 w:1)
	fn cancel_approval() -> Weight {
		// Minimum execution time: 36_806 nanoseconds.
		Weight::from_ref_time(37_187_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Approvals (r:1 w:1)
	fn force_cancel_approval() -> Weight {
		// Minimum execution time: 38_015 nanoseconds.
		Weight::from_ref_time(38_525_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}
