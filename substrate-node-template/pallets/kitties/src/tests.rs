use crate::{mock::*, Error, Event, Kitty, KittyId, KittyName, NextKittyId};
use frame_support::{assert_noop, assert_ok, pallet_prelude::DispatchResultWithPostInfo};

const ACCOUNT_ID_1: AccountId = 1;
const ACCOUNT_ID_2: AccountId = 2;
const KITTY_ID_0: KittyId = 0;
const KITTY_NAME: KittyName = *b"12345678";

fn init_balance(account: AccountId, new_free: Balance) -> DispatchResultWithPostInfo {
	Balances::set_balance(RuntimeOrigin::root(), account, new_free, 0)
}

#[test]
fn create_kitty() {
	new_test_ext().execute_with(|| {
		assert_ok!(init_balance(ACCOUNT_ID_1, 10_000_000));

		let signer = RuntimeOrigin::signed(ACCOUNT_ID_1);

		// 检查初始状态
		assert_eq!(PalletKitties::next_kitty_id(), KITTY_ID_0);

		// create kitty
		assert_ok!(PalletKitties::create_kitty(signer.clone(), KITTY_NAME));
		assert_eq!(PalletKitties::next_kitty_id(), KITTY_ID_0 + 1);
		assert_eq!(PalletKitties::kitty_owner(KITTY_ID_0), Some(ACCOUNT_ID_1));
		assert_eq!(PalletKitties::kitty_parents(KITTY_ID_0), None);
		let kitty = Kitty { name: KITTY_NAME, dna: PalletKitties::random_kitty_dna(&ACCOUNT_ID_1) };
		assert_eq!(PalletKitties::kitties(KITTY_ID_0), Some(kitty.clone()));
		System::assert_last_event(
			Event::KittyCreated { account: ACCOUNT_ID_1, kitty_id: KITTY_ID_0, kitty }.into(),
		);

		// KittyId 溢出
		NextKittyId::<Test>::set(KittyId::max_value());
		assert_noop!(
			PalletKitties::create_kitty(signer, KITTY_NAME),
			Error::<Test>::KittyIdOverflow
		);
	});
}

#[test]
fn bred_kitty() {
	new_test_ext().execute_with(|| {
		assert_ok!(init_balance(ACCOUNT_ID_1, 10_000_000));

		let signer = RuntimeOrigin::signed(ACCOUNT_ID_1);

		let parent_id_0 = KITTY_ID_0;
		let parent_id_1 = KITTY_ID_0 + 1;
		let child_id = KITTY_ID_0 + 2;

		// parent 相同
		assert_noop!(
			PalletKitties::bred_kitty(signer.clone(), parent_id_0, parent_id_0, KITTY_NAME),
			Error::<Test>::SameParentKittyId
		);
		// parent 不存在
		assert_noop!(
			PalletKitties::bred_kitty(signer.clone(), parent_id_0, parent_id_1, KITTY_NAME),
			Error::<Test>::KittyNotExist
		);

		// 创建两只Kitty作为parent
		assert_ok!(PalletKitties::create_kitty(signer.clone(), KITTY_NAME));
		assert_ok!(PalletKitties::create_kitty(signer.clone(), KITTY_NAME));
		assert_eq!(PalletKitties::next_kitty_id(), child_id);
		let parent_1 =
			Kitty { name: KITTY_NAME, dna: PalletKitties::random_kitty_dna(&ACCOUNT_ID_1) };
		let parent_2 =
			Kitty { name: KITTY_NAME, dna: PalletKitties::random_kitty_dna(&ACCOUNT_ID_1) };

		// bred kitty
		assert_ok!(PalletKitties::bred_kitty(signer, parent_id_0, parent_id_1, KITTY_NAME));
		assert_eq!(PalletKitties::next_kitty_id(), child_id + 1);
		assert_eq!(PalletKitties::kitty_owner(child_id), Some(ACCOUNT_ID_1));
		assert_eq!(PalletKitties::kitty_parents(child_id), Some((parent_id_0, parent_id_1)));
		let child = Kitty {
			name: KITTY_NAME,
			dna: PalletKitties::child_kitty_dna(&ACCOUNT_ID_1, &parent_1, &parent_2),
		};
		assert_eq!(PalletKitties::kitties(child_id), Some(child.clone()));
		System::assert_last_event(
			Event::KittyBred { account: ACCOUNT_ID_1, kitty_id: child_id, kitty: child }.into(),
		);
	});
}

#[test]
fn transfer_kitty() {
	new_test_ext().execute_with(|| {
		assert_ok!(init_balance(ACCOUNT_ID_1, 10_000_000));
		assert_ok!(init_balance(ACCOUNT_ID_2, 10_000_000));

		let signer = RuntimeOrigin::signed(ACCOUNT_ID_1);
		let signer_2 = RuntimeOrigin::signed(ACCOUNT_ID_2);

		// transfer 不存在的 kitty
		assert_noop!(
			PalletKitties::transfer_kitty(signer.clone(), ACCOUNT_ID_2, KITTY_ID_0),
			Error::<Test>::KittyNotExist
		);

		// create kitty
		assert_ok!(PalletKitties::create_kitty(signer.clone(), KITTY_NAME));
		assert_eq!(PalletKitties::kitty_owner(KITTY_ID_0), Some(ACCOUNT_ID_1));

		// 非ower进行transfer
		assert_noop!(
			PalletKitties::transfer_kitty(signer_2.clone(), ACCOUNT_ID_1, KITTY_ID_0),
			Error::<Test>::NotKittyOwner
		);

		// transfer 给 ower
		assert_noop!(
			PalletKitties::transfer_kitty(signer.clone(), ACCOUNT_ID_1, KITTY_ID_0),
			Error::<Test>::TransferKittyToOwner
		);

		// transfer 1 -> 2
		assert_ok!(PalletKitties::transfer_kitty(signer, ACCOUNT_ID_2, KITTY_ID_0));
		assert_eq!(PalletKitties::kitty_owner(KITTY_ID_0), Some(ACCOUNT_ID_2));
		System::assert_last_event(
			Event::KittyTransferred {
				sender: ACCOUNT_ID_1,
				recipient: ACCOUNT_ID_2,
				kitty_id: KITTY_ID_0,
			}
			.into(),
		);

		// transfer 2 -> 1
		assert_ok!(PalletKitties::transfer_kitty(signer_2, ACCOUNT_ID_1, KITTY_ID_0));
		assert_eq!(PalletKitties::kitty_owner(KITTY_ID_0), Some(ACCOUNT_ID_1));
		System::assert_last_event(
			Event::KittyTransferred {
				sender: ACCOUNT_ID_2,
				recipient: ACCOUNT_ID_1,
				kitty_id: KITTY_ID_0,
			}
			.into(),
		);
	});
}

#[test]
fn sale_kitty() {
	new_test_ext().execute_with(|| {
		assert_ok!(Balances::set_balance(RuntimeOrigin::root(), ACCOUNT_ID_1, 10_000_000, 0));

		let signer = RuntimeOrigin::signed(ACCOUNT_ID_1);
		let signer_2 = RuntimeOrigin::signed(ACCOUNT_ID_2);

		// sale 不存在的 kitty
		assert_noop!(
			PalletKitties::sale_kitty(signer.clone(), KITTY_ID_0),
			Error::<Test>::KittyNotExist
		);

		// sale 不属于自己的 kitty
		assert_ok!(PalletKitties::create_kitty(signer.clone(), KITTY_NAME));
		assert_noop!(PalletKitties::sale_kitty(signer_2, KITTY_ID_0), Error::<Test>::NotKittyOwner);

		// sale 上架成功
		assert_ok!(PalletKitties::sale_kitty(signer.clone(), KITTY_ID_0));
		System::assert_last_event(
			Event::KittyOnSale { account: ACCOUNT_ID_1, kitty_id: KITTY_ID_0 }.into(),
		);

		// 重复上架
		assert_noop!(
			PalletKitties::sale_kitty(signer, KITTY_ID_0),
			Error::<Test>::KittyAlreadyOnSale
		);
	});
}

#[test]
fn buy_kitty() {
	new_test_ext().execute_with(|| {
		assert_ok!(Balances::set_balance(RuntimeOrigin::root(), ACCOUNT_ID_1, 10_000_000, 0));
		assert_ok!(Balances::set_balance(RuntimeOrigin::root(), ACCOUNT_ID_2, 10_000_000, 0));

		let signer = RuntimeOrigin::signed(ACCOUNT_ID_1);
		let signer_2 = RuntimeOrigin::signed(ACCOUNT_ID_2);

		// buy 不存在的 kitty
		assert_noop!(
			PalletKitties::buy_kitty(signer.clone(), KITTY_ID_0),
			Error::<Test>::KittyNotExist
		);

		// buy 自己的 kitty
		assert_ok!(PalletKitties::create_kitty(signer.clone(), KITTY_NAME));
		assert_noop!(
			PalletKitties::buy_kitty(signer.clone(), KITTY_ID_0),
			Error::<Test>::KittyAlreadyOwned
		);

		// buy 未上架的 kitty
		assert_noop!(
			PalletKitties::buy_kitty(signer_2.clone(), KITTY_ID_0),
			Error::<Test>::KittyNotOnSale
		);

		// buy 成功
		assert_ok!(PalletKitties::sale_kitty(signer.clone(), KITTY_ID_0));
		assert_ok!(PalletKitties::buy_kitty(signer_2.clone(), KITTY_ID_0));
		System::assert_last_event(
			Event::KittyBought { buyer: ACCOUNT_ID_2, kitty_id: KITTY_ID_0 }.into(),
		);
	});
}
