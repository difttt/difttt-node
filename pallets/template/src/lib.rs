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
	times: u64,
	done: bool,
}

#[frame_support::pallet]
pub mod pallet {
	use crate::{Action, Recipe, Triger};
	use frame_support::{ensure, pallet_prelude::*, traits::UnixTime};
	use frame_system::pallet_prelude::*;
	use sp_runtime::{
		offchain::{
			http,
			storage::StorageValueRef,
			storage_lock::{BlockAndTime, StorageLock},
			Duration,
		},
		traits::{BlockNumberProvider, One},
	};
	use sp_std::{collections::btree_map::BTreeMap, prelude::*};

	const FETCH_TIMEOUT_PERIOD: u64 = 3000; // in milli-seconds
	const LOCK_TIMEOUT_EXPIRATION: u64 = FETCH_TIMEOUT_PERIOD + 1000; // in milli-seconds
	const LOCK_BLOCK_EXPIRATION: u32 = 3; // in block number

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type TimeProvider: UnixTime;
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
		/// Event documentation should end with an array that provides descriptive names for
		/// event parameters. [something, who]
		SomethingStored(u32, T::AccountId),

		TrigerCreated(u64, Triger),
		ActionCreated(u64, Action),
		RecipeCreated(u64, Recipe),
		RecipeRemoved(u64),
		RecipeTurnOned(u64),
		RecipeTurnOffed(u64),
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
			//get trigger_id for testing
			log::info!("trigger_id: {}", triger_id);

			MapTriger::<T>::insert(triger_id, triger.clone());
			TrigerOwner::<T>::insert(user, triger_id, ());
			NextTrigerId::<T>::put(triger_id.saturating_add(One::one()));

			Self::deposit_event(Event::TrigerCreated(triger_id, triger));

			Ok(())
		}

		/// create_action
		#[pallet::weight(0)]
		pub fn create_action(origin: OriginFor<T>, action: Action) -> DispatchResult {
			let user = ensure_signed(origin)?;
			let action_id = NextActionId::<T>::get().unwrap_or_default();
			MapAction::<T>::insert(action_id, action.clone());
			ActionOwner::<T>::insert(user, action_id, ());
			NextActionId::<T>::put(action_id.saturating_add(One::one()));

			Self::deposit_event(Event::ActionCreated(action_id, action));

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

			let recipe = Recipe { triger_id, action_id, enable: true, times: 0, done: false };

			MapRecipe::<T>::insert(recipe_id, recipe.clone());
			RecipeOwner::<T>::insert(user, recipe_id, ());
			NextRecipeId::<T>::put(recipe_id.saturating_add(One::one()));

			Self::deposit_event(Event::RecipeCreated(recipe_id, recipe));

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

			Self::deposit_event(Event::RecipeRemoved(recipe_id));

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
					Self::deposit_event(Event::RecipeTurnOned(recipe_id));
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
					Self::deposit_event(Event::RecipeTurnOffed(recipe_id));
				}
				Ok(())
			})?;

			Ok(())
		}
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn offchain_worker(_block_number: T::BlockNumber) {
			log::info!("###### Hello from pallet-template-offchain-worker.");

			// let parent_hash = <frame_system::Pallet<T>>::block_hash(block_number -
			// 1u32.into()); log::info!("###### Current block: {:?} (parent hash: {:?})",
			// block_number, parent_hash);

			let timestamp_now = T::TimeProvider::now();
			log::info!("###### Current time: {:?} ", timestamp_now.as_secs());

			let store_hashmap_recipe = StorageValueRef::local(b"template_ocw::recipe_task");

			let mut map_recipe_task: BTreeMap<u64, Recipe>;
			if let Ok(Some(info)) = store_hashmap_recipe.get::<BTreeMap<u64, Recipe>>() {
				map_recipe_task = info;
			} else {
				map_recipe_task = BTreeMap::new();
			}

			let mut lock = StorageLock::<BlockAndTime<Self>>::with_block_and_time_deadline(
				b"offchain-demo::lock",
				LOCK_BLOCK_EXPIRATION,
				Duration::from_millis(LOCK_TIMEOUT_EXPIRATION),
			);

			let mut map_running_action_recipe_task: BTreeMap<u64, Recipe> = BTreeMap::new();
			if let Ok(_guard) = lock.try_lock() {
				for (recipe_id, recipe) in MapRecipe::<T>::iter() {
					if recipe.enable && !recipe.done {
						if !map_recipe_task.contains_key(&recipe_id) {
							map_recipe_task.insert(
								recipe_id,
								Recipe {
									triger_id: recipe.triger_id,
									action_id: recipe.action_id,
									enable: true,
									times: 0,
									done: false,
								},
							);
						}
					} else {
						map_recipe_task.remove(&recipe_id);
					};
				}

				for (recipe_id, recipe) in map_recipe_task.iter_mut() {
					let triger = MapTriger::<T>::get(recipe.triger_id);

					match triger {
						Some(Triger::Timer(insert_time, timer_seconds)) => {
							if insert_time + recipe.times * timer_seconds >
								timestamp_now.as_secs()
							{
								(*recipe).times += 1;
								log::info!(
									"###### Current Triger times: {:?} ",
									recipe.times
								);

								map_running_action_recipe_task
									.insert(*recipe_id, recipe.clone());
							}
						},
						Some(Triger::Schedule(_, timestamp)) => {
							if timestamp > timestamp_now.as_secs() {
								(*recipe).times += 1;
								(*recipe).done = true;

								map_running_action_recipe_task
									.insert(*recipe_id, recipe.clone());
							}
						},
						Some(Triger::PriceGT(_, price)) => {
							//todo(check price gt)
							(*recipe).times += 1;
							(*recipe).done = true;

							map_running_action_recipe_task
								.insert(*recipe_id, recipe.clone());
						},
						Some(Triger::PriceLT(_, price)) => {
							//todo(check price gt)
							(*recipe).times += 1;
							(*recipe).done = true;
							map_running_action_recipe_task
								.insert(*recipe_id, recipe.clone());
						},
						_ => {},
					}
				}

				store_hashmap_recipe.set(&map_recipe_task);
			};

			//todo run action
			for (recipe_id, recipe) in map_running_action_recipe_task.iter() {
				let action = MapAction::<T>::get(recipe.action_id);
				match action {
					Some(Action::MailWithToken(url, token, revicer, title, body)) => {
						//todo(publish email task)
					},

					Some(Action::Oracle(token_name, source_url)) => {
						//todo(publish oracle task)
					},
					_ => {},
				}
			}
		}
	}

	impl<T: Config> Pallet<T> {}

	impl<T: Config> BlockNumberProvider for Pallet<T> {
		type BlockNumber = T::BlockNumber;

		fn current_block_number() -> Self::BlockNumber {
			<frame_system::Pallet<T>>::block_number()
		}
	}
}
