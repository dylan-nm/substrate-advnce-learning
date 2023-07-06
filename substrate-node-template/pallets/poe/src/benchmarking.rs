use crate::*;
use frame_benchmarking::{account, benchmarks, vec, whitelisted_caller};
use frame_support::pallet_prelude::*;
use frame_system::RawOrigin;

benchmarks! {
	create_claim {
		let caller: T::AccountId = whitelisted_caller();
		let l in 0 .. T::MaxClaimLength::get();
		let claim = BoundedVec::try_from(vec![0; l as usize]).unwrap();
	}: _(RawOrigin::Signed(caller), claim)

	revoke_claim {
		let caller: T::AccountId = whitelisted_caller();
		let l in 0 .. T::MaxClaimLength::get();
		let claim = BoundedVec::try_from(vec![0; l as usize]).unwrap();
		let _ = Pallet::<T>::create_claim(RawOrigin::Signed(caller.clone()).into(), claim.clone());
	}: _(RawOrigin::Signed(caller), claim)

	transfer_claim {
		let caller: T::AccountId = whitelisted_caller();
		let l in 0 .. T::MaxClaimLength::get();
		let claim = BoundedVec::try_from(vec![0; l as usize]).unwrap();
		let _ = Pallet::<T>::create_claim(RawOrigin::Signed(caller.clone()).into(), claim.clone());
		let recipient: T::AccountId = account("recipient", 0, 0);
	}: _(RawOrigin::Signed(caller), recipient, claim)

	impl_benchmark_test_suite!(PalletPoe, crate::mock::new_test_ext(), crate::mock::Test);
}
