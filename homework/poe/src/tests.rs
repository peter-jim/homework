use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};
use super::*;

// 创建存证成功的测试用例
#[test]
fn create_claim_works() {
	new_test_ext().execute_with(|| {
	// Dispatch a signed extrinsic.引入测试用外部事件
		let claim = vec![0,1];
		assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));
		// assert.ok(value)这个函数的用法没查到。
		assert_eq!(
		
			Proofs::<Test>::get(&claim), 
            
			(1, frame_system::Pallet::<Test>::block_number()));
		
	})
}
// 创建存证失败的测试用例
#[test]
fn create_claim_failed_when_claim_already_exist(){
	new_test_ext().execute_with(|| {
		let claim = vec![0,1];
		let _ = (PoeModule::create_claim(Origin::signed(1), claim.clone()));
			assert_noop!(
				PoeModule::create_claim(Origin::signed(1), claim.clone()),
		
        Error::<Test>::ProofAlreadyExist
	);
	})
}

// 撤销存证成功的测试用例
#[test]
fn revoke_claim_works(){
	new_test_ext().execute_with(||{
		let claim = vec![0,1];
		let _ = (PoeModule::create_claim(Origin::signed(1), claim.clone()));

		assert_ok!(PoeModule::revoke_claim(Origin::signed(1), claim.clone()));
		assert_eq!(
			Proofs::<Test>::get(&claim),
            
			(0, 0)
		);

	})
}

// 撤销存证失败的测试用例
#[test]
fn revoke_claim_failed_when_claim_is_not_exist(){
	new_test_ext().execute_with(|| {
		let claim = vec![0,1];
		let _ = (PoeModule::revoke_claim(Origin::signed(1), claim.clone()));
			assert_noop!(
				PoeModule::revoke_claim(Origin::signed(1), claim.clone()),
        // 教程演示代码需要修改：NoSuchProof是2021-08版的写法。
			Error::<Test>::ClaimNotExist
	);
	})
}

// 作业：转移存证成功的测试用例
#[test]
fn transfer_claim_work() {
	new_test_ext().execute_with(||{
		let claim = vec![0,1];
		PoeModule::create_claim(Origin::signed(1),claim.clone());
		let _ = PoeModule::transfer_claim(Origin::signed(1),claim.clone(),2);
		assert_eq!(
			Proofs::<Test>::get(&claim),
			(2,frame_system::Pallet::<Test>::block_number()
		));
	})
}


#[test]
fn transfer_claim_fail() {
	new_test_ext().execute_with(||{
		let claim = vec![0,1];
		PoeModule::create_claim(Origin::signed(1),claim.clone());
		let _ = PoeModule::transfer_claim(Origin::signed(1),claim.clone(),2);
		assert_noop!(
			PoeModule::revoke_claim(Origin::signed(1),claim.clone()),
			Error::<Test>::NotClaimOwner
		);
	})
}