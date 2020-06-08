#![cfg_attr(not(feature = "std"), no_std)]

/// A FRAME pallet template with necessary imports

/// Feel free to remove or edit this file as needed.
/// If you change the name of this file, make sure to update its references in runtime/src/lib.rs
/// If you remove this file, you can remove those references

/// For more guidance on Substrate FRAME, see the example pallet
/// https://github.com/paritytech/substrate/blob/master/frame/example/src/lib.rs

use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch,ensure,traits::{Get},};
use frame_system::{self as system, ensure_signed};
use sp_std::prelude::*;
use sp_runtime::traits::StaticLookup;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// The pallet's configuration trait.
pub trait Trait: system::Trait {
	// Add other types and constants required to configure this pallet.

	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
	type MaxClaimLength: Get<u32>;
}

// This pallet's storage items.
decl_storage! {
	// It is important to update your storage name so that your pallet's
	// storage items are isolated from other pallets.
	// ---------------------------------vvvvvvvvvvvvvv
	trait Store for Module<T: Trait> as TemplateModule {
		Proofs get(fn proofs): map hasher(blake2_128_concat) Vec<u8> =>(T::AccountId,T::BlockNumber);
	}
}

// The pallet's events
decl_event!(
	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
		/// Just a dummy event.
		/// Event `Something` is declared with a parameter of the type `u32` and `AccountId`
		/// To emit this event, we call the deposit function, from our runtime functions
		ClaimCreated(AccountId, Vec<u8>),
		ClaimRevoked(AccountId, Vec<u8>),
		ClaimTransfered(AccountId, Vec<u8>, AccountId),
	}
);

// The pallet's errors
decl_error! {
	pub enum Error for Module<T: Trait> {
		ProofAlreadyExist,
		ClaimNotExist,
		NotClaimOwner,
		ProofTooLong,
	}
}

// The pallet's dispatchable functions.
decl_module! {
	/// The module declaration.
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Initializing errors
		// this includes information about your errors in the node's metadata.
		// it is needed only if you are using errors in your pallet
		type Error = Error<T>;

		// Initializing events
		// this is needed only if you are using events in your pallet
		fn deposit_event() = default;

		/// create_claim
		#[weight = 0]
		pub fn create_claim(origin, claim:Vec<u8>) -> dispatch::DispatchResult {
			// Check it was signed and get the signer. See also: ensure_root and ensure_none
			let sender = ensure_signed(origin)?;

			ensure!(!Proofs::<T>::contains_key(&claim),Error::<T>::ProofAlreadyExist);
			ensure!(T::MaxClaimLength::get() >= claim.len() as u32, Error::<T>::ProofTooLong);

			Proofs::<T>::insert(&claim,(sender.clone(),system::Module::<T>::block_number()));
			// Here we are raising the Something event
			Self::deposit_event(RawEvent::ClaimCreated(sender, claim));
			Ok(())
		}

		/// revoke_claim
		#[weight = 0]
		pub fn revoke_claim(origin, claim:Vec<u8>) -> dispatch::DispatchResult {
			// Check it was signed and get the signer. See also: ensure_root and ensure_none
			let sender = ensure_signed(origin)?;
			ensure!(Proofs::<T>::contains_key(&claim),Error::<T>::ClaimNotExist);
		
			let (owner,_block_number) = Proofs::<T>::get(&claim);
			ensure!(owner == sender,Error::<T>::NotClaimOwner);
		
			Proofs::<T>::remove(&claim);

			Self::deposit_event(RawEvent::ClaimRevoked(sender, claim));
			Ok(())
		}

		///transfer_claim
		#[weight = 0]
		pub fn transfer_claim(origin, claim:Vec<u8>,dest: <T::Lookup as StaticLookup>::Source) -> dispatch::DispatchResult {
			// Check it was signed and get the signer. See also: ensure_root and ensure_none
			let sender = ensure_signed(origin)?;
			ensure!(Proofs::<T>::contains_key(&claim),Error::<T>::ClaimNotExist);
		
			let (owner,_block_number) = Proofs::<T>::get(&claim);
			ensure!(owner == sender,Error::<T>::NotClaimOwner);
		
			let dest = T::Lookup::lookup(dest)?;

            Proofs::<T>::insert(&claim, (&dest, system::Module::<T>::block_number()));
			Self::deposit_event(RawEvent::ClaimTransfered(sender, claim, dest));
			Ok(())
		}
	}
}
