use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok, traits::ConstU32, BoundedVec};

use super::*;

#[test]
fn create_trigger_should_work() {
	new_test_ext().execute_with(|| {
		// pub enum Triger {
		// 	Timer(u64, u64),    //insert_time,  timer_seconds
		// 	Schedule(u64, u64), //insert_time,  timestamp
		// 	PriceGT(u64, u64),  //insert_time,  price   //todo,price use float
		// 	PriceLT(u64, u64),  //insert_time,  price   //todo,price use float
		// }

		let timer = Triger::Timer(1, 1);
		let schedule = Triger::Schedule(2, 2);
		let price_gt = Triger::PriceGT(3, 3);
		let price_lt = Triger::PriceLT(4, 4);
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::create_triger(Origin::signed(1), timer)); //枚举实例一，通过
		assert_ok!(TemplateModule::create_triger(Origin::signed(1), schedule)); //枚举实例二，通过
		assert_ok!(TemplateModule::create_triger(Origin::signed(1), price_gt)); //枚举实例三，通过
		assert_ok!(TemplateModule::create_triger(Origin::signed(1), price_lt)); //枚举实例四，通过
	});
}

#[test]
fn create_action_should_work() {
	new_test_ext().execute_with(|| {
		let a_u8_const128: BoundedVec<u8, ConstU32<128>> = vec![1, 2, 3].try_into().unwrap();
		let b_u8_const256: BoundedVec<u8, ConstU32<256>> = vec![4, 5, 6].try_into().unwrap();
		let c_u8_const128: BoundedVec<u8, ConstU32<128>> = vec![1, 2, 3].try_into().unwrap();
		let d_u8_const128: BoundedVec<u8, ConstU32<128>> = vec![1, 2, 3].try_into().unwrap();
		let e_u8_const256: BoundedVec<u8, ConstU32<256>> = vec![4, 5, 6].try_into().unwrap();
		let mail_with_token = Action::MailWithToken(
			a_u8_const128,
			b_u8_const256,
			c_u8_const128,
			d_u8_const128,
			e_u8_const256,
		);
		let a_u8_const32: BoundedVec<u8, ConstU32<32>> = vec![1, 2, 3].try_into().unwrap();
		let b_u8_const128: BoundedVec<u8, ConstU32<128>> = vec![4, 5, 6].try_into().unwrap();
		let oracle = Action::Oracle(a_u8_const32, b_u8_const128);
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::create_action(Origin::signed(1), mail_with_token)); //枚举实例一，通过
		assert_ok!(TemplateModule::create_action(Origin::signed(1), oracle)); //枚举实例二，通过
	});
}

#[test]
fn create_recipe_should_work() {
	new_test_ext().execute_with(|| {
		ActionOwner::<Test>::insert(1, 1, ());
		TrigerOwner::<Test>::insert(1, 1, ());
		assert_ok!(TemplateModule::create_recipe(Origin::signed(1), 1, 1)); //测试失败，可能是参数设置不正确
	});
}

#[test]
fn turn_on_recipe_should_work() {
	new_test_ext().execute_with(|| {
		RecipeOwner::<Test>::insert(1, 1, ());
		assert_ok!(TemplateModule::turn_on_recipe(Origin::signed(1), 1)); //测试失败，可能是参数设置不正确
	});
}

#[test]
fn turn_off_recipe_should_work() {
	new_test_ext().execute_with(|| {
		RecipeOwner::<Test>::insert(0, 0, ());
		assert_ok!(TemplateModule::turn_off_recipe(Origin::signed(1), 0)); //测试失败，可能是参数设置不正确
	});
}

#[test]
fn del_recipe_should_work() {
	new_test_ext().execute_with(|| {
		RecipeOwner::<Test>::insert(2, 2, ());
		assert_ok!(TemplateModule::turn_off_recipe(Origin::signed(1), 2)); //测试失败，可能是参数设置不正确
	});
}
