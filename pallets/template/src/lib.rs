#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode, MaxEncodedLen};
/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;
use sp_std::{
	cmp::{Eq, PartialEq},
	vec::Vec,
};

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[derive(Encode, Decode, Eq, PartialEq, Copy, Clone, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub enum Triger {
	Timer(u64, u64),    //insert_time,  timer_seconds
	Schedule(u64, u64), //insert_time,  timestamp
	PriceGT(u64, u64),  //insert_time,  price   //todo,price use float
	PriceLT(u64, u64),  //insert_time,  price   //todo,price use float
}

#[derive(Encode, Decode, Eq, PartialEq, Clone, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub enum Action {
	MailWithToken(Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>), //url, encrypted access_token by asymmetric encryption, revicer, title, body
}

#[frame_support::pallet]
pub mod pallet {
	use crate::Action;
	use crate::Triger;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
	pub type Something<T> = StorageValue<_, u32>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(u32, T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// create_trigerid
		#[pallet::weight(0)]
		pub fn create_triger(origin: OriginFor<T>, triger: Triger) -> DispatchResult {
			Ok(())
		}

		/// create_action
		#[pallet::weight(0)]
		pub fn create_action(origin: OriginFor<T>, action: Action) -> DispatchResult {
			Ok(())
		}

		/// test
		#[pallet::weight(0)]
		pub fn create_recipe(
			origin: OriginFor<T>,
			triger_id: u64,
			action_id: u64,
		) -> DispatchResult {
			Ok(())
		}

		/// test
		#[pallet::weight(0)]
		pub fn del_recipe(origin: OriginFor<T>, recipe_id: u64) -> DispatchResult {
			Ok(())
		}

		/// test
		#[pallet::weight(0)]
		pub fn turn_on_recipe(origin: OriginFor<T>, recipe_id: u64) -> DispatchResult {
			Ok(())
		}

		/// test
		#[pallet::weight(0)]
		pub fn turn_off_recipe(origin: OriginFor<T>, recipe_id: u64) -> DispatchResult {
			Ok(())
		}
	}
}
