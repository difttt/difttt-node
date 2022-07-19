use primitives::{Balance, CurrencyId, TradingPair};
//swap token
pub trait BuyTokenInterface {
	fn buy_token(
		who: &T::AccountId,
		path: &[CurrencyId],
		token_a: Balance,
		token_b: Balance,
	) -> sp_std::result::Result<Balance, DispatchError>;

	fn swap_token(
		who: &T::AccountId,
		path: &[CurrencyId],
		limit: SwapLimit<Balance>,
	) -> sp_std::result::Result<Balance, DispatchError>;
}
