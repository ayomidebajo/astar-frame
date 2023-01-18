//! Autogenerated weights for `pallet_xc_asset_config`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-12-29, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `shiden-collator-02-ovh`, CPU: `Intel(R) Xeon(R) E-2136 CPU @ 3.30GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("astar-dev"), DB CACHE: 1024

// Executed Command:
// ./astar-collator
// benchmark
// pallet
// --chain
// astar-dev
// --execution
// wasm
// --wasm-execution
// compiled
// --heap-pages
// 4096
// --pallet
// pallet_xc_asset_config
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --output
// pallet_xc_asset_config_weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_asset_manager.
pub trait WeightInfo {
	fn register_asset_location() -> Weight;
	fn set_asset_units_per_second() -> Weight;
	fn change_existing_asset_location() -> Weight;
	fn remove_payment_asset() -> Weight;
	fn remove_asset() -> Weight;
}

/// Weights for pallet_asset_manager using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: XcAssetConfig AssetIdToLocation (r:1 w:1)
	// Storage: EVM AccountCodes (r:0 w:1)
	// Storage: XcAssetConfig AssetLocationToId (r:0 w:1)
	fn register_asset_location() -> Weight {
		Weight::from_ref_time(19_576_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: XcAssetConfig AssetLocationToId (r:1 w:0)
	// Storage: XcAssetConfig AssetLocationUnitsPerSecond (r:0 w:1)
	fn set_asset_units_per_second() -> Weight {
		Weight::from_ref_time(19_287_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: XcAssetConfig AssetIdToLocation (r:1 w:1)
	// Storage: XcAssetConfig AssetLocationUnitsPerSecond (r:1 w:2)
	// Storage: XcAssetConfig AssetLocationToId (r:0 w:2)
	fn change_existing_asset_location() -> Weight {
		Weight::from_ref_time(24_640_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
	// Storage: XcAssetConfig AssetLocationUnitsPerSecond (r:0 w:1)
	fn remove_payment_asset() -> Weight {
		Weight::from_ref_time(14_878_000 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: XcAssetConfig AssetIdToLocation (r:1 w:1)
	// Storage: EVM AccountCodes (r:0 w:1)
	// Storage: XcAssetConfig AssetLocationUnitsPerSecond (r:0 w:1)
	// Storage: XcAssetConfig AssetLocationToId (r:0 w:1)
	fn remove_asset() -> Weight {
		Weight::from_ref_time(22_900_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: XcAssetConfig AssetIdToLocation (r:1 w:1)
	// Storage: EVM AccountCodes (r:0 w:1)
	// Storage: XcAssetConfig AssetLocationToId (r:0 w:1)
	fn register_asset_location() -> Weight {
		Weight::from_ref_time(19_576_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}
	// Storage: XcAssetConfig AssetLocationToId (r:1 w:0)
	// Storage: XcAssetConfig AssetLocationUnitsPerSecond (r:0 w:1)
	fn set_asset_units_per_second() -> Weight {
		Weight::from_ref_time(19_287_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: XcAssetConfig AssetIdToLocation (r:1 w:1)
	// Storage: XcAssetConfig AssetLocationUnitsPerSecond (r:1 w:2)
	// Storage: XcAssetConfig AssetLocationToId (r:0 w:2)
	fn change_existing_asset_location() -> Weight {
		Weight::from_ref_time(24_640_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(5 as u64))
	}
	// Storage: XcAssetConfig AssetLocationUnitsPerSecond (r:0 w:1)
	fn remove_payment_asset() -> Weight {
		Weight::from_ref_time(14_878_000 as u64)
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: XcAssetConfig AssetIdToLocation (r:1 w:1)
	// Storage: EVM AccountCodes (r:0 w:1)
	// Storage: XcAssetConfig AssetLocationUnitsPerSecond (r:0 w:1)
	// Storage: XcAssetConfig AssetLocationToId (r:0 w:1)
	fn remove_asset() -> Weight {
		Weight::from_ref_time(22_900_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(4 as u64))
	}
}
