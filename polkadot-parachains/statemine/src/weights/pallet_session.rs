//! Autogenerated weights for pallet_session
//!
//! THIS FILE WAS AUTO-GENERATED USING THE AXLIB BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-06-08, STEPS: `[50, ]`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("statemine-dev"), DB CACHE: 128

// Executed Command:
// ./target/release/axia-collator
// benchmark
// --chain=statemine-dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=pallet_session
// --extrinsic=*
// --steps=50
// --repeat=20
// --raw
// --output=./axia-parachains/statemine-runtime/src/weights

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for pallet_session.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_session::WeightInfo for WeightInfo<T> {
	fn set_keys() -> Weight {
		(25_201_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn purge_keys() -> Weight {
		(17_510_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
}
