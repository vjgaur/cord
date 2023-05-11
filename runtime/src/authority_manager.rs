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
//
//! A pallet for managing authorities on CORD.

use sp_staking::SessionIndex;
use sp_std::vec::Vec;

pub use pallet::*;

type Session<T> = pallet_session::Pallet<T>;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{dispatch::DispatchResult, pallet_prelude::*, traits::EnsureOrigin};
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	/// Configuration for the proposer.
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_session::Config {
		/// The overreaching event type.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// Privileged origin that can add or remove validators.
		type AuthorityOrigin: EnsureOrigin<<Self as frame_system::Config>::RuntimeOrigin>;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// New Authorities were added to the set.
		AuthoritiesRegistered(Vec<T::ValidatorId>),
		/// Authorities were removed from the set.
		AuthoritiesDeregistered(Vec<T::ValidatorId>),
	}

	/// Authorities that should be retired.
	#[pallet::storage]
	pub(crate) type AuthoritiesToRetire<T: Config> =
		StorageValue<_, Vec<T::ValidatorId>, ValueQuery>;

	/// Authorities that should be added.
	#[pallet::storage]
	pub(crate) type AuthoritiesToAdd<T: Config> = StorageValue<_, Vec<T::ValidatorId>, ValueQuery>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Add new authorities to the set.
		/// The new authorities will be active from current session + 2.
		#[pallet::call_index(0)]
		#[pallet::weight({0})]
		pub fn register(origin: OriginFor<T>, authorities: Vec<T::ValidatorId>) -> DispatchResult {
			T::AuthorityOrigin::ensure_origin(origin)?;

			authorities.clone().into_iter().for_each(AuthoritiesToAdd::<T>::append);

			Self::deposit_event(Event::AuthoritiesRegistered(authorities));
			Ok(())
		}

		/// Remove authorities from the set.
		/// The removed authorities will be deactivated from current session +
		/// 2.
		#[pallet::call_index(1)]
		#[pallet::weight({0})]
		pub fn deregister(
			origin: OriginFor<T>,
			authorities: Vec<T::ValidatorId>,
		) -> DispatchResult {
			T::AuthorityOrigin::ensure_origin(origin)?;

			authorities.clone().into_iter().for_each(AuthoritiesToRetire::<T>::append);

			Self::deposit_event(Event::AuthoritiesDeregistered(authorities));
			Ok(())
		}
	}

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub authorities: Vec<T::ValidatorId>,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self { authorities: Default::default() }
		}
	}
	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			let authorities = &self.authorities;

			if !authorities.is_empty() {
				AuthoritiesToAdd::<T>::put(authorities);
			}
		}
	}
}

impl<T: Config> pallet_session::SessionManager<T::ValidatorId> for Pallet<T> {
	fn new_session(new_index: SessionIndex) -> Option<Vec<T::ValidatorId>> {
		if new_index <= 1 {
			return None
		}

		let mut authorities = Session::<T>::validators();

		AuthoritiesToRetire::<T>::take().iter().for_each(|v| {
			if let Some(pos) = authorities.iter().position(|r| r == v) {
				authorities.swap_remove(pos);
			}
		});

		AuthoritiesToAdd::<T>::take().into_iter().for_each(|v| {
			if !authorities.contains(&v) {
				authorities.push(v);
			}
		});

		Some(authorities)
	}

	fn end_session(_: SessionIndex) {}

	fn start_session(_start_index: SessionIndex) {}
}
