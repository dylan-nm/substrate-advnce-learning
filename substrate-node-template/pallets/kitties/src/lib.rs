#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

mod storage;

#[frame_support::pallet]
mod pallet {
	pub use crate::storage::current_version::*;

	use crate::storage::upgrade_storage;
	use frame_support::{
		pallet_prelude::*,
		traits::{Currency, ExistenceRequirement, Randomness},
		PalletId,
	};
	use frame_system::pallet_prelude::*;
	use sp_io::hashing::blake2_128;
	use sp_runtime::traits::AccountIdConversion;

	////////////////////////////////////////////////////////////////////////////////////////////////////
	/// config

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type PalletId: Get<PalletId>;
		type Currency: Currency<Self::AccountId>;
		type KittyDnaRandomness: Randomness<Self::Hash, Self::BlockNumber>;
		#[pallet::constant]
		type KittyPrice: Get<Balance<Self>>;
	}

	type Balance<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	////////////////////////////////////////////////////////////////////////////////////////////////////
	/// storage

	#[pallet::storage]
	#[pallet::getter(fn next_kitty_id)]
	pub type NextKittyId<T: Config> = StorageValue<_, KittyId, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn kitties)]
	pub type Kitties<T: Config> = StorageMap<_, Blake2_128Concat, KittyId, Kitty>;

	#[pallet::storage]
	#[pallet::getter(fn kitty_owner)]
	pub type KittyOwner<T: Config> = StorageMap<_, Blake2_128Concat, KittyId, T::AccountId>;

	#[pallet::storage]
	#[pallet::getter(fn kitty_parents)]
	pub type KittyParents<T: Config> = StorageMap<_, Blake2_128Concat, KittyId, (KittyId, KittyId)>;

	#[pallet::storage]
	#[pallet::getter(fn kitty_on_sale)]
	pub type KittyOnSale<T: Config> = StorageMap<_, Blake2_128Concat, KittyId, ()>;

	////////////////////////////////////////////////////////////////////////////////////////////////////
	/// event & error

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		KittyCreated { account: T::AccountId, kitty_id: KittyId, kitty: Kitty },
		KittyBred { account: T::AccountId, kitty_id: KittyId, kitty: Kitty },
		KittyTransferred { sender: T::AccountId, recipient: T::AccountId, kitty_id: KittyId },
		KittyOnSale { account: T::AccountId, kitty_id: KittyId },
		KittyBought { buyer: T::AccountId, kitty_id: KittyId },
	}

	#[pallet::error]
	pub enum Error<T> {
		KittyIdOverflow,
		SameParentKittyId,
		KittyNotExist,
		NotKittyOwner,
		TransferKittyToOwner,
		KittyAlreadyOnSale,
		KittyNotOnSale,
		KittyAlreadyOwned,
	}

	////////////////////////////////////////////////////////////////////////////////////////////////////
	/// pallet

	#[pallet::pallet]
	#[pallet::storage_version(STORAGE_VERSION)]
	pub struct Pallet<T>(_);

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_runtime_upgrade() -> Weight {
			upgrade_storage::<T>()
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(0)]
		pub fn create_kitty(origin: OriginFor<T>, name: KittyName) -> DispatchResult {
			let signer = ensure_signed(origin)?;
			let kitty_id = Self::generate_next_kitty_id()?;
			let kitty = Kitty { name, dna: Self::random_kitty_dna(&signer) };
			let price = T::KittyPrice::get();

			T::Currency::transfer(
				&signer,
				&Self::get_pallet_account_id(),
				price,
				ExistenceRequirement::KeepAlive,
			)?;
			Kitties::<T>::insert(kitty_id, &kitty);
			KittyOwner::<T>::insert(kitty_id, &signer);
			Self::deposit_event(Event::KittyCreated { account: signer, kitty_id, kitty });
			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(0)]
		pub fn bred_kitty(
			origin: OriginFor<T>,
			parent_id_1: KittyId,
			parent_id_2: KittyId,
			name: KittyName,
		) -> DispatchResult {
			let signer = ensure_signed(origin)?;
			ensure!(parent_id_1 != parent_id_2, Error::<T>::SameParentKittyId);
			let parent_1 = Self::kitties(parent_id_1).ok_or(Error::<T>::KittyNotExist)?;
			let parent_2 = Self::kitties(parent_id_2).ok_or(Error::<T>::KittyNotExist)?;

			let kitty_id = Self::generate_next_kitty_id()?;
			let kitty = Kitty { name, dna: Self::child_kitty_dna(&signer, &parent_1, &parent_2) };
			let price = T::KittyPrice::get();

			T::Currency::transfer(
				&signer,
				&Self::get_pallet_account_id(),
				price,
				ExistenceRequirement::KeepAlive,
			)?;
			Kitties::<T>::insert(kitty_id, &kitty);
			KittyOwner::<T>::insert(kitty_id, &signer);
			KittyParents::<T>::insert(kitty_id, (parent_id_1, parent_id_2));
			Self::deposit_event(Event::KittyBred { account: signer, kitty_id, kitty });
			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(0)]
		pub fn transfer_kitty(
			origin: OriginFor<T>,
			recipient: T::AccountId,
			kitty_id: KittyId,
		) -> DispatchResult {
			let signer = ensure_signed(origin)?;
			let owner = Self::kitty_owner(kitty_id).ok_or(Error::<T>::KittyNotExist)?;
			ensure!(signer == owner, Error::<T>::NotKittyOwner);
			ensure!(signer != recipient, Error::<T>::TransferKittyToOwner);

			KittyOwner::<T>::insert(kitty_id, &recipient);
			Self::deposit_event(Event::KittyTransferred { sender: signer, recipient, kitty_id });
			Ok(())
		}

		#[pallet::call_index(3)]
		#[pallet::weight(0)]
		pub fn sale_kitty(origin: OriginFor<T>, kitty_id: KittyId) -> DispatchResult {
			let signer = ensure_signed(origin)?;
			let owner = Self::kitty_owner(kitty_id).ok_or(Error::<T>::KittyNotExist)?;
			ensure!(signer == owner, Error::<T>::NotKittyOwner);
			ensure!(Self::kitty_on_sale(kitty_id).is_none(), Error::<T>::KittyAlreadyOnSale);

			KittyOnSale::<T>::insert(kitty_id, ());
			Self::deposit_event(Event::KittyOnSale { account: signer, kitty_id });
			Ok(())
		}

		#[pallet::call_index(4)]
		#[pallet::weight(0)]
		pub fn buy_kitty(origin: OriginFor<T>, kitty_id: u32) -> DispatchResult {
			let signer = ensure_signed(origin)?;
			let owner = Self::kitty_owner(kitty_id).ok_or(Error::<T>::KittyNotExist)?;
			ensure!(signer != owner, Error::<T>::KittyAlreadyOwned);
			ensure!(Self::kitty_on_sale(kitty_id).is_some(), Error::<T>::KittyNotOnSale);

			let price = T::KittyPrice::get();

			T::Currency::transfer(&signer, &owner, price, ExistenceRequirement::KeepAlive)?;
			KittyOwner::<T>::insert(kitty_id, &signer);
			KittyOnSale::<T>::remove(kitty_id);
			Self::deposit_event(Event::KittyBought { buyer: signer, kitty_id });
			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		fn get_pallet_account_id() -> T::AccountId {
			T::PalletId::get().into_account_truncating()
		}

		fn generate_next_kitty_id() -> Result<KittyId, DispatchError> {
			NextKittyId::<T>::try_mutate(|next_kitty_id| -> Result<KittyId, DispatchError> {
				let kitty_id = *next_kitty_id;
				*next_kitty_id = kitty_id.checked_add(1).ok_or(Error::<T>::KittyIdOverflow)?;
				Ok(kitty_id)
			})
		}

		pub(crate) fn random_kitty_dna(account: &T::AccountId) -> KittyDna {
			let payload = (
				T::KittyDnaRandomness::random_seed(),
				&account,
				frame_system::Pallet::<T>::extrinsic_index(),
			);
			payload.using_encoded(blake2_128)
		}

		pub(crate) fn child_kitty_dna(
			account: &T::AccountId,
			parent_1: &Kitty,
			parent_2: &Kitty,
		) -> KittyDna {
			let selector = Self::random_kitty_dna(&account);
			let mut dna = KittyDna::default();
			for i in 0..parent_1.dna.len() {
				dna[i] = (parent_1.dna[i] & selector[i]) | (parent_2.dna[i] & !selector[i])
			}
			return dna
		}
	}
}
