use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok, traits::ConstU32, BoundedVec};

const ACCOUNT_ID_1: u64 = 1;
const ACCOUNT_ID_2: u64 = 2;
const ACCOUNT_ID_3: u64 = 3;

fn new_claim() -> BoundedVec<u8, ConstU32<10>> {
	return BoundedVec::try_from(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]).unwrap()
}

#[test]
fn create_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = new_claim();
		let signer = RuntimeOrigin::signed(ACCOUNT_ID_1);

		// 创建存证
		assert_ok!(PalletPoe::create_claim(signer, claim.clone()));
		// 检查存证
		assert_eq!(PalletPoe::proofs(&claim), Some((ACCOUNT_ID_1, System::block_number())));
	})
}

#[test]
fn create_claim_failed_when_claim_exist() {
	new_test_ext().execute_with(|| {
		let claim = new_claim();
		let signer = RuntimeOrigin::signed(ACCOUNT_ID_1);

		// 创建存证
		let _ = PalletPoe::create_claim(signer.clone(), claim.clone());
		// 再次创建存证
		assert_noop!(
			PalletPoe::create_claim(signer, claim.clone()),
			Error::<Test>::ClaimAlreadyExist
		);
		// 检查存证
		assert_eq!(PalletPoe::proofs(&claim), Some((ACCOUNT_ID_1, System::block_number())));
	});
}

#[test]
fn revoke_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = new_claim();
		let signer = RuntimeOrigin::signed(ACCOUNT_ID_1);

		// 创建存证
		assert_ok!(PalletPoe::create_claim(signer.clone(), claim.clone()));
		// 撤销存证
		assert_ok!(PalletPoe::revoke_claim(signer, claim.clone()));
		// 检查存证
		assert_eq!(PalletPoe::proofs(&claim), None);
	})
}

#[test]
fn revoke_claim_failed_when_claim_not_exist() {
	new_test_ext().execute_with(|| {
		let claim = new_claim();
		let signer = RuntimeOrigin::signed(ACCOUNT_ID_1);

		// 撤销存证
		assert_noop!(PalletPoe::revoke_claim(signer, claim.clone()), Error::<Test>::ClaimNotExist);
		// 检查存证
		assert_eq!(PalletPoe::proofs(&claim), None);
	});
}

#[test]
fn revoke_claim_failed_when_not_owner() {
	new_test_ext().execute_with(|| {
		let claim = new_claim();
		let signer = RuntimeOrigin::signed(ACCOUNT_ID_1);
		let signer_2 = RuntimeOrigin::signed(ACCOUNT_ID_2);

		// 创建存证
		assert_ok!(PalletPoe::create_claim(signer, claim.clone()));
		// 撤销存证
		assert_noop!(
			PalletPoe::revoke_claim(signer_2, claim.clone()),
			Error::<Test>::NotClaimOwner
		);
		// 检查存证
		assert_eq!(PalletPoe::proofs(&claim), Some((ACCOUNT_ID_1, System::block_number())));
	})
}

#[test]
fn transfer_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = new_claim();
		let signer = RuntimeOrigin::signed(ACCOUNT_ID_1);

		// 创建存证
		assert_ok!(PalletPoe::create_claim(signer.clone(), claim.clone()));
		// 转移存证
		assert_ok!(PalletPoe::transfer_claim(signer, ACCOUNT_ID_2, claim.clone()));
		// 检查存证
		assert_eq!(PalletPoe::proofs(&claim), Some((ACCOUNT_ID_2, System::block_number())));
	})
}

#[test]
fn transfer_claim_failed_when_claim_not_exist() {
	new_test_ext().execute_with(|| {
		let claim = new_claim();
		let signer = RuntimeOrigin::signed(ACCOUNT_ID_1);

		// 转移存证
		assert_noop!(
			PalletPoe::transfer_claim(signer, ACCOUNT_ID_2, claim.clone()),
			Error::<Test>::ClaimNotExist
		);
		// 检查存证
		assert_eq!(PalletPoe::proofs(&claim), None);
	})
}

#[test]
fn transfer_claim_failed_when_not_owner() {
	new_test_ext().execute_with(|| {
		let claim = new_claim();
		let signer = RuntimeOrigin::signed(ACCOUNT_ID_1);
		let signer_2 = RuntimeOrigin::signed(ACCOUNT_ID_2);

		// 创建存证
		assert_ok!(PalletPoe::create_claim(signer, claim.clone()));
		// 转移存证
		assert_noop!(
			PalletPoe::transfer_claim(signer_2, ACCOUNT_ID_3, claim.clone()),
			Error::<Test>::NotClaimOwner
		);
		// 检查存证
		assert_eq!(PalletPoe::proofs(&claim), Some((ACCOUNT_ID_1, System::block_number())));
	})
}

#[test]
fn transfer_claim_failed_when_transfer_to_owner() {
	new_test_ext().execute_with(|| {
		let claim = new_claim();
		let signer = RuntimeOrigin::signed(ACCOUNT_ID_1);

		// 创建存证
		assert_ok!(PalletPoe::create_claim(signer.clone(), claim.clone()));
		// 转移存证
		assert_noop!(
			PalletPoe::transfer_claim(signer, ACCOUNT_ID_1, claim.clone()),
			Error::<Test>::TransferToOwner
		);
		// 检查存证
		assert_eq!(PalletPoe::proofs(&claim), Some((ACCOUNT_ID_1, System::block_number())));
	})
}
