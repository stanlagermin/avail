// This file is part of Substrate.

// Copyright (C) 2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for `nomad_home`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-09-13, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `ip-172-31-12-189`, CPU: `Intel(R) Xeon(R) Platinum 8175M CPU @ 2.50GHz`
//! EXECUTION: ``, WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: 1024

// Executed Command:
// ./target/release/data-avail
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=nomad_home
// --extra
// --extrinsic=*
// --heap-pages=4096
// --header=./HEADER-APACHE2
// --log=warn
// --output
// ./output/nomad_home.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `nomad_home`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> nomad_home::WeightInfo for WeightInfo<T> {
	/// Storage: `NomadHome::Base` (r:1 w:1)
	/// Proof: `NomadHome::Base` (`max_values`: Some(1), `max_size`: Some(57), added: 552, mode: `MaxEncodedLen`)
	/// Storage: `NomadHome::RootToIndex` (r:1 w:0)
	/// Proof: `NomadHome::RootToIndex` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	fn improper_update() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `122`
		//  Estimated: `3509`
		// Minimum execution time: 529_760_000 picoseconds.
		Weight::from_parts(532_833_000, 0)
			.saturating_add(Weight::from_parts(0, 3509))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `NomadHome::Base` (r:1 w:0)
	/// Proof: `NomadHome::Base` (`max_values`: Some(1), `max_size`: Some(57), added: 552, mode: `MaxEncodedLen`)
	/// Storage: `NomadHome::Nonces` (r:1 w:1)
	/// Proof: `NomadHome::Nonces` (`max_values`: None, `max_size`: Some(16), added: 2491, mode: `MaxEncodedLen`)
	/// Storage: `NomadHome::Tree` (r:1 w:1)
	/// Proof: `NomadHome::Tree` (`max_values`: Some(1), `max_size`: Some(1028), added: 1523, mode: `MaxEncodedLen`)
	/// Storage: `NomadHome::IndexToRoot` (r:0 w:1)
	/// Proof: `NomadHome::IndexToRoot` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	/// Storage: `NomadHome::RootToIndex` (r:0 w:1)
	/// Proof: `NomadHome::RootToIndex` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	/// The range of component `b` is `[1, 2048]`.
	fn dispatch(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1139`
		//  Estimated: `3481`
		// Minimum execution time: 110_556_000 picoseconds.
		Weight::from_parts(114_320_097, 0)
			.saturating_add(Weight::from_parts(0, 3481))
			// Standard Error: 219
			.saturating_add(Weight::from_parts(8_050, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: `NomadHome::Base` (r:1 w:1)
	/// Proof: `NomadHome::Base` (`max_values`: Some(1), `max_size`: Some(57), added: 552, mode: `MaxEncodedLen`)
	/// Storage: `NomadHome::RootToIndex` (r:32 w:32)
	/// Proof: `NomadHome::RootToIndex` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	/// Storage: `NomadHome::IndexToRoot` (r:31 w:32)
	/// Proof: `NomadHome::IndexToRoot` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	fn update() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3350`
		//  Estimated: `81598`
		// Minimum execution time: 934_876_000 picoseconds.
		Weight::from_parts(969_516_000, 0)
			.saturating_add(Weight::from_parts(0, 81598))
			.saturating_add(T::DbWeight::get().reads(64))
			.saturating_add(T::DbWeight::get().writes(65))
	}
	/// Storage: `NomadHome::Base` (r:1 w:1)
	/// Proof: `NomadHome::Base` (`max_values`: Some(1), `max_size`: Some(57), added: 552, mode: `MaxEncodedLen`)
	/// Storage: `NomadUpdaterManager::Updater` (r:1 w:1)
	/// Proof: `NomadUpdaterManager::Updater` (`max_values`: Some(1), `max_size`: Some(20), added: 515, mode: `MaxEncodedLen`)
	fn set_updater() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `171`
		//  Estimated: `1542`
		// Minimum execution time: 36_352_000 picoseconds.
		Weight::from_parts(37_540_000, 0)
			.saturating_add(Weight::from_parts(0, 1542))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}
