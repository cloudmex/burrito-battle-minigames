use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use std::collections::HashMap;
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::{env,ext_contract, Balance,Gas, near_bindgen, AccountId, PromiseOrValue, PromiseResult, PanicOnDefault, log, BorshStorageKey};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::serde_json::{json,from_str};
use near_sdk::Promise;
use uint::construct_uint;
use near_sdk::json_types::{U128, U64};


use std::cmp::min;

near_sdk::setup_alloc!();

pub type EpochHeight = u64;
pub type TokenId = String;
pub const BURRITO_CONTRACT: &str = "dev-1652924595303-59024384289373";
pub const HOSPITAL_CONTRACT: &str = "dev-1658170507800-83790945510897";
pub const STRWTOKEN_CONTRACT: &str = "dev-1653415145729-47929415561597";

const GAS_FOR_RESOLVE_TRANSFER: Gas = Gas(10_000_000_000_000);
const GAS_FOR_NFT_TRANSFER_CALL: Gas = Gas(25_000_000_000_000 + GAS_FOR_RESOLVE_TRANSFER.0);
const MIN_GAS_FOR_NFT_TRANSFER_CALL: Gas = Gas(100_000_000_000_000);
const NO_DEPOSIT: Balance = 0;

pub use crate::xcc::*;
mod xcc;

construct_uint! {
    pub struct U256(4);
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Burrito {
    owner_id : String,
    name : String,
    description : String,
    burrito_type : String,
    hp : String,
    attack : String,
    defense : String,
    speed : String,
    win : String,
    global_win : String,
    level : String,
    media : String
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct RecoveryCapsules {
    count: u64,
    capsules: Vec<Capsule>,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Capsule {
    burrito_id: String,
    burrito_owner: String,
    burrito_contract: String,
    start_time: EpochHeight,
    finish_time: EpochHeight
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ClaimedBurrito {
    complete : bool,
    msg : String,
    burrito_id : String
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub owner_account_id: AccountId,
    pub treasury_id: AccountId,
    pub cost_strw: u128,
    pub epoch_to_restore: u64,
    pub capsules: HashMap<AccountId, RecoveryCapsules>,
}

#[near_bindgen]
impl Contract {
    //Initialize the contract
    #[init]
    pub fn new(owner_account_id: AccountId, treasury_id: AccountId, cost_strw: u128, epoch_to_restore: u64) -> Self {
        assert!(!env::state_exists(), "The contract is already initialized");
        let result = Self{
            owner_account_id,
            treasury_id : treasury_id,
            cost_strw : cost_strw,
            epoch_to_restore : epoch_to_restore,
            capsules: HashMap::new()
        };
        return result;
    }
   
    // Obtener capsulas del jugador
    pub fn get_player_capsules(&self, player: AccountId) -> RecoveryCapsules {
        let exist_capsules_player = self.capsules.get(&player);
        if exist_capsules_player.is_none() {
            log!("No hay ninguna capsula registrada");
    
            let player_capsules = RecoveryCapsules {
                count: 0,
                capsules: [].to_vec()
            };

            return player_capsules;
        } else {
            log!("Ya existe capsula registrada");
            let info = self.capsules.get(&player).unwrap();

            let player_capsules = RecoveryCapsules {
                count: info.count.clone(),
                capsules: info.capsules.clone()
            };

            return player_capsules;
        }
    }

    // Mostrar costos y epocas
    pub fn get_contract_info(&self, ) {
        log!("Treasury Account: {}",self.treasury_id);
        log!("STRW Cost: {}",self.cost_strw);
        log!("Epoch To Restore: {}",self.epoch_to_restore);
    }

    // ELIMINAR TODAS LAS CAPSULAS CREADAS
    pub fn delete_all_capsules(&mut self){
        self.capsules.clear();
    }

    // Insertar burrito en capsula
    pub fn nft_on_transfer(&mut self,sender_id: AccountId,previous_owner_id: AccountId,token_id: String,msg: String)  -> PromiseOrValue<bool>{
        let contract_id = env::predecessor_account_id();
        let signer_id = env::signer_account_id();
        let player = previous_owner_id.clone();
        let actual_epoch = env::block_timestamp();

        let exist_capsules_player = self.capsules.get(&player);

        if exist_capsules_player.is_none() {
            log!("No hay ninguna capsula registrada");
            
            let new_capsule = Capsule {
                burrito_id: token_id.clone().to_string(),
                burrito_owner: player.clone().to_string(),
                burrito_contract: contract_id.clone().to_string(),
                start_time: actual_epoch.clone(),
                finish_time: actual_epoch.clone()+(43200000000000*self.epoch_to_restore)
            };
            let mut capsules: Vec<Capsule> = [].to_vec();
            capsules.push(new_capsule.clone());

            let player_capsules = RecoveryCapsules {
                count: 1,
                capsules: capsules
            };

            // Consultar información del burrito
            let call = ext_nft::get_burrito_capsule(
                token_id.clone().to_string(),
                BURRITO_CONTRACT.parse::<AccountId>().unwrap(),
                NO_DEPOSIT,
                Gas(100_000_000_000_000)
            );

            let callback = ext_self::get_burrito_info(
                player.clone().to_string().clone(),
                player_capsules.clone(),
                HOSPITAL_CONTRACT.parse::<AccountId>().unwrap(),
                NO_DEPOSIT,
                Gas(100_000_000_000_000)
            );

            return near_sdk::PromiseOrValue::Promise(call.then(callback));  

        } else {
            log!("Ya existe capsula registrada");
            let info = self.capsules.get(&player).unwrap();
            let mut capsules = info.capsules.clone().to_vec();

            if info.count.clone() >= 3 {
                log!("Las 3 capsulas de rehabilitación ya están llenas");
                return near_sdk::PromiseOrValue::Value(true); // Regresar burrito al jugador
            }
            
            let new_capsule = Capsule {
                burrito_id: token_id.clone().to_string(),
                burrito_owner: player.clone().to_string(),
                burrito_contract: contract_id.clone().to_string(),
                start_time: actual_epoch.clone(),
                finish_time: actual_epoch.clone()+(43200000000000*self.epoch_to_restore)
            };

            capsules.push(new_capsule.clone());
            let n_capsule : u64 = info.count.clone()+1;
            let player_capsules = RecoveryCapsules {
                count: n_capsule,
                capsules: capsules
            };

            // Consultar información del burrito
            let call = ext_nft::get_burrito_capsule(
                token_id.clone().to_string(),
                BURRITO_CONTRACT.parse::<AccountId>().unwrap(),
                NO_DEPOSIT,
                Gas(100_000_000_000_000)
            );

            let callback = ext_self::get_burrito_info(
                player.clone().to_string().clone(),
                player_capsules.clone(),
                HOSPITAL_CONTRACT.parse::<AccountId>().unwrap(),
                NO_DEPOSIT,
                Gas(100_000_000_000_000)
            );

            return near_sdk::PromiseOrValue::Promise(call.then(callback));            
        }
    }


    pub fn get_burrito_info(&mut self, player: AccountId, capsules: RecoveryCapsules) -> PromiseOrValue<bool> {        
        assert_eq!(
            env::promise_results_count(),
            1,
            "Éste es un método callback"
        );
        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Failed => {
                return near_sdk::PromiseOrValue::Value(true);
            },
            PromiseResult::Successful(result) => {
                let value = std::str::from_utf8(&result).unwrap();
                let burrito: Burrito = serde_json::from_str(&value).unwrap();

                if burrito.hp.parse::<u8>().unwrap() >= 0 {
                    log!("El burrito aún tiene vidas");
                    return near_sdk::PromiseOrValue::Value(true); // Regresar burrito al jugador
                }
                
                ext_nft::get_balance_and_transfer_hospital(
                    player.clone().to_string(),
                    "Capsule".to_string(),
                    self.treasury_id.to_string(),
                    self.cost_strw.clone(),
                    STRWTOKEN_CONTRACT.parse::<AccountId>().unwrap(),
                    NO_DEPOSIT,
                    Gas(60_000_000_000_000)
                );

                self.capsules.insert(player,capsules.clone());

                return near_sdk::PromiseOrValue::Value(true); // Regresar al burrito
                //return near_sdk::PromiseOrValue::Value(false); // No regresar al burrito
            }
        }
    }

    
}