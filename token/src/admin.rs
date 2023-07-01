use soroban_sdk::{Address, Env};

use crate::storage_types::DataKey;

pub fn has_administrator(e: &Env) -> bool {
    let key = DataKey::Admin;
    e.storage().instance().has(&key)
}

pub fn read_administrator(e: &Env) -> Address {
    let key = DataKey::Admin;
    e.storage().instance().get(&key).unwrap()
}

pub fn write_administrator(e: &Env, id: &Address) {
    let key = DataKey::Admin;
    e.storage().instance().set(&key, id);
}

pub fn write_whitelist(e: &Env, addr: Address) {
    let key = DataKey::Whitelist(addr);
    e.storage().instance().set(&key, &true);
}

pub fn write_blacklist(e: &Env, addr: Address) {
    let key = DataKey::Whitelist(addr);
    e.storage().instance().remove(&key);
}

pub fn is_whitelisted(e: &Env, addr: Address) -> bool {
    let key = DataKey::Whitelist(addr);
    e.storage().instance().has(&key)
}
