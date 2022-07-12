use primitives::{Balance, CurrencyId, TradingPair};
//swap token
pub trait BuyTokenInterface {
     fn buyToken(sell_token_name:CurrencyId,buy_token_name:CurrencyId,amount:Balance);
}