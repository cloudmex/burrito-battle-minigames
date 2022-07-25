use crate::*;
use near_sdk::{ext_contract, Gas, PromiseResult};

const GAS_FOR_RESOLVE_TRANSFER: Gas = Gas(10_000_000_000_000);
const GAS_FOR_NFT_TRANSFER_CALL: Gas = Gas(25_000_000_000_000 + GAS_FOR_RESOLVE_TRANSFER.0);
const MIN_GAS_FOR_NFT_TRANSFER_CALL: Gas = Gas(100_000_000_000_000);
const NO_DEPOSIT: Balance = 0;

#[ext_contract(ext_nft)]
pub trait ExternsContract {
    fn nft_transfer(
        &mut self,
        receiver_id: AccountId,
        token_id: String
    );
    fn get_balance_and_transfer_hospital(&self,account_id: String, action: String, treasury_id: String, cost: u128) -> bool;
    fn get_burrito_capsule(&self,burrito_id: TokenId) -> Burrito;
}

#[ext_contract(ext_self)]
trait NonFungibleTokenResolver {
    fn get_burrito_info(&self, player: String, capsules: RecoveryCapsules) -> bool;
}