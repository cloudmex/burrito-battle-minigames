use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use std::collections::HashMap;
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::{env,ext_contract, Balance,Gas, near_bindgen, AccountId, PromiseOrValue, PromiseResult, PanicOnDefault, log, BorshStorageKey, require};
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

#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKey {
    Incursions,
    Battles,
    ByPlayerId,
    IncursionsInAwait,
    IncursionsInAwaitInner { incursion_id: u64 },
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum IncursionStatus {
    WaitingPlayers,
    InProgress,
    Finished,
    Null
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

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct BurritoAttacker {
    burrito_type : String,
    attack : String,
    level : String
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct BurritoDefender {
    burrito_type : String,
    level : String
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Rewards {
    complete : bool,
    win : String,
    msg : String,
    rewards : f32
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct MegaBurrito {
    name : String,
    burrito_type : String,
    start_health : String,
    health : String,
    attack : String,
    defense : String,
    speed : String,
    level : String
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct BurritoPlayer {
    burrito_id: String,
    burrito_owner: AccountId
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Player {
    incursion_id: u64,
    burrito_id: String,
    burrito_owner: String,
    burrito_contract: String
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Incursion {
    id: u64,
    status: IncursionStatus,
    create_time: EpochHeight,
    start_time: EpochHeight,
    finish_time: EpochHeight,
    players_number: u8,
    registered_players: u8,
    win: String,
    mega_burrito: MegaBurrito,
    players: Vec<BurritoPlayer>,
    rewards: u64
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct IncrusionPlayer {
    incursion: Incursion,
    player: Player
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct IncrusionRound {
    incursion: Incursion,
    room: BPvsMB
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct BPvsMB {
    turn : String, 
    burrito_player_id : String,
    name : String,
    burrito_type : String,
    start_health : String,
    health : String,    
    hp : String,
    attack : String,
    defense : String,
    speed : String,    
    level : String,
    media : String,
    strong_attack_player : String,
    shields_player : String,
    incursion_id : u64,
    strong_attack_cpu : String,
    shields_cpu : String,
    damage_player : f32
}

#[derive( Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct MsgInput {
    pub incursion_id: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct PlayerInBattle {
    player_name: String,
    media: String,
    is_alive: bool
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct OldContract {
    pub owner_account_id: AccountId,
    pub treasury_id: AccountId,
    pub incursions: HashMap<u64,Incursion>,
    pub mb_vs_bp: HashMap<AccountId, BPvsMB>,
    pub player_incursion: HashMap<AccountId, Player>,
    pub last_id: u64,

    pub burrito_contract: String,
    pub incursion_contract: String,
    pub strw_contract: String
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub owner_account_id: AccountId,
    pub treasury_id: AccountId,
    pub incursions: HashMap<u64,Incursion>,
    pub mb_vs_bp: HashMap<AccountId, BPvsMB>,
    pub player_incursion: HashMap<AccountId, Player>,
    pub last_id: u64,

    pub burrito_contract: String,
    pub incursion_contract: String,
    pub strw_contract: String
}

#[near_bindgen]
impl Contract {
    //Initialize the contract
    #[init]
    pub fn new(owner_account_id: AccountId, treasury_id: AccountId , burrito_contract: String, incursion_contract: String, strw_contract: String) -> Self {
        assert!(!env::state_exists(), "The contract is already initialized");
        let result = Self{
            owner_account_id,
            treasury_id : treasury_id,
            incursions: HashMap::new(),
            mb_vs_bp: HashMap::new(),
            player_incursion: HashMap::new(),
            last_id: 0,
            burrito_contract,
            incursion_contract,
            strw_contract
        };
        return result;
    }

    // Cambiar contratos
    pub fn change_contracts(&mut self, burrito_contract: String, incursion_contract: String, strw_contract: String) {
        self.assert_owner();
        self.burrito_contract = burrito_contract;
        self.incursion_contract = incursion_contract;
        self.strw_contract = strw_contract;
    }

    // Cambiar owner
    pub fn change_owner(&mut self, owner_account_id: AccountId) {
        self.assert_owner();
        self.owner_account_id = owner_account_id;
    }

    // Cambiar tesorero
    pub fn change_treasury(&mut self, treasury_id: AccountId) {
        self.assert_owner();
        self.treasury_id = treasury_id;
    }

    // Verificar si es owner
    fn assert_owner(&self) {
        require!(self.signer_is_owner(), "Method is private to owner")
    }

    fn signer_is_owner(&self) -> bool {
        self.is_owner(&env::predecessor_account_id())
    }

    fn is_owner(&self, minter: &AccountId) -> bool {
        minter.as_str() == self.owner_account_id.as_str()
    }

    // Mostrar contratos
    pub fn show_contracts(&self) {
        log!("burrito_contract: {}",self.burrito_contract);
        log!("incursion_contract: {}",self.incursion_contract);
        log!("strw_contract: {}",self.strw_contract);
    }

    pub fn show_last_id(&self) -> u64{
        return self.last_id;
    }
    
    // Crear nueva incursión
    pub fn create_incursion(&mut self) -> Incursion {
        let actual_epoch = env::block_timestamp();

        if self.last_id == 0 {
            log!("No Hay ninguna incursión");

            let mut mega_burrito = MegaBurrito {
                name : "Cerberus".to_string(),
                burrito_type : "Fuego".to_string(),
                start_health : "100".to_string(),
                health : "100".to_string(),
                attack : "15".to_string(),
                defense : "15".to_string(),
                speed : "15".to_string(),
                level : "40".to_string()
            };

            let rand_type = *env::random_seed().get(0).unwrap();
            let rand_name = *env::random_seed().get(1).unwrap();
            let mut burrito_type: String = "Fuego".to_string();
            let mut burrito_name: String = "Cerberus".to_string();

            // Obtener tipo
            if rand_type > 0 &&  rand_type <= 51 {
                burrito_type = "Fuego".to_string();
            }
            if rand_type >= 52 &&  rand_type <= 102 {
                burrito_type = "Agua".to_string();
            }
            if rand_type >= 103 &&  rand_type <= 153 {
                burrito_type = "Planta".to_string();
            }
            if rand_type >= 154 &&  rand_type <= 204 {
                burrito_type = "Eléctrico".to_string();
            }
            if rand_type >= 205 &&  rand_type < 255 {
                burrito_type = "Volador".to_string();
            }

            // Obtener nombre
            if rand_type > 0 &&  rand_type <= 85 {
                burrito_name = "Cerberus".to_string();
            }
            if rand_type >= 86 &&  rand_type <= 170 {
                burrito_name = "Hades".to_string();
            }
            if rand_type >= 171 &&  rand_type < 255 {
                burrito_name = "Mictlantecuhtli".to_string();
            }

            mega_burrito.name = burrito_name;
            let rand_rewards = *env::random_seed().get(2).unwrap();
            let mut incursion_rewards: u64 = 0;

            // Obtener recompensa total aleatoria para la incursion 70,000 - 200,000
            if rand_rewards > 0 &&  rand_rewards < 18 {
                incursion_rewards = 70000;
            }
            if rand_rewards >= 18 &&  rand_rewards < 36 {
                incursion_rewards = 80000;
            }
            if rand_rewards >= 36 &&  rand_rewards < 54 {
                incursion_rewards = 90000;
            }
            if rand_rewards >= 54 &&  rand_rewards < 72 {
                incursion_rewards = 100000;
            }
            if rand_rewards >= 72 &&  rand_rewards < 90 {
                incursion_rewards = 110000;
            }
            if rand_rewards >= 90 &&  rand_rewards < 108 {
                incursion_rewards = 120000;
            }
            if rand_rewards >= 108 &&  rand_rewards < 126 {
                incursion_rewards = 130000;
            }
            if rand_rewards >= 126 &&  rand_rewards < 144 {
                incursion_rewards = 140000;
            }
            if rand_rewards >= 144 &&  rand_rewards < 162 {
                incursion_rewards = 150000;
            }
            if rand_rewards >= 162 &&  rand_rewards < 180 {
                incursion_rewards = 160000;
            }
            if rand_rewards >= 180 &&  rand_rewards < 198 {
                incursion_rewards = 170000;
            }
            if rand_rewards >= 198 &&  rand_rewards < 216 {
                incursion_rewards = 180000;
            }
            if rand_rewards >= 216 &&  rand_rewards < 234 {
                incursion_rewards = 190000;
            }
            if rand_rewards >= 234 &&  rand_rewards < 255 {
                incursion_rewards = 200000;
            }

            let new_incursion = Incursion{
                id: self.last_id.clone()+1,
                status: IncursionStatus::WaitingPlayers,
                create_time: actual_epoch.clone(),
                start_time: actual_epoch+3600000000000,  // 1 hr.
                finish_time: actual_epoch+7200000000000, // 2 hrs.
                //start_time: actual_epoch+180000000000, // 3 min.
                //finish_time: actual_epoch+480000000000, // 8 min.
                players_number: 10,
                registered_players: 0,
                win: "".to_string(),
                mega_burrito: mega_burrito,
                players: [].to_vec(),
                rewards: incursion_rewards
            };

            // Crear incursion
            self.incursions.insert(self.last_id.clone()+1,new_incursion.clone());
            self.last_id += 1;
            return new_incursion;

        } else {
            let last_incursion = self.incursions.get(&(self.last_id));            
            //if actual_epoch.clone() <= (last_incursion.clone().unwrap().finish_time+120000000000) { // 2 min.
            if actual_epoch.clone() <= (last_incursion.clone().unwrap().finish_time+108000000000000) {  // 30 hrs.
                env::panic_str("Aún no pasan 30 horas desde la última incursión");
            }

            let mut mega_burrito = MegaBurrito {
                name : "Cerberus".to_string(),
                burrito_type : "Fuego".to_string(),
                start_health : "200".to_string(),
                health : "200".to_string(),
                attack : "15".to_string(),
                defense : "15".to_string(),
                speed : "15".to_string(),
                level : "40".to_string()
            };

            let rand_type = *env::random_seed().get(0).unwrap();
            let rand_name = *env::random_seed().get(1).unwrap();
            let mut burrito_type: String = "Fuego".to_string();
            let mut burrito_name: String = "Cerberus".to_string();

            // Obtener tipo
            if rand_type > 0 &&  rand_type <= 51 {
                burrito_type = "Fuego".to_string();
            }
            if rand_type >= 52 &&  rand_type <= 102 {
                burrito_type = "Agua".to_string();
            }
            if rand_type >= 103 &&  rand_type <= 153 {
                burrito_type = "Planta".to_string();
            }
            if rand_type >= 154 &&  rand_type <= 204 {
                burrito_type = "Eléctrico".to_string();
            }
            if rand_type >= 205 &&  rand_type < 255 {
                burrito_type = "Volador".to_string();
            }

            // Obtener nombre
            if rand_type > 0 &&  rand_type <= 85 {
                burrito_name = "Cerberus".to_string();
            }
            if rand_type >= 86 &&  rand_type <= 170 {
                burrito_name = "Hades".to_string();
            }
            if rand_type >= 171 &&  rand_type < 255 {
                burrito_name = "Mictlantecuhtli".to_string();
            }

            mega_burrito.name = burrito_name;
            let rand_rewards = *env::random_seed().get(2).unwrap();
            let mut incursion_rewards: u64 = 0;
            
            // Obtener recompensa total aleatoria para la incursion 70,000 - 200,000
            if rand_rewards > 0 &&  rand_rewards < 18 {
                incursion_rewards = 70000;
            }
            if rand_rewards >= 18 &&  rand_rewards < 36 {
                incursion_rewards = 80000;
            }
            if rand_rewards >= 36 &&  rand_rewards < 54 {
                incursion_rewards = 90000;
            }
            if rand_rewards >= 54 &&  rand_rewards < 72 {
                incursion_rewards = 100000;
            }
            if rand_rewards >= 72 &&  rand_rewards < 90 {
                incursion_rewards = 110000;
            }
            if rand_rewards >= 90 &&  rand_rewards < 108 {
                incursion_rewards = 120000;
            }
            if rand_rewards >= 108 &&  rand_rewards < 126 {
                incursion_rewards = 130000;
            }
            if rand_rewards >= 126 &&  rand_rewards < 144 {
                incursion_rewards = 140000;
            }
            if rand_rewards >= 144 &&  rand_rewards < 162 {
                incursion_rewards = 150000;
            }
            if rand_rewards >= 162 &&  rand_rewards < 180 {
                incursion_rewards = 160000;
            }
            if rand_rewards >= 180 &&  rand_rewards < 198 {
                incursion_rewards = 170000;
            }
            if rand_rewards >= 198 &&  rand_rewards < 216 {
                incursion_rewards = 180000;
            }
            if rand_rewards >= 216 &&  rand_rewards < 234 {
                incursion_rewards = 190000;
            }
            if rand_rewards >= 234 &&  rand_rewards < 255 {
                incursion_rewards = 200000;
            }

            let new_incursion = Incursion{
                id: self.last_id+1,
                status: IncursionStatus::WaitingPlayers,
                create_time: actual_epoch,
                start_time: actual_epoch+3600000000000,  // 1 hr.
                finish_time: actual_epoch+7200000000000, // 2 hrs.
                //start_time: actual_epoch+180000000000, // 3 min.
                //finish_time: actual_epoch+480000000000, // 8 min.
                players_number: 10,
                registered_players: 0,
                win: "".to_string(),
                mega_burrito: mega_burrito,
                players: [].to_vec(),
                rewards: incursion_rewards
            };

            // Crear incursion
            self.incursions.insert(self.last_id+1,new_incursion.clone());
            self.last_id += 1;
            return new_incursion;
            
        }
    }

    // Obtener la incursión activa
    pub fn get_active_incursion(&self) -> Incursion{
        let last_incursion = self.incursions.get(&(self.last_id));

        if last_incursion.is_none() {
            let actual_epoch = env::block_timestamp();
            let null_incursion = Incursion{
                id: 0,
                status: IncursionStatus::Null,
                create_time: actual_epoch,
                start_time: actual_epoch+3600000000000,
                finish_time: actual_epoch+7200000000000,
                players_number: 0,
                registered_players: 0,
                win: "".to_string(),
                mega_burrito: MegaBurrito {
                    name : "".to_string(),
                    burrito_type : "".to_string(),
                    start_health : "".to_string(),
                    health : "".to_string(),
                    attack : "".to_string(),
                    defense : "".to_string(),
                    speed : "".to_string(),
                    level : "".to_string()
                },
                players: [].to_vec(),
                rewards: 0
            };

            return null_incursion;
        } else {
            let info = last_incursion.clone().unwrap();

            let active_incursion = Incursion{
                id: info.id.clone(),
                status: info.status.clone(),
                create_time: info.create_time.clone(),
                start_time: info.start_time.clone(),
                finish_time: info.finish_time.clone(),
                players_number: info.players_number.clone(),
                registered_players: info.registered_players.clone(),
                win: info.win.clone(),
                mega_burrito: info.mega_burrito.clone(),
                players: info.players.clone(),
                rewards: info.rewards.clone()
            };

            return active_incursion;
        }
    }

    // ELIMINAR TODAS LAS INCURSIONES CREADAS
    pub fn delete_all_incursions(&mut self){
        self.assert_owner();
        self.incursions.clear();
        self.mb_vs_bp.clear();
        self.player_incursion.clear();
        self.last_id = 0;
    }

    // Comenzar incursión
    pub fn start_active_incursion(&mut self) -> Incursion{
        self.assert_owner();
        let filtered_incursions : HashMap<u64,Incursion> = self.incursions.clone()
        .into_iter()
        .filter(|(_, v)| 
            (v.status == IncursionStatus::WaitingPlayers))
        .collect();

        if filtered_incursions.len() == 0 {
            let actual_epoch = env::block_timestamp();
            let null_incursion = Incursion{
                id: 0,
                status: IncursionStatus::Null,
                create_time: actual_epoch,
                start_time: actual_epoch+43200000000000,
                finish_time: actual_epoch+86400000000000,
                players_number: 0,
                registered_players: 0,
                win: "".to_string(),
                mega_burrito: MegaBurrito {
                    name : "".to_string(),
                    burrito_type : "".to_string(),
                    start_health : "".to_string(),
                    health : "".to_string(),
                    attack : "".to_string(),
                    defense : "".to_string(),
                    speed : "".to_string(),
                    level : "".to_string()
                },
                players: [].to_vec(),
                rewards: 0
            };

            return null_incursion;
        } else {
            let mut key : u64 = 0;

            for (k, v) in filtered_incursions.iter() {
                key = k.clone();
            }

            let info = filtered_incursions.get(&key).unwrap();

            let updated_incursion = Incursion{
                id: info.id.clone(),
                status: IncursionStatus::InProgress,
                create_time: info.create_time.clone(),
                start_time: info.start_time.clone(),
                finish_time: info.finish_time.clone(),
                players_number: info.players_number.clone(),
                registered_players: info.registered_players.clone(),
                win: info.win.clone(),
                mega_burrito: info.mega_burrito.clone(),
                players: info.players.clone(),
                rewards: info.rewards.clone()
            };

            self.incursions.insert(key,updated_incursion.clone());

            return updated_incursion;
        }
    }

    // Finalizar incursión
    pub fn finish_active_incursion(&mut self) -> Incursion{
        self.assert_owner();
        let filtered_incursions : HashMap<u64,Incursion> = self.incursions.clone()
        .into_iter()
        .filter(|(_, v)| 
            (v.status == IncursionStatus::InProgress))
        .collect();

        if filtered_incursions.len() == 0 {
            let actual_epoch = env::block_timestamp();
            let null_incursion = Incursion{
                id: 0,
                status: IncursionStatus::Null,
                create_time: actual_epoch,
                start_time: actual_epoch+43200000000000,
                finish_time: actual_epoch+86400000000000,
                players_number: 0,
                registered_players: 0,
                win: "".to_string(),
                mega_burrito: MegaBurrito {
                    name : "".to_string(),
                    burrito_type : "".to_string(),
                    start_health : "".to_string(),
                    health : "".to_string(),
                    attack : "".to_string(),
                    defense : "".to_string(),
                    speed : "".to_string(),
                    level : "".to_string()
                },
                players: [].to_vec(),
                rewards: 0
            };

            return null_incursion;
        } else {
            let mut key : u64 = 0;

            for (k, v) in filtered_incursions.iter() {
                key = k.clone();
            }

            let info = filtered_incursions.get(&key).unwrap();

            let updated_incursion = Incursion{
                id: info.id.clone(),
                status: IncursionStatus::Finished,
                create_time: info.create_time.clone(),
                start_time: info.start_time.clone(),
                finish_time: info.finish_time.clone(),
                players_number: info.players_number.clone(),
                registered_players: info.registered_players.clone(),
                win: info.win.clone(),
                mega_burrito: info.mega_burrito.clone(),
                players: info.players.clone(),
                rewards: info.rewards.clone()
            };

            self.incursions.insert(key,updated_incursion.clone());

            return updated_incursion;
        }
    }

    // Registrarme en incursión
    pub fn nft_on_transfer(&mut self,sender_id: AccountId,previous_owner_id: AccountId,token_id: String,msg: String)  -> PromiseOrValue<bool>{
        let contract_id = env::predecessor_account_id();
        let signer_id = env::signer_account_id();
        let msg_json: MsgInput = from_str(&msg).unwrap();
        let incursion_id = msg_json.incursion_id.clone();
        let player = previous_owner_id.clone();

        let new_player = BurritoPlayer {
            burrito_id: token_id.clone().to_string(),
            burrito_owner: player.clone(),
        };

        log!("Incursion: {}",incursion_id.clone());

        let exist_incursion = self.incursions.get(&incursion_id);
        
        if exist_incursion.is_none() {
            log!("No hay ninguna incursión activa");
            return near_sdk::PromiseOrValue::Value(true);
        }

        let info = self.incursions.get(&incursion_id).unwrap();

        let mut active_incursion = Incursion{
            id: info.id.clone(),
            status: info.status.clone(),
            create_time: info.create_time.clone(),
            start_time: info.start_time.clone(),
            finish_time: info.finish_time.clone(),
            players_number: info.players_number.clone(),
            registered_players: info.registered_players.clone(),
            win: info.win.clone(),
            mega_burrito: info.mega_burrito.clone(),
            players: info.players.clone(),
            rewards: info.rewards.clone()
        };

        if active_incursion.registered_players.clone() == active_incursion.players_number.clone(){
            log!("La incursión ya está llena");
            return near_sdk::PromiseOrValue::Value(true);
        }

        let exist_player: Vec<BurritoPlayer> = active_incursion
        .players.clone()
        .into_iter()
        .filter(|p| p.burrito_owner == player)
        .collect();

        if exist_player.len() > 0{
            log!("Ya te encuentras registrado en esta incursión");
            return near_sdk::PromiseOrValue::Value(true);
        }

        // Consultar información del burrito
        let call = ext_nft::get_burrito_incursion(
            token_id.clone().to_string(),
            self.burrito_contract.parse::<AccountId>().unwrap(),
            NO_DEPOSIT,
            Gas(100_000_000_000_000)
        );

        let callback = ext_self::get_burrito_info(
            active_incursion.clone(),
            new_player.clone(),
            contract_id.to_string(),
            self.incursion_contract.parse::<AccountId>().unwrap(),
            NO_DEPOSIT,
            Gas(100_000_000_000_000)
        );

        return near_sdk::PromiseOrValue::Promise(call.then(callback));  

    }

    // Recuperar información de los burritos y guardarla en la sala de batalla
    pub fn get_burrito_info(&mut self,incursion:Incursion, player:BurritoPlayer, contract_id:String) -> PromiseOrValue<bool> {
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

                // Si el burrito enviado no tiene vida entonces se cancela la transferencia del burrito y el registro a la incursión
                if burrito.hp.parse::<u8>().unwrap() <= 0 {
                    log!("El burrito a utilizar no tiene vidas");
                    return near_sdk::PromiseOrValue::Value(true);
                }
                
                near_sdk::PromiseOrValue::Promise(
                    ext_nft::get_balance_and_transfer_minigames(
                        player.burrito_owner.clone().to_string(),
                        "Incursion".to_string(),
                        self.treasury_id.to_string(),
                        self.strw_contract.parse::<AccountId>().unwrap(),
                        NO_DEPOSIT,
                        Gas(60_000_000_000_000)
                    ).then(ext_self::register_player_incursion(
                        incursion.clone(),
                        player.clone(),
                        contract_id.to_string(),
                        self.incursion_contract.parse::<AccountId>().unwrap(),
                        NO_DEPOSIT,
                        Gas(20_000_000_000_000)
                    ))
                )
            }
        }
    }

    pub fn register_player_incursion(&mut self,incursion:Incursion, player:BurritoPlayer, contract_id:String) -> bool {        
        let new_player = player;

        let info = self.incursions.get(&incursion.clone().id).unwrap();

        let mut active_incursion = Incursion{
            id: info.id.clone(),
            status: info.status.clone(),
            create_time: info.create_time.clone(),
            start_time: info.start_time.clone(),
            finish_time: info.finish_time.clone(),
            players_number: info.players_number.clone(),
            registered_players: info.registered_players.clone(),
            win: info.win.clone(),
            mega_burrito: info.mega_burrito.clone(),
            players: info.players.clone(),
            rewards: info.rewards.clone()
        };

        let mut players: Vec<BurritoPlayer> = active_incursion.players.clone();
        players.push(new_player.clone());

        active_incursion.players = players;
        active_incursion.registered_players += 1;

        self.incursions.insert(active_incursion.id.clone(),active_incursion.clone());

        let player = Player {
            incursion_id: active_incursion.id.clone(),
            burrito_id: new_player.burrito_id.clone(),
            burrito_owner: new_player.burrito_owner.clone().to_string(),
            burrito_contract: contract_id.clone()
        };

        self.player_incursion.insert(new_player.burrito_owner.clone(),player);

        //return true; // Regresar token al jugador
        return false; // No regresar el token al jugador
    }

    // Obtener la incursión del jugador
    pub fn get_player_incursion(&mut self) -> IncrusionPlayer{
        let player_id = env::signer_account_id();
        let player = self.player_incursion.get(&player_id);

        if player.is_none() {
            log!("No te encuentras registrado en ninguna incursión");
            let actual_epoch = env::block_timestamp();
            let null_incursion = Incursion{
                id: 0,
                status: IncursionStatus::Null,
                create_time: actual_epoch,
                start_time: actual_epoch+3600000000000,
                finish_time: actual_epoch+7200000000000,
                players_number: 0,
                registered_players: 0,
                win: "".to_string(),
                mega_burrito: MegaBurrito {
                    name : "".to_string(),
                    burrito_type : "".to_string(),
                    start_health : "".to_string(),
                    health : "".to_string(),
                    attack : "".to_string(),
                    defense : "".to_string(),
                    speed : "".to_string(),
                    level : "".to_string()
                },
                players: [].to_vec(),
                rewards: 0
            };
            let null_player = Player {
                incursion_id: 0,
                burrito_id: "".to_string(),
                burrito_owner: "".to_string(),
                burrito_contract: "".to_string()
            };

            let player_null_incursion = IncrusionPlayer {
                incursion: null_incursion,
                player: null_player
            };

            return player_null_incursion;
        } else {
            let info = self.incursions.get(&player.clone().unwrap().incursion_id).unwrap();
            let player_info = player.clone().unwrap();
            let player_incursion = Player{
                incursion_id: player_info.incursion_id.clone(),
                burrito_id: player_info.burrito_id.clone(),
                burrito_owner: player_info.burrito_owner.clone(),
                burrito_contract: player_info.burrito_contract.clone()
            };

            let incursion = Incursion{
                id: info.id.clone(),
                status: info.status.clone(),
                create_time: info.create_time.clone(),
                start_time: info.start_time.clone(),
                finish_time: info.finish_time.clone(),
                players_number: info.players_number.clone(),
                registered_players: info.registered_players.clone(),
                win: info.win.clone(),
                mega_burrito: info.mega_burrito.clone(),
                players: info.players.clone(),
                rewards: info.rewards.clone()
            };

            let player_info = player.clone().unwrap();
            let player_incursion = Player{
                incursion_id: player_info.incursion_id.clone(),
                burrito_id: player_info.burrito_id.clone(),
                burrito_owner: player_info.burrito_owner.clone(),
                burrito_contract: player_info.burrito_contract.clone()
            };

            let player_incursion = IncrusionPlayer {
                incursion: incursion,
                player: player_incursion
            };

            return player_incursion;
        }
    }

    // Crear sala de batalla jugador vs mega burrito
    pub fn create_battle_room(&mut self) -> PromiseOrValue<BPvsMB>{
        let player_id = env::predecessor_account_id();
        let player = self.player_incursion.get(&player_id.clone());

        if player.is_none() {
            log!("No te encuentras registrado en ninguna incursión");
            let actual_epoch = env::block_timestamp();
            let null_room = BPvsMB{
                turn : "".to_string(),
                burrito_player_id : "".to_string(),
                name : "".to_string(),
                burrito_type : "".to_string(),
                start_health : "".to_string(),
                health : "".to_string(),
                hp : "".to_string(),
                attack : "".to_string(),
                defense : "".to_string(),
                speed : "".to_string(),
                level : "".to_string(),
                media : "".to_string(),
                strong_attack_player : "".to_string(),
                shields_player : "".to_string(),
                incursion_id : 0,
                strong_attack_cpu : "".to_string(),
                shields_cpu : "".to_string(),
                damage_player : 0.0
            };
            return near_sdk::PromiseOrValue::Value(null_room);        
        } else {
            // Verificar si ya tiene una sala creada
            let room = self.mb_vs_bp.get(&player_id);
            
            if room.is_none() {
                log!("Creando sala de batalla");

                let incursion = self.incursions.get(&player.clone().unwrap().incursion_id).unwrap();

                // Verificar que ya comenzó la incursion
                let actual_epoch = env::block_timestamp();

                if incursion.mega_burrito.health.parse::<f32>().unwrap() <= 0.0 {
                    env::panic_str("El megaburrito ya fue vencido");       
                }

                if actual_epoch < incursion.start_time {
                    env::panic_str("Aún no inicia la incursión");       
                }

                if actual_epoch > incursion.finish_time {
                    env::panic_str("Ya terminó la incursión");       
                }

                let player_info = player.clone().unwrap();
                let player_incursion = Player{
                    incursion_id: player_info.incursion_id.clone(),
                    burrito_id: player_info.burrito_id.clone(),
                    burrito_owner: player_info.burrito_owner.clone(),
                    burrito_contract: player_info.burrito_contract.clone()
                };

                let call = ext_nft::get_burrito_incursion(
                    player.clone().unwrap().burrito_id.to_string(),
                    self.burrito_contract.parse::<AccountId>().unwrap(),
                    NO_DEPOSIT,
                    Gas(100_000_000_000_000)
                );
        
                let callback = ext_self::save_battle_room(
                    player_incursion.clone(),
                    incursion.clone(),
                    player_id.clone(),
                    self.incursion_contract.parse::<AccountId>().unwrap(),
                    NO_DEPOSIT,
                    Gas(100_000_000_000_000)
                );
                return near_sdk::PromiseOrValue::Promise(call.then(callback));  
            } else {
                env::panic_str("Ya tienes una incursion iniciada, debes terminarla");       
            }          
        }
    }

    // Recuperar información de los burritos y guardarla en la sala de batalla
    pub fn save_battle_room(&mut self, player: Player, incrusion: Incursion, player_id: AccountId) -> BPvsMB {
        assert_eq!(
            env::promise_results_count(),
            1,
            "Éste es un método callback"
        );
        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Failed => {
                let null_room = BPvsMB{
                    turn : "".to_string(),
                    burrito_player_id : "".to_string(),
                    name : "".to_string(),
                    burrito_type : "".to_string(),
                    start_health : "".to_string(),
                    health : "".to_string(),
                    hp : "".to_string(),
                    attack : "".to_string(),
                    defense : "".to_string(),
                    speed : "".to_string(),
                    level : "".to_string(),
                    media : "".to_string(),
                    strong_attack_player : "".to_string(),
                    shields_player : "".to_string(),
                    incursion_id : 0,
                    strong_attack_cpu : "".to_string(),
                    shields_cpu : "".to_string(),
                    damage_player : 0.0
                };
                return null_room;
            },
            PromiseResult::Successful(result) => {
                env::log(
                    json!(incrusion)
                    .to_string()
                    .as_bytes(),
                );
                let value = std::str::from_utf8(&result).unwrap();
                let burrito_for_battle: Burrito = serde_json::from_str(&value).unwrap();

                let mut room = BPvsMB{
                    turn : "".to_string(),
                    burrito_player_id : player.clone().burrito_id,
                    name : burrito_for_battle.name.clone(),
                    burrito_type : burrito_for_battle.burrito_type.clone(),
                    start_health : (burrito_for_battle.attack.clone().parse::<u8>().unwrap()+burrito_for_battle.defense.clone().parse::<u8>().unwrap()+burrito_for_battle.speed.clone().parse::<u8>().unwrap()).to_string(),
                    health : (burrito_for_battle.attack.clone().parse::<u8>().unwrap()+burrito_for_battle.defense.clone().parse::<u8>().unwrap()+burrito_for_battle.speed.clone().parse::<u8>().unwrap()).to_string(),
                    hp : burrito_for_battle.hp.clone(),
                    attack : burrito_for_battle.attack.clone(),
                    defense : burrito_for_battle.defense.clone(),
                    speed : burrito_for_battle.speed.clone(),
                    level : burrito_for_battle.level.clone(),
                    media : burrito_for_battle.media.clone(),
                    strong_attack_player : "3".to_string(),
                    shields_player : "3".to_string(),
                    incursion_id : incrusion.id.clone(),
                    strong_attack_cpu : "3".to_string(),
                    shields_cpu : "3".to_string(),
                    damage_player : 0.0
                };

                if incrusion.mega_burrito.speed.parse::<u8>().unwrap() > burrito_for_battle.speed.clone().parse::<u8>().unwrap() {
                    room.turn = "CPU".to_string();
                } else {
                    room.turn = "Player".to_string();
                }
                self.mb_vs_bp.insert(player_id,room.clone());

                return room;
            }
        }
    }

    // Verificar si tiene una sala activa en la incursion
    pub fn is_in_battle_incursion(&self, account_id : AccountId) -> bool {
        let token_owner_id = account_id.clone();

        let br = self.mb_vs_bp.get(&token_owner_id);

        if br.is_none() {
            return false;
        } else {
            return true;
        }
    }

    // Obtener sala de batalla activa de jugador
    pub fn get_active_battle_room(&self) -> IncrusionRound{
        let player_id = env::predecessor_account_id();
        let room = self.mb_vs_bp.get(&player_id);

        if room.is_none() {
            env::panic_str("No existe sala creada de esta cuenta");
        } else {
            let info = room.unwrap();

            let room = BPvsMB{
                turn : info.turn.to_string(),
                burrito_player_id : info.burrito_player_id.to_string(),
                name : info.name.to_string(),
                burrito_type : info.burrito_type.to_string(),
                start_health : info.start_health.to_string(),
                health : info.health.to_string(),
                hp : info.hp.to_string(),
                attack : info.attack.to_string(),
                defense : info.defense.to_string(),
                speed : info.speed.to_string(),
                level : info.level.to_string(),
                media : info.media.to_string(),
                strong_attack_player : info.strong_attack_player.to_string(),
                shields_player : info.shields_player.to_string(),
                incursion_id : info.incursion_id.to_string().parse::<u64>().unwrap(),
                strong_attack_cpu : info.strong_attack_cpu.to_string(),
                shields_cpu : info.shields_cpu.to_string(),
                damage_player : info.damage_player.to_string().parse::<f32>().unwrap(),
            };

            let incursion = self.incursions.get(&info.incursion_id.to_string().parse::<u64>().unwrap()).unwrap();

            let mut incursion_info = Incursion{
                id: incursion.id.clone(),
                status: incursion.status.clone(),
                create_time: incursion.create_time.clone(),
                start_time: incursion.start_time.clone(),
                finish_time: incursion.finish_time.clone(),
                players_number: incursion.players_number.clone(),
                registered_players: incursion.registered_players.clone(),
                win: incursion.win.clone(),
                mega_burrito: incursion.mega_burrito.clone(),
                players: incursion.players.clone(),
                rewards: incursion.rewards.clone()
            };

            let incursion_round = IncrusionRound {
                incursion: incursion_info,
                room: room
            };
    
            return incursion_round;
        }
    }
     
    // Método combate de incursion (type_move 1 = Ataque Debil, 2 = Ataque Fuerte, 3 = No Defenderse 4 = Defenderse)
    pub fn battle_player_incursion(&mut self, type_move: String) -> IncrusionRound {
        let token_owner_id = env::signer_account_id();

        let br = self.mb_vs_bp.get(&token_owner_id.clone());
        
        if br.is_none() {
            env::panic_str("No tienes una batalla activa");
        }

        let info = br.unwrap();
        
        let battle_room = BPvsMB {
            turn : info.turn.to_string(),
            burrito_player_id : info.burrito_player_id.clone().to_string(),
            name : info.name.to_string(),
            burrito_type : info.burrito_type.to_string(),
            start_health : info.start_health.to_string(),
            health : info.health.to_string(),
            hp : info.hp.to_string(),
            attack : info.attack.to_string(),
            defense : info.defense.to_string(),
            speed : info.speed.to_string(),
            level : info.level.to_string(),
            media : info.media.to_string(),
            strong_attack_player : info.strong_attack_player.to_string(),
            shields_player : info.shields_player.to_string(),
            incursion_id : info.incursion_id.to_string().parse::<u64>().unwrap(),
            strong_attack_cpu : info.strong_attack_cpu.to_string(),
            shields_cpu : info.shields_cpu.to_string(),
            damage_player : info.damage_player.to_string().parse::<f32>().unwrap()
        };

        // Obtenemos la información de la incursión
        let incursion = self.incursions.get(&info.incursion_id.to_string().parse::<u64>().unwrap()).unwrap();

        let mut incursion_info = Incursion{
            id: incursion.id.clone(),
            status: incursion.status.clone(),
            create_time: incursion.create_time.clone(),
            start_time: incursion.start_time.clone(),
            finish_time: incursion.finish_time.clone(),
            players_number: incursion.players_number.clone(),
            registered_players: incursion.registered_players.clone(),
            win: incursion.win.clone(),
            mega_burrito: incursion.mega_burrito.clone(),
            players: incursion.players.clone(),
            rewards: incursion.rewards.clone()
        };

        // Verificar si el tiempo de la incursion no ah terminado
        let actual_epoch = env::block_timestamp();
        if actual_epoch > incursion_info.finish_time.clone() {
            let incursion_round = IncrusionRound {
                incursion: incursion_info,
                room: battle_room.clone()
            };
    
            return incursion_round;
           // env::panic_str("La incursion ya terminó, no puedes realizar ataques");       
        }

        if (type_move == "1" || type_move == "2") && battle_room.turn == "CPU"{
            env::panic_str("No puedes realizar un ataque, debes elegir si defenderte o no");
        }

        if (type_move == "3" || type_move == "4") && battle_room.turn == "Player"{
            env::panic_str("No puedes defenderte, debes realizar un ataque");
        }

        if type_move == "2" && battle_room.strong_attack_player.parse::<u8>().unwrap() == 0 {
            env::panic_str("No tienes mas ataques fuertes, debes realizar uno normal");
        }

        if type_move == "4" && battle_room.shields_player.parse::<u8>().unwrap() == 0 {
            env::panic_str("No tienes mas escudos, no puedes defenderte");
        }

        let mut old_battle_room = battle_room;
        let mut cpu_type_move = "1";

        // Verificar si el megaburrito aún sigue con vida
        if incursion_info.mega_burrito.health.parse::<f32>().unwrap() <= 0.0 {
            env::log(
                json!(old_battle_room)
                .to_string()
                .as_bytes(),
            );

            let incursion_round = IncrusionRound {
                incursion: incursion_info,
                room: old_battle_room
            };
    
            return incursion_round;
        }

        // Verificar si se utilizo un escudo para finalizar la ronda
        if old_battle_room.turn == "Player"{
            if type_move == "2"{
                old_battle_room.strong_attack_player = (old_battle_room.strong_attack_player.parse::<u8>().unwrap()-1).to_string();
            }
            // Validar si el CPU aun tiene escudos y elegir aleatoriamente si utilizara uno o no
            if old_battle_room.shields_cpu.parse::<u8>().unwrap() > 0 {
                let use_shield: u8 = *env::random_seed().get(0).unwrap();
                if use_shield % 2 == 1 {
                    old_battle_room.shields_cpu = (old_battle_room.shields_cpu.parse::<u8>().unwrap()-1).to_string();
                    old_battle_room.turn = "CPU".to_string();
                    self.mb_vs_bp.insert(token_owner_id,old_battle_room.clone());
                    env::log(
                        json!(old_battle_room)
                        .to_string()
                        .as_bytes(),
                    );
                    
                    let incursion_round = IncrusionRound {
                        incursion: incursion_info,
                        room: old_battle_room
                    };
            
                    return incursion_round;
                }
            }
        } else {
            if old_battle_room.strong_attack_cpu.parse::<u8>().unwrap() > 0 {
                let use_strong_attack: u8 = *env::random_seed().get(0).unwrap();
                if old_battle_room.shields_player.parse::<u8>().unwrap() == 0 {
                    old_battle_room.strong_attack_cpu = (old_battle_room.strong_attack_cpu.parse::<u8>().unwrap()-1).to_string();
                    cpu_type_move = "2";
                } else {
                    if use_strong_attack % 2 == 1 {
                        old_battle_room.strong_attack_cpu = (old_battle_room.strong_attack_cpu.parse::<u8>().unwrap()-1).to_string();
                        cpu_type_move = "2";
                    }
                }
            }
            if type_move == "4"{
                old_battle_room.shields_player = (old_battle_room.shields_player.parse::<u8>().unwrap()-1).to_string();
                old_battle_room.turn = "Player".to_string();
                self.mb_vs_bp.insert(token_owner_id,old_battle_room.clone());
                env::log(
                    json!(old_battle_room)
                    .to_string()
                    .as_bytes(),
                );

                let incursion_round = IncrusionRound {
                    incursion: incursion_info,
                    room: old_battle_room
                };
        
                return incursion_round;
            }
        }
      
        // Crear estructura burrito
        let burrito = Burrito {
            owner_id : old_battle_room.burrito_player_id.clone(),
            name : old_battle_room.name.clone(),
            description : "".to_string(),
            burrito_type : old_battle_room.burrito_type.clone(),
            hp : old_battle_room.hp.clone(),
            attack : old_battle_room.attack.clone(),
            defense : old_battle_room.defense.clone(),
            speed : old_battle_room.speed.clone(),
            win : "".to_string(),
            global_win : "".to_string(),
            level : old_battle_room.level.clone(),
            media : old_battle_room.media.clone()
        };

        // Crear estructura burrito cpu
        let burrito_cpu = MegaBurrito {
            name : incursion.mega_burrito.name.to_string(),
            burrito_type : incursion.mega_burrito.burrito_type.to_string(),
            attack : incursion.mega_burrito.attack.to_string(),
            defense : incursion.mega_burrito.defense.to_string(),
            speed : incursion.mega_burrito.speed.to_string(),
            level : incursion.mega_burrito.level.to_string(),
            start_health : incursion.mega_burrito.start_health.to_string(),
            health : incursion.mega_burrito.health.to_string()
        };

        // Calculos de daño
        let rand_attack: u8 = *env::random_seed().get(0).unwrap();
        let mut attack_mult: f32 = 0.0;
        let mut type_mult: f32 = 0.0;

        let burrito_attacker;
        let burrito_defender;
        let mut old_health_burrito_defender: f32 = 0.0;

        if old_battle_room.turn == "Player"{
            burrito_attacker = BurritoAttacker {
                burrito_type : burrito.burrito_type.clone(),
                attack : burrito.attack.clone(),
                level : burrito.level.clone()
            };
            burrito_defender = BurritoDefender {
                burrito_type : burrito_cpu.burrito_type.clone(),
                level : burrito_cpu.level.clone()
            };
            old_health_burrito_defender = burrito_cpu.health.parse::<f32>().unwrap();
        } else {
            burrito_attacker = BurritoAttacker {
                burrito_type : burrito_cpu.burrito_type.clone(),
                attack : burrito_cpu.attack.clone(),
                level : burrito_cpu.level.clone()
            };
            burrito_defender = BurritoDefender {
                burrito_type : burrito.burrito_type.clone(),
                level : burrito.level.clone()
            };
            old_health_burrito_defender = old_battle_room.health.clone().parse::<f32>().unwrap();
        }

        if rand_attack < 10 {
            attack_mult = rand_attack as f32 * 0.1;
        }
        if rand_attack >= 10 && rand_attack < 100 {
            attack_mult = rand_attack as f32 * 0.01;
        }
        if rand_attack >= 100 && rand_attack < 255 {
            attack_mult = rand_attack as f32 * 0.001;
        }
        if burrito_attacker.burrito_type == "Fuego" && burrito_defender.burrito_type == "Planta"{
            type_mult = (burrito_attacker.attack.parse::<f32>().unwrap()*attack_mult)*0.25
        }
        if burrito_attacker.burrito_type == "Agua" && burrito_defender.burrito_type == "Fuego"{
            type_mult = (burrito_attacker.attack.parse::<f32>().unwrap()*attack_mult)*0.25
        }
        if burrito_attacker.burrito_type == "Planta" && burrito_defender.burrito_type == "Eléctrico"{
            type_mult = (burrito_attacker.attack.parse::<f32>().unwrap()*attack_mult)*0.25
        }
        if burrito_attacker.burrito_type == "Eléctrico" && burrito_defender.burrito_type == "Volador"{
            type_mult = (burrito_attacker.attack.parse::<f32>().unwrap()*attack_mult)*0.25
        }
        if burrito_attacker.burrito_type == "Volador" && burrito_defender.burrito_type == "Agua"{
            type_mult = (burrito_attacker.attack.parse::<f32>().unwrap()*attack_mult)*0.25
        }

        let mut attack = 0.0;
        if old_battle_room.turn == "Player"{
            attack = (burrito_attacker.attack.parse::<f32>().unwrap()*attack_mult)+type_mult;
        } else {
            attack = (burrito_attacker.attack.parse::<f32>().unwrap()*attack_mult)+type_mult;
        }

        if attack < 1.0 {
            attack += 2.0;
        }

        // Verificar el tipo de ataque
        if old_battle_room.turn == "Player"{
            attack += 3.0;
            if type_move == "2"{
                attack = attack*2.0;
            }
        } else {
            attack += 3.0;
            if cpu_type_move == "2"{
                attack = attack*2.0;
            }
        }
        attack = (attack * 100.0).round() / 100.0;
        let new_health_burrito_defender = old_health_burrito_defender - attack;
        
        // Actualizar registro de sala de batalla
        if old_battle_room.turn == "Player"{
            if new_health_burrito_defender <= 0.0 {
                //old_battle_room.health_cpu = new_health_burrito_defender.to_string();
                let actual_epoch = env::block_timestamp();
                incursion_info.mega_burrito.health = new_health_burrito_defender.to_string();
                incursion_info.finish_time = actual_epoch;
                self.incursions.insert(info.incursion_id.to_string().parse::<u64>().unwrap(),incursion_info.clone());
                old_battle_room.damage_player += attack;
                self.mb_vs_bp.insert(token_owner_id,old_battle_room.clone());

                
                env::log(
                    json!(old_battle_room)
                    .to_string()
                    .as_bytes(),
                );

                let incursion_round = IncrusionRound {
                    incursion: incursion_info,
                    room: old_battle_room
                };
        
                return incursion_round;

            } else {
                incursion_info.mega_burrito.health = new_health_burrito_defender.to_string();
                self.incursions.insert(info.incursion_id.to_string().parse::<u64>().unwrap(),incursion_info.clone());

                old_battle_room.turn = "CPU".to_string();
                old_battle_room.damage_player += attack;
                self.mb_vs_bp.insert(token_owner_id,old_battle_room.clone());
            }
        } else {
            if new_health_burrito_defender <= 0.0 {
                // Guardar registro general de la batalla (Jugador, Burrito, Estatus)
                old_battle_room.health = new_health_burrito_defender.to_string();                
                self.mb_vs_bp.insert(token_owner_id.clone(),old_battle_room.clone());

                env::log(
                    json!(old_battle_room)
                    .to_string()
                    .as_bytes(),
                );

                let incursion_round = IncrusionRound {
                    incursion: incursion_info,
                    room: old_battle_room
                };
        
                return incursion_round;
            } else {
                old_battle_room.health = new_health_burrito_defender.to_string();
                old_battle_room.turn = "Player".to_string();
                self.mb_vs_bp.insert(token_owner_id,old_battle_room.clone());
            }                
        }
        
        env::log(
            json!(old_battle_room)
            .to_string()
            .as_bytes(),
        );

        let incursion_round = IncrusionRound {
            incursion: incursion_info,
            room: old_battle_room
        };

        return incursion_round;
    }

    pub fn can_withdraw_burrito(&self, account_id : AccountId) -> bool {
        let token_owner_id = account_id.clone();

        let br = self.player_incursion.get(&token_owner_id);

        if br.is_none() {
            return false;
        } else {
            return true;
        }
    }
        
    pub fn burritos_incursion_info(&self, incursion_id : u64) -> Vec<PlayerInBattle> {
        let exist_incursion = self.incursions.get(&incursion_id);
        let mut players: Vec<PlayerInBattle> = [].to_vec();

        if exist_incursion.is_none() {
            log!("No existe la incursion a buscar");
            return players;
        }

        let info = self.incursions.get(&incursion_id).unwrap();

        let mut incursion = Incursion{
            id: info.id.clone(),
            status: info.status.clone(),
            create_time: info.create_time.clone(),
            start_time: info.start_time.clone(),
            finish_time: info.finish_time.clone(),
            players_number: info.players_number.clone(),
            registered_players: info.registered_players.clone(),
            win: info.win.clone(),
            mega_burrito: info.mega_burrito.clone(),
            players: info.players.clone(),
            rewards: info.rewards.clone()
        };

        for x in incursion.players {
            let room = self.mb_vs_bp.get(&x.burrito_owner);

            if room.is_none() {
                // No existe sala creada de jugador, por lo tanto aun sigue con vida el burrito
                // let player = PlayerInBattle {
                //     player_name: x.burrito_owner.to_string(),
                //     media: "".to_string(),
                //     is_alive: true 
                // };
                // players.push(player.clone());
            } else {
                // Existe sala de batalla por lo tanto podemos verificar si el burrito aún tiene vida o no
                let info = room.unwrap();
                let player = PlayerInBattle {
                    player_name: x.burrito_owner.to_string(),
                    media: info.media.to_string(),
                    is_alive: if info.health.to_string().parse::<f32>().unwrap() >= 0.0  { true } else { false } 
                };
                players.push(player.clone());
            }
        }

        return players;

    }

    // Recuperar burrito
    #[payable]
    pub fn withdraw_burrito_owner(&mut self) -> Rewards{
        let signer_id = env::signer_account_id();
        let deposit = env::attached_deposit();
        let player = self.player_incursion.get(&signer_id.clone());
        let room = self.mb_vs_bp.get(&signer_id.clone());

        if player.is_none() {
            log!("No te encuentras registrado en ninguna incursión");
            let rewards = Rewards{
                complete : false,
                win : "".to_string(),
                msg : "No te encuentras registrado en ninguna incursión".to_string(),
                rewards : 0.0
            };
            return rewards;
        }

        let incursion_player = player.clone().unwrap();

        if signer_id.clone() != incursion_player.burrito_owner.clone().parse::<AccountId>().unwrap(){
            log!("No eres el dueño del burrito");
            let rewards = Rewards{
                complete : false,
                win : "".to_string(),
                msg : "No eres el dueño del burrito".to_string(),
                rewards : 0.0
            };
            return rewards;
        }

        // Obtener información de la incursión
        let info_incursion = self.incursions.get(&player.clone().unwrap().incursion_id).unwrap();
        let incursion = Incursion{
            id: info_incursion.id.clone(),
            status: info_incursion.status.clone(),
            create_time: info_incursion.create_time.clone(),
            start_time: info_incursion.start_time.clone(),
            finish_time: info_incursion.finish_time.clone(),
            players_number: info_incursion.players_number.clone(),
            registered_players: info_incursion.registered_players.clone(),
            win: info_incursion.win.clone(),
            mega_burrito: info_incursion.mega_burrito.clone(),
            players: info_incursion.players.clone(),
            rewards: info_incursion.rewards.clone()
        };

        // Verificar que la incursión ya terminó
        let actual_epoch = env::block_timestamp();
        if actual_epoch < incursion.finish_time {
            log!("Aún no termina la incursión");
            let rewards = Rewards{
                complete : false,
                win : "".to_string(),
                msg : "Aún no termina la incursión".to_string(),
                rewards : 0.0
            };
            return rewards;    
        }

        // Si no existe sala creada solo se le regresará a su burrito
        if room.is_none() {
            log!("No existe sala creada de esta cuenta");
            // Recuperar al burrito
            ext_nft::nft_transfer(
                signer_id.clone(),
                incursion_player.burrito_id.clone(),
                incursion_player.burrito_contract.parse::<AccountId>().unwrap(),
                deposit,
                MIN_GAS_FOR_NFT_TRANSFER_CALL
            );

            let rewards = Rewards{
                complete : true,
                win : "".to_string(),
                msg : "No participaste en la batalla".to_string(),
                rewards : 0.0
            };
            return rewards;        
        }
             
        // Obtener información de la sala de batalla
        let info_room = room.unwrap();
        let room = BPvsMB {
            turn : info_room.turn.to_string(),
            burrito_player_id : info_room.burrito_player_id.to_string(),
            name : info_room.name.to_string(),
            burrito_type : info_room.burrito_type.to_string(),
            start_health : info_room.start_health.to_string(),
            health : info_room.health.to_string(),
            hp : info_room.hp.to_string(),
            attack : info_room.attack.to_string(),
            defense : info_room.defense.to_string(),
            speed : info_room.speed.to_string(),
            level : info_room.level.to_string(),
            media : info_room.media.to_string(),
            strong_attack_player : info_room.strong_attack_player.to_string(),
            shields_player : info_room.shields_player.to_string(),
            incursion_id : info_room.incursion_id.to_string().parse::<u64>().unwrap(),
            strong_attack_cpu : info_room.strong_attack_cpu.to_string(),
            shields_cpu : info_room.shields_cpu.to_string(),
            damage_player : info_room.damage_player.to_string().parse::<f32>().unwrap(),
        };

        // Generar recompensas 30% + Proporcional al daño realizado de la bolsa total
        let player_reward = ((incursion.rewards.clone()/100).to_string().parse::<f32>().unwrap()*room.damage_player)+3000.0;
        let tokens_to_mint = player_reward.clone()*1000000000000000000000000.0;

        if incursion.mega_burrito.health.parse::<f32>().unwrap() > 0.0 {
            log!("No vencieron al mega burrito");

            ext_nft::decrease_all_burrito_hp(
                incursion_player.burrito_id.clone(),
                self.burrito_contract.parse::<AccountId>().unwrap(),
                NO_DEPOSIT,
                GAS_FOR_NFT_TRANSFER_CALL
            ).then(ext_nft::nft_transfer( // Restar una vida al burrito
                signer_id.clone(),
                incursion_player.burrito_id.clone(),
                incursion_player.burrito_contract.parse::<AccountId>().unwrap(),
                deposit,
                MIN_GAS_FOR_NFT_TRANSFER_CALL
            ));

            // Remover registro de burrito de jugador
            self.player_incursion.remove(&signer_id.clone());
            // Remover sala de batalla de jugador
            self.mb_vs_bp.remove(&signer_id.clone());

            let rewards = Rewards{
                complete : true,
                win : "MegaBurrito".to_string(),
                msg : "No vencieron al mega burrito".to_string(),
                rewards : 0.0
            };

            return rewards;
        } else {
            log!("Vencieron al mega burrito");
            // Recuperar al burrito
            ext_nft::nft_transfer(
                signer_id.clone(),
                incursion_player.burrito_id.clone(),
                incursion_player.burrito_contract.parse::<AccountId>().unwrap(),
                deposit,
                MIN_GAS_FOR_NFT_TRANSFER_CALL
            ).then(ext_nft::reward_player( // Restar una vida al burrito
                signer_id.clone().to_string(),
                tokens_to_mint.to_string(),
                self.strw_contract.parse::<AccountId>().unwrap(),
                NO_DEPOSIT,
                MIN_GAS_FOR_NFT_TRANSFER_CALL
            ));

            // Remover registro de burrito de jugador
            self.player_incursion.remove(&signer_id.clone());
            // Remover sala de batalla de jugador
            self.mb_vs_bp.remove(&signer_id.clone());

            let rewards = Rewards{
                complete : true,
                win : "Players".to_string(),
                msg : "Vencieron al mega burrito".to_string(),
                rewards : player_reward.clone()
            };

            return rewards;
        }
    }
   
}