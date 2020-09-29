use sp_runtime::{
  DispatchError,
};

pub struct IncentivePoolAccountInfo<Share, Balance> {
  pub shares: Share,
  pub accumlated_rewards: Balance,
}

pub trait IncentiveOps<AccountId, CurrencyId, Share, Balance> {
  fn add_share(who: &AccountId, left: &CurrencyId, right: &CurrencyId, amount: &Share) -> Result<Share, DispatchError>;
  fn remove_share(who: &AccountId, left: &CurrencyId, right: &CurrencyId, amount: &Share) -> Result<Share, DispatchError>;

  fn get_account_shares(who: &AccountId, left: &CurrencyId, right: &CurrencyId) -> Share;
  fn get_accumlated_rewards(who: &AccountId, left: &CurrencyId, right: &CurrencyId) -> Balance;
  fn get_account_info(who: &AccountId, left: &CurrencyId, right: &CurrencyId) -> IncentivePoolAccountInfo<Share, Balance>;
}