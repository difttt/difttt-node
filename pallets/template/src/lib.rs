#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{traits::ConstU32, BoundedVec};
/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;
use sp_std::cmp::{Eq, PartialEq};

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

#[derive(Encode, Decode, Eq, PartialEq, Clone, RuntimeDebug, TypeInfo, MaxEncodedLen)]
//#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
//#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub enum Action {
	MailWithToken(
		BoundedVec<u8, ConstU32<128>>,
		BoundedVec<u8, ConstU32<256>>,
		BoundedVec<u8, ConstU32<128>>,
		BoundedVec<u8, ConstU32<128>>,
		BoundedVec<u8, ConstU32<256>>,
	),
	/* url, encrypted access_token
	 * by asymmetric encryption,
	 * revicer, title, body */
	Oracle(BoundedVec<u8, ConstU32<32>>, BoundedVec<u8, ConstU32<128>>), // TokenName, SourceURL
}

#[derive(Encode, Decode, Eq, PartialEq, Clone, RuntimeDebug, TypeInfo, MaxEncodedLen)]
// #[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
// #[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct Recipe {
	triger_id: u64,
	action_id: u64,
	enable: bool,
}

#[frame_support::pallet]
pub mod pallet {
	use crate::{Action, Recipe, Triger};
	use frame_support::{ensure, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use sp_runtime::traits::{One, Saturating};

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
	#[pallet::getter(fn triger_owner)]
	pub type TrigerOwner<T: Config> =
		StorageDoubleMap<_, Twox64Concat, T::AccountId, Twox64Concat, u64, (), OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn action_owner)]
	pub type ActionOwner<T: Config> =
		StorageDoubleMap<_, Twox64Concat, T::AccountId, Twox64Concat, u64, (), OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn recipe_owner)]
	pub type RecipeOwner<T: Config> =
		StorageDoubleMap<_, Twox64Concat, T::AccountId, Twox64Concat, u64, (), OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn map_triger)]
	pub(super) type MapTriger<T: Config> = StorageMap<_, Twox64Concat, u64, Triger>;

	#[pallet::storage]
	#[pallet::getter(fn map_action)]
	pub(super) type MapAction<T: Config> = StorageMap<_, Twox64Concat, u64, Action>;

	#[pallet::storage]
	#[pallet::getter(fn map_recipe)]
	pub(super) type MapRecipe<T: Config> = StorageMap<_, Twox64Concat, u64, Recipe>;

	#[pallet::storage]
	#[pallet::getter(fn next_triger_id)]
	pub type NextTrigerId<T: Config> = StorageValue<_, u64>;
	#[pallet::storage]
	#[pallet::getter(fn next_action_id)]
	pub type NextActionId<T: Config> = StorageValue<_, u64>;
	#[pallet::storage]
	#[pallet::getter(fn next_recipe_id)]
	pub type NextRecipeId<T: Config> = StorageValue<_, u64>;

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

		TrigerIdNotExist,
		ActionIdNotExist,
		RecipeIdNotExist,
		NotOwner,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// create_trigerid
		#[pallet::weight(0)]
		pub fn create_triger(origin: OriginFor<T>, triger: Triger) -> DispatchResult {
			let user = ensure_signed(origin)?;
			let triger_id = NextTrigerId::<T>::get().unwrap_or_default();

			MapTriger::<T>::insert(triger_id, triger);
			TrigerOwner::<T>::insert(user, triger_id, ());
			NextTrigerId::<T>::put(triger_id.saturating_add(One::one()));

			Ok(())
		}

		/// create_action
		#[pallet::weight(0)]
		pub fn create_action(origin: OriginFor<T>, action: Action) -> DispatchResult {
			let user = ensure_signed(origin)?;
			let action_id = NextActionId::<T>::get().unwrap_or_default();

			MapAction::<T>::insert(action_id, action);
			ActionOwner::<T>::insert(user, action_id, ());
			NextActionId::<T>::put(action_id.saturating_add(One::one()));

			Ok(())
		}

		/// test
		#[pallet::weight(0)]
		pub fn create_recipe(
			origin: OriginFor<T>,
			triger_id: u64,
			action_id: u64,
		) -> DispatchResult {
			let user = ensure_signed(origin)?;
			let recipe_id = NextRecipeId::<T>::get().unwrap_or_default();

			ensure!(MapTriger::<T>::contains_key(&triger_id), Error::<T>::TrigerIdNotExist);
			ensure!(MapAction::<T>::contains_key(&action_id), Error::<T>::ActionIdNotExist);

			MapRecipe::<T>::insert(recipe_id, Recipe { triger_id, action_id, enable: true });
			RecipeOwner::<T>::insert(user, recipe_id, ());
			NextRecipeId::<T>::put(recipe_id.saturating_add(One::one()));

			Ok(())
		}

		/// test
		#[pallet::weight(0)]
		pub fn del_recipe(origin: OriginFor<T>, recipe_id: u64) -> DispatchResult {
			let user = ensure_signed(origin)?;

			ensure!(MapRecipe::<T>::contains_key(&recipe_id), Error::<T>::RecipeIdNotExist);
			ensure!(RecipeOwner::<T>::contains_key(&user, &recipe_id), Error::<T>::NotOwner);

			RecipeOwner::<T>::remove(user, recipe_id);
			MapRecipe::<T>::remove(recipe_id);

			Ok(())
		}

		/// test
		#[pallet::weight(0)]
		pub fn turn_on_recipe(origin: OriginFor<T>, recipe_id: u64) -> DispatchResult {
			let user = ensure_signed(origin)?;

			ensure!(MapRecipe::<T>::contains_key(&recipe_id), Error::<T>::RecipeIdNotExist);
			ensure!(RecipeOwner::<T>::contains_key(&user, &recipe_id), Error::<T>::NotOwner);

			MapRecipe::<T>::try_mutate(recipe_id, |recipe| -> DispatchResult {
				if let Some(recipe) = recipe {
					recipe.enable = true;
				}
				Ok(())
			})?;

			Ok(())
		}

		/// test
		#[pallet::weight(0)]
		pub fn turn_off_recipe(origin: OriginFor<T>, recipe_id: u64) -> DispatchResult {
			let user = ensure_signed(origin)?;

			ensure!(MapRecipe::<T>::contains_key(&recipe_id), Error::<T>::RecipeIdNotExist);
			ensure!(RecipeOwner::<T>::contains_key(user, &recipe_id), Error::<T>::NotOwner);

			MapRecipe::<T>::try_mutate(recipe_id, |recipe| -> DispatchResult {
				if let Some(recipe) = recipe {
					recipe.enable = false;
				}
				Ok(())
			})?;

			Ok(())
		}
	}
}
