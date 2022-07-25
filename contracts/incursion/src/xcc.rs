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
    fn get_balance_and_transfer_minigames(&self,account_id: String, action: String, treasury_id: String) -> bool;
    fn get_burrito_incursion(&self,burrito_id: TokenId) -> Burrito;
    fn decrease_all_burrito_hp(&self, burrito_id: TokenId) -> Burrito;
    fn reward_player(&self,player_owner_id: String,tokens_mint: String) -> String;
}

#[ext_contract(ext_self)]
trait NonFungibleTokenResolver {
    fn get_burrito_info(&self,incursion:Incursion, player:BurritoPlayer, contract_id: String) -> bool;
    fn register_player_incursion(&self,incursion:Incursion, player:BurritoPlayer, contract_id: String) -> bool;
    fn save_battle_room(&self, player:Player, incrusion:Incursion, player_id:AccountId) -> BPvsMB;    
}