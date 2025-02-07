// This file is part of CORD – https://cord.network

// Copyright (C) Dhiway Networks Pvt. Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later

// CORD is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// CORD is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with CORD. If not, see <https://www.gnu.org/licenses/>.

//! Autogenerated weights for `pallet_did`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-05-10, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `ip-172-31-3-249`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/cord
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_did
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --header=./HEADER-GPL3
// --output=./runtime/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_did`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_did::WeightInfo for WeightInfo<T> {
	/// Storage: Did DidBlacklist (r:1 w:0)
	/// Proof: Did DidBlacklist (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Did Did (r:1 w:1)
	/// Proof: Did Did (max_values: None, max_size: Some(2056), added: 4531, mode: MaxEncodedLen)
	/// Storage: Did DidEndpointsCount (r:0 w:1)
	/// Proof: Did DidEndpointsCount (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	/// Storage: Did ServiceEndpoints (r:0 w:25)
	/// Proof: Did ServiceEndpoints (max_values: None, max_size: Some(413), added: 2888, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 10]`.
	/// The range of component `c` is `[1, 25]`.
	fn create_ed25519_keys(n: u32, c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `42`
		//  Estimated: `5521`
		// Minimum execution time: 97_895_000 picoseconds.
		Weight::from_parts(83_815_511, 0)
			.saturating_add(Weight::from_parts(0, 5521))
			// Standard Error: 18_942
			.saturating_add(Weight::from_parts(1_134_624, 0).saturating_mul(n.into()))
			// Standard Error: 7_323
			.saturating_add(Weight::from_parts(5_642_060, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(c.into())))
	}
	/// Storage: Did DidBlacklist (r:1 w:0)
	/// Proof: Did DidBlacklist (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Did Did (r:1 w:1)
	/// Proof: Did Did (max_values: None, max_size: Some(2056), added: 4531, mode: MaxEncodedLen)
	/// Storage: Did DidEndpointsCount (r:0 w:1)
	/// Proof: Did DidEndpointsCount (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	/// Storage: Did ServiceEndpoints (r:0 w:25)
	/// Proof: Did ServiceEndpoints (max_values: None, max_size: Some(413), added: 2888, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 10]`.
	/// The range of component `c` is `[1, 25]`.
	fn create_sr25519_keys(n: u32, c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `42`
		//  Estimated: `5521`
		// Minimum execution time: 101_095_000 picoseconds.
		Weight::from_parts(85_711_603, 0)
			.saturating_add(Weight::from_parts(0, 5521))
			// Standard Error: 22_551
			.saturating_add(Weight::from_parts(1_248_468, 0).saturating_mul(n.into()))
			// Standard Error: 8_718
			.saturating_add(Weight::from_parts(6_295_082, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(c.into())))
	}
	/// Storage: Did DidBlacklist (r:1 w:0)
	/// Proof: Did DidBlacklist (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Did Did (r:1 w:1)
	/// Proof: Did Did (max_values: None, max_size: Some(2056), added: 4531, mode: MaxEncodedLen)
	/// Storage: Did DidEndpointsCount (r:0 w:1)
	/// Proof: Did DidEndpointsCount (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	/// Storage: Did ServiceEndpoints (r:0 w:25)
	/// Proof: Did ServiceEndpoints (max_values: None, max_size: Some(413), added: 2888, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 10]`.
	/// The range of component `c` is `[1, 25]`.
	fn create_ecdsa_keys(n: u32, c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `42`
		//  Estimated: `5521`
		// Minimum execution time: 87_831_000 picoseconds.
		Weight::from_parts(77_228_770, 0)
			.saturating_add(Weight::from_parts(0, 5521))
			// Standard Error: 19_890
			.saturating_add(Weight::from_parts(896_050, 0).saturating_mul(n.into()))
			// Standard Error: 7_690
			.saturating_add(Weight::from_parts(5_224_827, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(c.into())))
	}
	/// Storage: Did DidEndpointsCount (r:1 w:1)
	/// Proof: Did DidEndpointsCount (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	/// Storage: Did ServiceEndpoints (r:25 w:25)
	/// Proof: Did ServiceEndpoints (max_values: None, max_size: Some(413), added: 2888, mode: MaxEncodedLen)
	/// Storage: Did Did (r:1 w:1)
	/// Proof: Did Did (max_values: None, max_size: Some(2056), added: 4531, mode: MaxEncodedLen)
	/// Storage: Did DidBlacklist (r:0 w:1)
	/// Proof: Did DidBlacklist (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// The range of component `c` is `[1, 25]`.
	fn delete(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `365 + c * (105 ±0)`
		//  Estimated: `5521 + c * (2888 ±0)`
		// Minimum execution time: 27_135_000 picoseconds.
		Weight::from_parts(26_966_101, 0)
			.saturating_add(Weight::from_parts(0, 5521))
			// Standard Error: 5_039
			.saturating_add(Weight::from_parts(1_334_498, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(c.into())))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(c.into())))
			.saturating_add(Weight::from_parts(0, 2888).saturating_mul(c.into()))
	}
	/// Storage: Did Did (r:1 w:1)
	/// Proof: Did Did (max_values: None, max_size: Some(2056), added: 4531, mode: MaxEncodedLen)
	fn submit_did_call_ed25519_key() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `233`
		//  Estimated: `5521`
		// Minimum execution time: 76_471_000 picoseconds.
		Weight::from_parts(78_037_000, 0)
			.saturating_add(Weight::from_parts(0, 5521))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Did Did (r:1 w:1)
	/// Proof: Did Did (max_values: None, max_size: Some(2056), added: 4531, mode: MaxEncodedLen)
	fn submit_did_call_sr25519_key() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `233`
		//  Estimated: `5521`
		// Minimum execution time: 77_455_000 picoseconds.
		Weight::from_parts(79_004_000, 0)
			.saturating_add(Weight::from_parts(0, 5521))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Did Did (r:1 w:1)
	/// Proof: Did Did (max_values: None, max_size: Some(2056), added: 4531, mode: MaxEncodedLen)
	fn submit_did_call_ecdsa_key() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `234`
		//  Estimated: `5521`
		// Minimum execution time: 67_571_000 picoseconds.
		Weight::from_parts(68_894_000, 0)
			.saturating_add(Weight::from_parts(0, 5521))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Did Did (r:1 w:1)
	/// Proof: Did Did (max_values: None, max_size: Some(2056), added: 4531, mode: MaxEncodedLen)
	fn set_ed25519_authentication_key() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1457`
		//  Estimated: `5521`
		// Minimum execution time: 26_751_000 picoseconds.
		Weight::from_parts(27_675_000, 0)
			.saturating_add(Weight::from_parts(0, 5521))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Did Did (r:1 w:1)
	/// Proof: Did Did (max_values: None, max_size: Some(2056), added: 4531, mode: MaxEncodedLen)
	fn set_sr25519_authentication_key() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1457`
		//  Estimated: `5521`
		// Minimum execution time: 26_593_000 picoseconds.
		Weight::from_parts(27_538_000, 0)
			.saturating_add(Weight::from_parts(0, 5521))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Did Did (r:1 w:1)
	/// Proof: Did Did (max_values: None, max_size: Some(2056), added: 4531, mode: MaxEncodedLen)
	fn set_ecdsa_authentication_key() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1460`
		//  Estimated: `5521`
		// Minimum execution time: 26_982_000 picoseconds.
		Weight::from_parts(27_969_000, 0)
			.saturating_add(Weight::from_parts(0, 5521))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Did Did (r:1 w:1)
	/// Proof: Did Did (max_values: None, max_size: Some(2056), added: 4531, mode: MaxEncodedLen)
	fn set_ed25519_delegation_key() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1457`
		//  Estimated: `5521`
		// Minimum execution time: 26_213_000 picoseconds.
		Weight::from_parts(27_383_000, 0)
			.saturating_add(Weight::from_parts(0, 5521))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Did Did (r:1 w:1)
	/// Proof: Did Did (max_values: None, max_size: Some(2056), added: 4531, mode: MaxEncodedLen)
	fn set_sr25519_delegation_key() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1457`
		//  Estimated: `5521`
		// Minimum execution time: 27_024_000 picoseconds.
		Weight::from_parts(27_525_000, 0)
			.saturating_add(Weight::from_parts(0, 5521))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Did Did (r:1 w:1)
	/// Proof: Did Did (max_values: None, max_size: Some(2056), added: 4531, mode: MaxEncodedLen)
	fn set_ecdsa_delegation_key() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1460`
		//  Estimated: `5521`
		// Minimum execution time: 26_579_000 picoseconds.
		Weight::from_parts(27_707_000, 0)
			.saturating_add(Weight::from_parts(0, 5521))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Did Did (r:1 w:1)
	/// Proof: Did Did (max_values: None, max_size: Some(2056), added: 4531, mode: MaxEncodedLen)
	fn remove_ed25519_delegation_key() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1457`
		//  Estimated: `5521`
		// Minimum execution time: 24_596_000 picoseconds.
		Weight::from_parts(25_558_000, 0)
			.saturating_add(Weight::from_parts(0, 5521))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Did Did (r:1 w:1)
	/// Proof: Did Did (max_values: None, max_size: Some(2056), added: 4531, mode: MaxEncodedLen)
	fn remove_sr25519_delegation_key() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1457`
		//  Estimated: `5521`
		// Minimum execution time: 24_665_000 picoseconds.
		Weight::from_parts(25_259_000, 0)
			.saturating_add(Weight::from_parts(0, 5521))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Did Did (r:1 w:1)
	/// Proof: Did Did (max_values: None, max_size: Some(2056), added: 4531, mode: MaxEncodedLen)
	fn remove_ecdsa_delegation_key() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1460`
		//  Estimated: `5521`
		// Minimum execution time: 24_568_000 picoseconds.
		Weight::from_parts(25_422_000, 0)
			.saturating_add(Weight::from_parts(0, 5521))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Did Did (r:1 w:1)
	/// Proof: Did Did (max_values: None, max_size: Some(2056), added: 4531, mode: MaxEncodedLen)
	fn set_ed25519_assertion_key() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1457`
		//  Estimated: `5521`
		// Minimum execution time: 26_828_000 picoseconds.
		Weight::from_parts(27_532_000, 0)
			.saturating_add(Weight::from_parts(0, 5521))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Did Did (r:1 w:1)
	/// Proof: Did Did (max_values: None, max_size: Some(2056), added: 4531, mode: MaxEncodedLen)
	fn set_sr25519_assertion_key() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1457`
		//  Estimated: `5521`
		// Minimum execution time: 26_919_000 picoseconds.
		Weight::from_parts(27_674_000, 0)
			.saturating_add(Weight::from_parts(0, 5521))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Did Did (r:1 w:1)
	/// Proof: Did Did (max_values: None, max_size: Some(2056), added: 4531, mode: MaxEncodedLen)
	fn set_ecdsa_assertion_key() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1460`
		//  Estimated: `5521`
		// Minimum execution time: 26_605_000 picoseconds.
		Weight::from_parts(27_551_000, 0)
			.saturating_add(Weight::from_parts(0, 5521))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Did Did (r:1 w:1)
	/// Proof: Did Did (max_values: None, max_size: Some(2056), added: 4531, mode: MaxEncodedLen)
	fn remove_ed25519_assertion_key() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1457`
		//  Estimated: `5521`
		// Minimum execution time: 24_661_000 picoseconds.
		Weight::from_parts(25_502_000, 0)
			.saturating_add(Weight::from_parts(0, 5521))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Did Did (r:1 w:1)
	/// Proof: Did Did (max_values: None, max_size: Some(2056), added: 4531, mode: MaxEncodedLen)
	fn remove_sr25519_assertion_key() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1457`
		//  Estimated: `5521`
		// Minimum execution time: 24_752_000 picoseconds.
		Weight::from_parts(25_421_000, 0)
			.saturating_add(Weight::from_parts(0, 5521))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Did Did (r:1 w:1)
	/// Proof: Did Did (max_values: None, max_size: Some(2056), added: 4531, mode: MaxEncodedLen)
	fn remove_ecdsa_assertion_key() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1460`
		//  Estimated: `5521`
		// Minimum execution time: 24_738_000 picoseconds.
		Weight::from_parts(25_386_000, 0)
			.saturating_add(Weight::from_parts(0, 5521))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Did Did (r:1 w:1)
	/// Proof: Did Did (max_values: None, max_size: Some(2056), added: 4531, mode: MaxEncodedLen)
	fn add_ed25519_key_agreement_key() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1355`
		//  Estimated: `5521`
		// Minimum execution time: 25_806_000 picoseconds.
		Weight::from_parts(26_834_000, 0)
			.saturating_add(Weight::from_parts(0, 5521))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Did Did (r:1 w:1)
	/// Proof: Did Did (max_values: None, max_size: Some(2056), added: 4531, mode: MaxEncodedLen)
	fn add_sr25519_key_agreement_key() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1355`
		//  Estimated: `5521`
		// Minimum execution time: 26_100_000 picoseconds.
		Weight::from_parts(26_836_000, 0)
			.saturating_add(Weight::from_parts(0, 5521))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Did Did (r:1 w:1)
	/// Proof: Did Did (max_values: None, max_size: Some(2056), added: 4531, mode: MaxEncodedLen)
	fn add_ecdsa_key_agreement_key() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1358`
		//  Estimated: `5521`
		// Minimum execution time: 26_278_000 picoseconds.
		Weight::from_parts(26_805_000, 0)
			.saturating_add(Weight::from_parts(0, 5521))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Did Did (r:1 w:1)
	/// Proof: Did Did (max_values: None, max_size: Some(2056), added: 4531, mode: MaxEncodedLen)
	fn remove_ed25519_key_agreement_key() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1457`
		//  Estimated: `5521`
		// Minimum execution time: 24_812_000 picoseconds.
		Weight::from_parts(25_396_000, 0)
			.saturating_add(Weight::from_parts(0, 5521))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Did Did (r:1 w:1)
	/// Proof: Did Did (max_values: None, max_size: Some(2056), added: 4531, mode: MaxEncodedLen)
	fn remove_sr25519_key_agreement_key() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1457`
		//  Estimated: `5521`
		// Minimum execution time: 24_780_000 picoseconds.
		Weight::from_parts(25_790_000, 0)
			.saturating_add(Weight::from_parts(0, 5521))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Did Did (r:1 w:1)
	/// Proof: Did Did (max_values: None, max_size: Some(2056), added: 4531, mode: MaxEncodedLen)
	fn remove_ecdsa_key_agreement_key() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1460`
		//  Estimated: `5521`
		// Minimum execution time: 24_927_000 picoseconds.
		Weight::from_parts(25_613_000, 0)
			.saturating_add(Weight::from_parts(0, 5521))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Did Did (r:1 w:0)
	/// Proof: Did Did (max_values: None, max_size: Some(2056), added: 4531, mode: MaxEncodedLen)
	/// Storage: Did DidEndpointsCount (r:1 w:1)
	/// Proof: Did DidEndpointsCount (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	/// Storage: Did ServiceEndpoints (r:1 w:1)
	/// Proof: Did ServiceEndpoints (max_values: None, max_size: Some(413), added: 2888, mode: MaxEncodedLen)
	fn add_service_endpoint() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `838`
		//  Estimated: `5521`
		// Minimum execution time: 30_285_000 picoseconds.
		Weight::from_parts(31_595_000, 0)
			.saturating_add(Weight::from_parts(0, 5521))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Did ServiceEndpoints (r:1 w:1)
	/// Proof: Did ServiceEndpoints (max_values: None, max_size: Some(413), added: 2888, mode: MaxEncodedLen)
	/// Storage: Did DidEndpointsCount (r:1 w:1)
	/// Proof: Did DidEndpointsCount (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	fn remove_service_endpoint() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1066`
		//  Estimated: `3878`
		// Minimum execution time: 24_255_000 picoseconds.
		Weight::from_parts(25_672_000, 0)
			.saturating_add(Weight::from_parts(0, 3878))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Did Did (r:1 w:0)
	/// Proof: Did Did (max_values: None, max_size: Some(2056), added: 4531, mode: MaxEncodedLen)
	/// The range of component `l` is `[1, 5242880]`.
	fn signature_verification_sr25519(l: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1459`
		//  Estimated: `5521`
		// Minimum execution time: 66_092_000 picoseconds.
		Weight::from_parts(77_690_914, 0)
			.saturating_add(Weight::from_parts(0, 5521))
			// Standard Error: 3
			.saturating_add(Weight::from_parts(4_347, 0).saturating_mul(l.into()))
			.saturating_add(T::DbWeight::get().reads(1))
	}
	/// Storage: Did Did (r:1 w:0)
	/// Proof: Did Did (max_values: None, max_size: Some(2056), added: 4531, mode: MaxEncodedLen)
	/// The range of component `l` is `[1, 5242880]`.
	fn signature_verification_ed25519(l: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1459`
		//  Estimated: `5521`
		// Minimum execution time: 64_344_000 picoseconds.
		Weight::from_parts(46_784_176, 0)
			.saturating_add(Weight::from_parts(0, 5521))
			// Standard Error: 0
			.saturating_add(Weight::from_parts(2_013, 0).saturating_mul(l.into()))
			.saturating_add(T::DbWeight::get().reads(1))
	}
	/// Storage: Did Did (r:1 w:0)
	/// Proof: Did Did (max_values: None, max_size: Some(2056), added: 4531, mode: MaxEncodedLen)
	/// The range of component `l` is `[1, 5242880]`.
	fn signature_verification_ecdsa(l: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1460`
		//  Estimated: `5521`
		// Minimum execution time: 55_541_000 picoseconds.
		Weight::from_parts(42_955_286, 0)
			.saturating_add(Weight::from_parts(0, 5521))
			// Standard Error: 0
			.saturating_add(Weight::from_parts(964, 0).saturating_mul(l.into()))
			.saturating_add(T::DbWeight::get().reads(1))
	}
}
