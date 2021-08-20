#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use sp_std::vec::Vec;
	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		#[pallet::constant]
		type MaxLen: Get<u8>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://substrate.dev/docs/en/knowledgebase/runtime/storage

	
	// Learn more about declaring storage items:
	// https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
    #[pallet::storage] 
    pub(super) type Proofs<T: Config> = StorageMap<_, Blake2_128Concat, Vec<u8>, (T::AccountId, T::BlockNumber), ValueQuery>;   


	// Pallets use events to inform users when important changes are made.
	// https://substrate.dev/docs/en/knowledgebase/runtime/events
	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		ClaimCreated(T::AccountId,Vec<u8>),
		ClaimRevoked(T::AccountId,Vec<u8>),
		SwapRevoked(T::AccountId,T::AccountId)

	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		ProofAlreadyExist,
		ClaimNotExist,
		NotClaimOwner,
		ErrorMaxLen,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T:Config> Pallet<T> {

		#[pallet::weight(0)]
		pub fn create_claim(origin:OriginFor<T>,claim:Vec<u8>)->DispatchResultWithPostInfo{
			let sender = ensure_signed(origin)?;

			ensure!(!Proofs::<T>::contains_key(&claim),Error::<T>::ProofAlreadyExist);
			ensure!((claim.len() as u8) <= T::MaxLen::get(),Error::<T>::ErrorMaxLen);

			let current_block = <frame_system::Module<T>>::block_number();

			Proofs::<T>::insert(&claim,(sender.clone(),frame_system::Pallet::<T>::block_number()));
			Self::deposit_event(Event::ClaimCreated(sender,claim));
			Ok(().into())
		}

		#[pallet::weight(0)]
		pub fn revoke_claim(origin: OriginFor<T>,claim: Vec<u8>) -> DispatchResultWithPostInfo{
			let sender = ensure_signed(origin)?;
			let (owner,_) = Proofs::<T>::get(&claim);

			ensure!(owner == sender,Error::<T>::NotClaimOwner);

			Proofs::<T>::remove(&claim);
			Self::deposit_event(Event::ClaimRevoked(sender,claim));
			Ok(().into())
		}

		#[pallet::weight(0)]
		pub fn transfer_claim(origin: OriginFor<T>,claim: Vec<u8>,dest: T::AccountId) -> DispatchResultWithPostInfo{
			let sender = ensure_signed(origin)?;
			let (owner,_block_number) = Proofs::<T>::get(&claim);

			ensure!(owner==sender,Error::<T>::NotClaimOwner);
			Proofs::<T>::insert(&claim,(dest.clone(),frame_system::Pallet::<T>::block_number()));
			Self::deposit_event(Event::SwapRevoked(sender,dest));
			Ok(().into())
		}
	}
}