use primitives::{Balance, CurrencyId, TradingPair};
//use method to transfer token
pub trait BuyTokenInterface {
     fn buyToken(sell_token_name:CurrencyId,buy_token_name:CurrencyId,amount:Balance);
}

//
// pub trait StorageInterface {
// 	type Value;
// 	fn get_param() -> Self::Value;
// 	fn set_param(v: Self::Value);
// }