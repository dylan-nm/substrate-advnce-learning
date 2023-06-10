use super::v0;
use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{pallet_prelude::*, traits::StorageVersion};
use scale_info::TypeInfo;

pub(crate) const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

pub type KittyId = v0::KittyId;
pub type KittyDna = v0::KittyDna;
pub type KittyName = [u8; 4];

#[derive(Clone, PartialEq, Eq, Default, TypeInfo, Encode, Decode, MaxEncodedLen, RuntimeDebug)]
pub struct Kitty {
	pub name: KittyName,
	pub dna: KittyDna,
}
