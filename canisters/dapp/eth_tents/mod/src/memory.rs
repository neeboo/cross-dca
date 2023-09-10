use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    DefaultMemoryImpl, RestrictedMemory, StableBTreeMap, StableCell,
};

use crate::rpc::stable_state::StableState;
use crate::types::{IntentWalletConfig, UserProfile, UserWallet};
use std::cell::RefCell;

const USER_PROFILE_MEM_ID: MemoryId = MemoryId::new(0);
const USER_WALLET_MEM_ID: MemoryId = MemoryId::new(1);
const USER_INTENT_MEM_ID: MemoryId = MemoryId::new(2);
const METADATA_PAGES: u64 = 16;

const WALLET_CONFIG_PAGES: u64 = 128;

type RM = RestrictedMemory<DefaultMemoryImpl>;
type VM = VirtualMemory<RM>;

thread_local! {
    pub static CONFIG: RefCell<StableCell<StableState, RM>> = RefCell::new(StableCell::init(RM::new(DefaultMemoryImpl::default(), 0..METADATA_PAGES), StableState::default()).expect("failed to initialize the config cell"));

    static MEMORY_MANAGER: RefCell<MemoryManager<RM>> = RefCell::new(
        MemoryManager::init(RM::new(DefaultMemoryImpl::default(), METADATA_PAGES..1024))
    );

    pub static USERS: RefCell<StableBTreeMap<u16, UserProfile, VM>> = MEMORY_MANAGER.with(|mm| {
        RefCell::new(StableBTreeMap::init(mm.borrow().get(USER_PROFILE_MEM_ID)))
    });

    pub static WALLETS: RefCell<StableBTreeMap<u16, UserWallet, VM>> = MEMORY_MANAGER.with(|mm| {
        RefCell::new(StableBTreeMap::init(mm.borrow().get(USER_WALLET_MEM_ID)))
    });

    pub static WALLET_CONFIG: RefCell<StableCell<IntentWalletConfig, RM>> = RefCell::new(StableCell::init(RM::new(DefaultMemoryImpl::default(), METADATA_PAGES..WALLET_CONFIG_PAGES), OtcWalletConfig::default()).expect("failed to initialize the config cell"));

    pub static INTENTS: RefCell<StableBTreeMap<u16, UserWallet, VM>> = MEMORY_MANAGER.with(|mm| {
        RefCell::new(StableBTreeMap::init(mm.borrow().get(USER_INTENT_MEM_ID)))
    });
}
