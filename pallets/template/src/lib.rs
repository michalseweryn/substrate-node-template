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
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use frame_support::inherent::IsFatalError;
	use frame_support::sp_std::result;
	use sp_std::vec::Vec;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		NextLine(Vec<u8>),
	}

	#[pallet::error]
	pub enum Error<T> {
		Error,
	}

	#[derive(Encode, Decode)]
	pub struct InherentError;

	impl InherentError {
		pub fn try_from(id: &InherentIdentifier, data: &[u8]) -> Option<Self> {
			if id == &INHERENT_IDENTIFIER {
				<InherentError as codec::Decode>::decode(&mut &data[..]).ok()
			} else {
				None
			}
		}
	}

	impl IsFatalError for InherentError {
		fn is_fatal_error(&self) -> bool {
			false
		}
	}

	pub const INHERENT_IDENTIFIER: [u8; 8] = *b"jedrzejs";
	
	#[pallet::inherent]
	impl<T:Config> ProvideInherent for Pallet<T> {
		type Call = Call<T>;
		type Error = InherentError;
	
		const INHERENT_IDENTIFIER: InherentIdentifier = INHERENT_IDENTIFIER;
	
		fn create_inherent(data: &InherentData) -> Option<Self::Call> {
			let inherent_data =
            data.get_data::<Vec<u8>>(&INHERENT_IDENTIFIER).unwrap().unwrap();
			Some(Call::show_next_line(inherent_data))
		}

		fn check_inherent(_call: &Self::Call, _data: &InherentData) -> result::Result<(), Self::Error> {
			Ok(())
		}

		fn is_inherent(call: &Self::Call) -> bool {
			matches!(call, Call::show_next_line(_))
		}
	}

	#[pallet::call]	
	impl<T:Config> Pallet<T> {
		#[pallet::weight(10_000)]
		pub fn show_next_line(_origin: OriginFor<T>, value: Vec<u8>) -> DispatchResult {
			Self::deposit_event(Event::NextLine(value));
			Ok(())
		}
	}
}
