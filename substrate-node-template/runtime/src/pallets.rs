use crate::{
	Balance, Balances, InsecureRandomnessCollectiveFlip, Runtime, RuntimeEvent, EXISTENTIAL_DEPOSIT,
};
use frame_support::{parameter_types, PalletId};
use sp_core::ConstU32;

// pallet-insecure-randomness-collective-flip
impl pallet_insecure_randomness_collective_flip::Config for Runtime {}

// pallet-template-2
impl pallet_template_2::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
}

// pallet-poe
impl pallet_poe::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type MaxClaimLength = ConstU32<512>;
}

// pallet-kitties
parameter_types! {
	pub KittiesPalletId: PalletId = PalletId(*b"py/kitty");
	pub KittyPrice: Balance = EXISTENTIAL_DEPOSIT * 1000;
}
impl pallet_kitties::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type PalletId = KittiesPalletId;
	type Currency = Balances;
	type KittyDnaRandomness = InsecureRandomnessCollectiveFlip;
	type KittyPrice = KittyPrice;
}

// pallet-ocw-tutorials
// impl pallet_ocw_tutorials::Config for Runtime {
// 	type RuntimeEvent = RuntimeEvent;
// }

// pallet-ocw-homework
impl pallet_ocw_homework::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type AppCrypto = crate::offchain::app_crypto::AppCryptoSr25519;
}
