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

const GAS_FOR_RESOLVE_TRANSFER: Gas = Gas(10_000_000_000_000);
const GAS_FOR_NFT_TRANSFER_CALL: Gas = Gas(25_000_000_000_000 + GAS_FOR_RESOLVE_TRANSFER.0);
const MIN_GAS_FOR_NFT_TRANSFER_CALL: Gas = Gas(100_000_000_000_000);
const NO_DEPOSIT: Balance = 0;

pub use crate::xcc::*;
pub use crate::migrate::*;
mod xcc;
mod migrate;

construct_uint! {
    pub struct U256(4);
}

#[derive( Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct MsgInput {
    pub capsule_number: u64,
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
    capsule1: Capsule,
    capsule2: Capsule,
    capsule3: Capsule
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
pub struct OldContract {
    pub owner_account_id: AccountId,
    pub treasury_id: AccountId,
    pub cost_strw: u128,
    pub epoch_to_restore: u64,
    pub capsules: HashMap<AccountId, RecoveryCapsules>,

    pub burrito_contract: String,
    pub hospital_contract: String,
    pub strw_contract: String
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub owner_account_id: AccountId,
    pub treasury_id: AccountId,
    pub cost_strw: u128,
    pub epoch_to_restore: u64,
    pub capsules: HashMap<AccountId, RecoveryCapsules>,

    pub burrito_contract: String,
    pub hospital_contract: String,
    pub strw_contract: String
}

#[near_bindgen]
impl Contract {
    //Initialize the contract
    #[init]
    pub fn new(owner_account_id: AccountId, treasury_id: AccountId, cost_strw: u128, epoch_to_restore: u64, burrito_contract: String, hospital_contract: String, strw_contract: String) -> Self {
        assert!(!env::state_exists(), "The contract is already initialized");
        let result = Self{
            owner_account_id,
            treasury_id : treasury_id,
            cost_strw : cost_strw,
            epoch_to_restore : epoch_to_restore,
            capsules: HashMap::new(),
            burrito_contract,
            hospital_contract,
            strw_contract
        };
        return result;
    }

    // Cambiar contratos
    pub fn change_contracts(&mut self, burrito_contract: String, hospital_contract: String, strw_contract: String) {
        self.assert_owner_calling();
        self.burrito_contract = burrito_contract;
        self.hospital_contract = hospital_contract;
        self.strw_contract = strw_contract;
    }

    // Mostrar contratos
    pub fn show_contracts(&self) {
        log!("burrito_contract: {}",self.burrito_contract);
        log!("hospital_contract: {}",self.hospital_contract);
        log!("strw_contract: {}",self.strw_contract);
    }

    // Cambiar owner
    pub fn change_owner(&mut self, owner_account_id: AccountId) {
        self.assert_owner_calling();
        self.owner_account_id = owner_account_id;
    }
    
    // Cambiar epocas que durará para recuperar burrito
    pub fn change_epoch_restore(&mut self, epoch_to_restore: u64) -> String{
        self.assert_owner_calling();
        self.epoch_to_restore = epoch_to_restore;
        "Epocas de restauración actualizadas".to_string()
    }

    // Cambiar costo de capsula
    pub fn change_strw_cost(&mut self, cost_strw: u128) -> String{
        self.assert_owner_calling();
        self.cost_strw = cost_strw;
        "Costo de capsula actualizado".to_string()
    }

    // Obtener costo de capsula
    pub fn get_strw_cost(&self) -> u128{
        self.cost_strw
    }

    // Cambiar tesorero
    pub fn change_treasury(&mut self, new_treasury: AccountId) -> String {
        self.assert_owner_calling();
        self.treasury_id = new_treasury;
        "Tesorero actualizado".to_string()
    }

    // Mostrar costos y epocas
    pub fn get_contract_info(&self, ) {
        log!("Owner Account: {}",self.owner_account_id);
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
        let msg_json: MsgInput = from_str(&msg).unwrap();
        let capsule_number = msg_json.capsule_number.clone();

        log!("Capsula a ingresar: {}",capsule_number.clone());

        if capsule_number.clone() <= 0 || capsule_number.clone() >= 4 {
            log!("Debes seleccionar la capsula del 1 al 3");
            return near_sdk::PromiseOrValue::Value(true); // Regresar burrito al jugador
        }

        let exist_capsules_player = self.capsules.get(&player);

        if exist_capsules_player.is_none() {
            log!("No hay ninguna capsula registrada");
            
            let new_capsule = Capsule {
                burrito_id: token_id.clone().to_string(),
                burrito_owner: player.clone().to_string(),
                burrito_contract: contract_id.clone().to_string(),
                start_time: actual_epoch.clone(),
                finish_time: actual_epoch.clone()+(300000000000*self.epoch_to_restore)
                //finish_time: actual_epoch.clone()+(43200000000000*self.epoch_to_restore)
            };
            
            let empty_capsule = Capsule {
                burrito_id: "".to_string(),
                burrito_owner: "".to_string(),
                burrito_contract: "".to_string(),
                start_time: 0,
                finish_time: 0
            };

            let mut player_capsules = RecoveryCapsules {
                count : 1,
                capsule1 : empty_capsule.clone(),
                capsule2 : empty_capsule.clone(),
                capsule3 : empty_capsule.clone()
            };

            if capsule_number == 1 {
                player_capsules.capsule1 = new_capsule.clone();
            }
            if capsule_number == 2 {
                player_capsules.capsule2 = new_capsule.clone();
            }
            if capsule_number == 3 {
                player_capsules.capsule3 = new_capsule.clone();
            }
            
            // Consultar información del burrito
            let call = ext_nft::get_burrito_capsule(
                token_id.clone().to_string(),
                self.burrito_contract.parse::<AccountId>().unwrap(),
                NO_DEPOSIT,
                Gas(100_000_000_000_000)
            );

            let callback = ext_self::get_burrito_info(
                player.clone().to_string().clone(),
                player_capsules.clone(),
                self.hospital_contract.parse::<AccountId>().unwrap(),
                NO_DEPOSIT,
                Gas(100_000_000_000_000)
            );

            return near_sdk::PromiseOrValue::Promise(call.then(callback));  

        } else {
            log!("Ya existe capsula registrada");
            let info = self.capsules.get(&player).unwrap();

            if info.count.clone() == 3 {
                log!("Las 3 capsulas de rehabilitación ya están llenas");
                return near_sdk::PromiseOrValue::Value(true); // Regresar burrito al jugador
            }
            
            let new_capsule = Capsule {
                burrito_id: token_id.clone().to_string(),
                burrito_owner: player.clone().to_string(),
                burrito_contract: contract_id.clone().to_string(),
                start_time: actual_epoch.clone(),
                finish_time: actual_epoch.clone()+(300000000000*self.epoch_to_restore)
                //finish_time: actual_epoch.clone()+(43200000000000*self.epoch_to_restore)
            };

            let mut player_capsules = RecoveryCapsules {
                count : 1,
                capsule1 : info.capsule1.clone(),
                capsule2 : info.capsule2.clone(),
                capsule3 : info.capsule3.clone()
            };

            let n_capsule : u64 = info.count.clone()+1;
            player_capsules.count = n_capsule;

            if capsule_number == 1 {
                if player_capsules.capsule1.burrito_id != "".to_string() {
                    log!("La capsula 1 ya está ocupada");
                    return near_sdk::PromiseOrValue::Value(true); // Regresar burrito al jugador
                }
                player_capsules.capsule1 = new_capsule.clone();
            }
            if capsule_number == 2 {
                if player_capsules.capsule2.burrito_id != "".to_string() {
                    log!("La capsula 2 ya está ocupada");
                    return near_sdk::PromiseOrValue::Value(true); // Regresar burrito al jugador
                }
                player_capsules.capsule2 = new_capsule.clone();
            }
            if capsule_number == 3 {
                if player_capsules.capsule3.burrito_id != "".to_string() {
                    log!("La capsula 3 ya está ocupada");
                    return near_sdk::PromiseOrValue::Value(true); // Regresar burrito al jugador
                }
                player_capsules.capsule3 = new_capsule.clone();
            }

            // Consultar información del burrito
            let call = ext_nft::get_burrito_capsule(
                token_id.clone().to_string(),
                self.burrito_contract.parse::<AccountId>().unwrap(),
                NO_DEPOSIT,
                Gas(100_000_000_000_000)
            );

            let callback = ext_self::get_burrito_info(
                player.clone().to_string().clone(),
                player_capsules.clone(),
                self.hospital_contract.parse::<AccountId>().unwrap(),
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

                if burrito.hp.parse::<u8>().unwrap() > 0 {
                    log!("El burrito aún tiene vidas");
                    return near_sdk::PromiseOrValue::Value(true); // Regresar burrito al jugador
                }
                
                ext_nft::get_balance_and_transfer_hospital(
                    player.clone().to_string(),
                    "Capsule".to_string(),
                    self.treasury_id.to_string(),
                    self.cost_strw.clone(),
                    self.strw_contract.parse::<AccountId>().unwrap(),
                    NO_DEPOSIT,
                    Gas(60_000_000_000_000)
                );

                self.capsules.insert(player,capsules.clone());

                return near_sdk::PromiseOrValue::Value(false); // No regresar al burrito
            }
        }
    }

    // Recuperar burrito
    #[payable]
    pub fn withdraw_burrito_owner(&mut self, capsule_number: u64) -> ClaimedBurrito {
        let player = env::signer_account_id();
        let deposit = env::attached_deposit();
    
        let exist_capsules_player = self.capsules.get(&player);

        if exist_capsules_player.is_none() {
            log!("No hay ninguna capsula registrada");
            let res = ClaimedBurrito{
                complete : false,
                msg : "No hay ninguna capsula registrada".to_string(),
                burrito_id : "".to_string()
            };
            return res;
        } else {
            log!("Hay capsulas registradas");
            log!("Capsula a retirar: {}",capsule_number);

            let info = self.capsules.get(&player).unwrap();

            let mut player_capsules = RecoveryCapsules {
                count: info.count.clone(),
                capsule1: info.capsule1.clone(),
                capsule2: info.capsule2.clone(),
                capsule3: info.capsule3.clone()
            };

            if capsule_number.clone() <= 0 || capsule_number.clone() >= 4 {
                log!("No existe la capsula ingresada");
                let res = ClaimedBurrito{
                    complete : false,
                    msg : "No existe la capsula ingresada".to_string(),
                    burrito_id : "".to_string()
                };
                return res;
            }

            if capsule_number.clone() == 1 && player_capsules.capsule1.burrito_id.clone() == "" {
                log!("No hay burrito en la capsula 1");
                let res = ClaimedBurrito{
                    complete : false,
                    msg : "No hay burrito en la capsula 1".to_string(),
                    burrito_id : "".to_string()
                };
                return res;
            }

            if capsule_number.clone() == 2 && player_capsules.capsule2.burrito_id.clone() == "" {
                log!("No hay burrito en la capsula 2");
                let res = ClaimedBurrito{
                    complete : false,
                    msg : "No hay burrito en la capsula 2".to_string(),
                    burrito_id : "".to_string()
                };
                return res;
            }

            if capsule_number.clone() == 3 && player_capsules.capsule3.burrito_id.clone() == "" {
                log!("No hay burrito en la capsula 3");
                let res = ClaimedBurrito{
                    complete : false,
                    msg : "No hay burrito en la capsula 3".to_string(),
                    burrito_id : "".to_string()
                };
                return res;
            }

            let mut capsule = Capsule {
                burrito_id: "".to_string(),
                burrito_owner: "".to_string(),
                burrito_contract: "".to_string(),
                start_time: 0,
                finish_time: 0
            };

            if capsule_number.clone() == 1 {
                capsule.burrito_id = info.capsule1.burrito_id.clone();
                capsule.burrito_owner = info.capsule1.burrito_owner.clone();
                capsule.burrito_contract = info.capsule1.burrito_contract.clone();
                capsule.start_time = info.capsule1.start_time.clone();
                capsule.finish_time = info.capsule1.finish_time.clone();
            }

            if capsule_number.clone() == 2 {
                capsule.burrito_id = info.capsule2.burrito_id.clone();
                capsule.burrito_owner = info.capsule2.burrito_owner.clone();
                capsule.burrito_contract = info.capsule2.burrito_contract.clone();
                capsule.start_time = info.capsule2.start_time.clone();
                capsule.finish_time = info.capsule2.finish_time.clone();
            }

            if capsule_number.clone() == 3 {
                capsule.burrito_id = info.capsule3.burrito_id.clone();
                capsule.burrito_owner = info.capsule3.burrito_owner.clone();
                capsule.burrito_contract = info.capsule3.burrito_contract.clone();
                capsule.start_time = info.capsule3.start_time.clone();
                capsule.finish_time = info.capsule3.finish_time.clone();
            }

            if player.clone() != capsule.burrito_owner.clone().parse::<AccountId>().unwrap(){
                log!("No eres el dueño del burrito");
                let res = ClaimedBurrito{
                    complete : false,
                    msg : "No eres el dueño del burrito".to_string(),
                    burrito_id : "".to_string()
                };
                return res;
            }
            
            let actual_epoch = env::block_timestamp();

            if actual_epoch < capsule.finish_time {
                log!("Aún no finaliza el tiempo de restauración");
                let res = ClaimedBurrito{
                    complete : false,
                    msg : "Aún no finaliza el tiempo de restauración".to_string(),
                    burrito_id : "".to_string()
                };
                return res;  
            }

            ext_nft::increase_all_burrito_hp(
                capsule.burrito_id.clone(),
                self.burrito_contract.parse::<AccountId>().unwrap(),
                NO_DEPOSIT,
                MIN_GAS_FOR_NFT_TRANSFER_CALL
            ).then(ext_nft::nft_transfer(
                player.clone(),
                capsule.burrito_id.clone(),
                capsule.burrito_contract.parse::<AccountId>().unwrap(),
                deposit,
                MIN_GAS_FOR_NFT_TRANSFER_CALL
            ));

            let res = ClaimedBurrito{
                complete : true,
                msg : "El burrito recuperó sus vidas".to_string(),
                burrito_id : capsule.burrito_id.clone()
            };

            let empty_capsule = Capsule {
                burrito_id: "".to_string(),
                burrito_owner: "".to_string(),
                burrito_contract: "".to_string(),
                start_time: 0,
                finish_time: 0
            };

            let new_count = player_capsules.count.clone() - 1;
            
            player_capsules.count = new_count;

            if capsule_number.clone() == 1 {
                player_capsules.capsule1 = empty_capsule.clone();
            }

            if capsule_number.clone() == 2 {
                player_capsules.capsule2 = empty_capsule.clone();
            }

            if capsule_number.clone() == 3 {
                player_capsules.capsule3 = empty_capsule.clone();
            }

            self.capsules.insert(player.clone(),player_capsules.clone());

            return res;  
        }
    }    

    // Obtener capsulas del jugador
    pub fn get_player_capsules(&self, player: AccountId) -> RecoveryCapsules {
        let exist_capsules_player = self.capsules.get(&player);
        if exist_capsules_player.is_none() {
            log!("No hay ninguna capsula registrada");
    
            let empty_capsule = Capsule {
                burrito_id: "".to_string(),
                burrito_owner: "".to_string(),
                burrito_contract: "".to_string(),
                start_time: 0,
                finish_time: 0
            };

            let player_capsules = RecoveryCapsules {
                count: 0,
                capsule1: empty_capsule.clone(),
                capsule2: empty_capsule.clone(),
                capsule3: empty_capsule.clone()
            };

            return player_capsules;
        } else {
            log!("Ya existe capsula registrada");
            let info = self.capsules.get(&player).unwrap();

            let player_capsules = RecoveryCapsules {
                count: info.count.clone(),
                capsule1: info.capsule1.clone(),
                capsule2: info.capsule2.clone(),
                capsule3: info.capsule3.clone()
            };

            return player_capsules;
        }
    }

    pub fn assert_owner_calling(&self) {
        assert!(
            env::predecessor_account_id() == self.owner_account_id,
            "can only be called by the owner"
        );
    }
    
}