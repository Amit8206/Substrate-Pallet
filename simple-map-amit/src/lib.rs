#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unused_unit)]
//! A pallet to demonstrate usage of a simple storage map
//!
//! Storage maps map a key type to a value type. The hasher used to hash the key can be customized.
//! This pallet uses the `blake2_128_concat` hasher. This is a good default hasher.

pub use pallet::*;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching event type.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(PhantomData<T>);

	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A user has set their entry By Amit
		EntrySet(T::AccountId, u32),

		/// A user has read their entry, leaving it in storage By Amit
		EntryGot(T::AccountId, u32),

		/// A user has read their entry, removing it from storage By Amit
		EntryTaken(T::AccountId, u32),

		/// A user has read their entry, incremented it, and written the new entry to storage By Amit
		/// Parameters are (user, old_entry, new_entry) By Amit
		EntryIncreased(T::AccountId, u32, u32),
	}

	#[pallet::storage]
	#[pallet::getter(fn simple_map)]
	pub(super) type SimpleMap<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, u32, ValueQuery>;

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::error]
	pub enum Error<T> {
		/// The requested user has not stored a value yet By Amit
		NoValueStoredInThisAccount,

		/// The value cannot be incremented further because it has reached the maximum allowed value By Amit
		MaxValueReached,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Set the value stored at a particular key By Amit
		#[pallet::weight(10_000)]
		pub fn set_user_input(origin: OriginFor<T>, entry: u32) -> DispatchResultWithPostInfo {
			// A user can only set their own entry By Amit
			let user = ensure_signed(origin)?;

			<SimpleMap<T>>::insert(&user, entry);

			Self::deposit_event(Event::EntrySet(user, entry));
			Ok(().into())
		}

		/// Read the value stored at a particular key and emit it in an event By Amit
		#[pallet::weight(10_000)]
		pub fn get_user_input(
			origin: OriginFor<T>,
			account: T::AccountId,
		) -> DispatchResultWithPostInfo {
			// Any user can get any other user's entry By Amit
			let getter = ensure_signed(origin)?;

			ensure!(
				<SimpleMap<T>>::contains_key(&account),
				Error::<T>::NoValueStoredInThisAccount
			);
			let entry = <SimpleMap<T>>::get(account);
			Self::deposit_event(Event::EntryGot(getter, entry));
			Ok(().into())
		}

		/// Read the value stored at a particular key, while removing it from the map. By Amit
		/// Also emit the read value in an event By Amit
		#[pallet::weight(10_000)]
		pub fn read_and_delete_user_input(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			// A user can only take (delete) their own entry By Amit
			let user = ensure_signed(origin)?;

			ensure!(
				<SimpleMap<T>>::contains_key(&user),
				Error::<T>::NoValueStoredInThisAccount
			);
			let entry = <SimpleMap<T>>::take(&user);
			Self::deposit_event(Event::EntryTaken(user, entry));
			Ok(().into())
		}

		/// Increase the value associated with a particular key By Amit
		#[pallet::weight(10_000)]
		pub fn increase_user_input(
			origin: OriginFor<T>,
			add_this_val: u32,
		) -> DispatchResultWithPostInfo {
			// A user can only mutate their own entry By Amit
			let user = ensure_signed(origin)?;

			ensure!(
				<SimpleMap<T>>::contains_key(&user),
				Error::<T>::NoValueStoredInThisAccount
			);
			let original_value = <SimpleMap<T>>::get(&user);

			let new_value = original_value
				.checked_add(add_this_val)
				.ok_or(Error::<T>::MaxValueReached)?;
			<SimpleMap<T>>::insert(&user, new_value);

			Self::deposit_event(Event::EntryIncreased(user, original_value, new_value));

			Ok(().into())
		}
	}
}
