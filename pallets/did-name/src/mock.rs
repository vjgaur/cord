// This file is part of CORD – https://cord.network

// Copyright (C) 2019-2023 BOTLabs GmbH.
// Copyright (C) Dhiway Networks Pvt. Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later
// Adapted to meet the requirements of the CORD project.

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
use self::did_name::AsciiDidName;
use super::*;
use crate as pallet_did_names;
use cord_utilities::mock::{mock_origin, SubjectId};
use frame_support::{
	construct_runtime, parameter_types,
	traits::{ConstU32, ConstU64},
};
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentifyAccount, IdentityLookup, Verify},
	MultiSignature,
};

use frame_system::EnsureRoot;

type Hash = sp_core::H256;
type Signature = MultiSignature;
type AccountPublic = <Signature as Verify>::Signer;
type AccountId = <AccountPublic as IdentifyAccount>::AccountId;

construct_runtime!(
	pub enum Test where
		Block = frame_system::mocking::MockBlock<Test>,
		NodeBlock = frame_system::mocking::MockBlock<Test>,
		UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		DidNames: pallet_did_names::{Pallet, Storage, Call, Event<T>},
		MockOrigin: mock_origin::{Pallet, Origin<T>},
	}
);

parameter_types! {
	pub const SS58Prefix: u8 = 29;
}

impl frame_system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = Hash;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = ConstU64<250>;
	type DbWeight = ();
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = SS58Prefix;
	type OnSetCode = ();
	type MaxConsumers = ConstU32<2>;
}

pub(crate) type TestDidName = AsciiDidName<Test>;
pub(crate) type TestDidNameOwner = SubjectId;
pub(crate) type TestDidNamePayer = AccountId;
pub(crate) type TestOwnerOrigin =
	mock_origin::EnsureDoubleOrigin<TestDidNamePayer, TestDidNameOwner>;
pub(crate) type TestOriginSuccess = mock_origin::DoubleOrigin<TestDidNamePayer, TestDidNameOwner>;
pub(crate) type TestBanOrigin = EnsureRoot<AccountId>;

parameter_types! {
	pub const MaxNameLength: u32 = 64;
	pub const MinNameLength: u32 = 3;
	pub const MaxPrefixLength: u32 = 54;
}

impl Config for Test {
	type BanOrigin = TestBanOrigin;
	type EnsureOrigin = TestOwnerOrigin;
	type OriginSuccess = TestOriginSuccess;
	type RuntimeEvent = RuntimeEvent;
	type MaxNameLength = MaxNameLength;
	type MinNameLength = MinNameLength;
	type MaxPrefixLength = MaxPrefixLength;
	type DidName = TestDidName;
	type DidNameOwner = TestDidNameOwner;
	type WeightInfo = ();
}

impl mock_origin::Config for Test {
	type RuntimeOrigin = RuntimeOrigin;
	type AccountId = AccountId;
	type SubjectId = SubjectId;
}

#[allow(dead_code)]
pub(crate) fn new_test_ext() -> sp_io::TestExternalities {
	let t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();

	let mut ext = sp_io::TestExternalities::new(t);
	#[cfg(feature = "runtime-benchmarks")]
	let keystore = sp_keystore::testing::MemoryKeystore::new();
	#[cfg(feature = "runtime-benchmarks")]
	ext.register_extension(sp_keystore::KeystoreExt(sp_std::sync::Arc::new(keystore)));
	ext.execute_with(|| System::set_block_number(1));
	ext
}
