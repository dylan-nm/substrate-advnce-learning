pub use v2 as current_version; // 当前版本
pub mod v2;

use crate::{Config, Kitties, Pallet};
use frame_support::{
	migration::storage_key_iter, pallet_prelude::*, traits::GetStorageVersion, weights::Weight,
	StoragePrefixedMap,
};
mod v0;
mod v1;

pub(crate) fn upgrade_storage<T: Config>() -> Weight {
	let on_chain_ver: StorageVersion = Pallet::<T>::on_chain_storage_version();
	if on_chain_ver == v1::STORAGE_VERSION {
		from_v1::<T>();
		current_version::STORAGE_VERSION.put::<Pallet<T>>();
	} else if on_chain_ver == v0::STORAGE_VERSION {
		from_v0::<T>();
		current_version::STORAGE_VERSION.put::<Pallet<T>>();
	}

	Weight::zero()
}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// v1 -> current

fn from_v1<T: Config>() {
	let module = Kitties::<T>::module_prefix();
	let item = Kitties::<T>::storage_prefix();

	for (kitty_id, kitty_old) in
		storage_key_iter::<v1::KittyId, v1::Kitty, Blake2_128Concat>(module, item).drain()
	{
		let kitty = current_version::Kitty {
			name: from_name_v1(&kitty_old.name, b"5678"),
			dna: kitty_old.dna,
		};
		Kitties::<T>::insert(kitty_id, &kitty);
	}
}

fn from_name_v1(name_v1: &v1::KittyName, append: &[u8; 4]) -> current_version::KittyName {
	let mut result = [0; 8];
	result[..4].copy_from_slice(name_v1);
	result[4..].copy_from_slice(append);
	result
}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// v0 -> current

fn from_v0<T: Config>() {
	let module = Kitties::<T>::module_prefix();
	let item = Kitties::<T>::storage_prefix();

	for (kitty_id, kitty_old) in
		storage_key_iter::<v0::KittyId, v0::Kitty, Blake2_128Concat>(module, item).drain()
	{
		let kitty = current_version::Kitty { name: *b"12345678", dna: kitty_old.0 };
		Kitties::<T>::insert(kitty_id, &kitty);
	}
}
