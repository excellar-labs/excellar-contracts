use soroban_sdk::{Address, Env};

use crate::storage_types::{
    DataKey, BALANCE_BUMP_AMOUNT, BALANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT,
    INSTANCE_LIFETIME_THRESHOLD,
};

pub fn has_administrator(e: &Env) -> bool {
    let key = DataKey::Admin;
    e.storage()
        .instance()
        .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
    e.storage().instance().has(&key)
}

pub fn read_administrator(e: &Env) -> Address {
    let key = DataKey::Admin;
    e.storage()
        .instance()
        .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
    e.storage().instance().get(&key).unwrap()
}

pub fn write_administrator(e: &Env, id: &Address) {
    let key = DataKey::Admin;
    e.storage()
        .instance()
        .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
    e.storage().instance().set(&key, id);
}

pub fn write_kyc(e: &Env, addr: Address) {
    let key = DataKey::Kyc(addr);
    e.storage().persistent().set(&key, &true);
    e.storage()
        .persistent()
        .extend_ttl(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
}

pub fn remove_kyc(e: &Env, addr: Address) {
    let key = DataKey::Kyc(addr);
    e.storage().persistent().remove(&key);
}

pub fn is_kyc_passed(e: &Env, addr: Address) -> bool {
    let key = DataKey::Kyc(addr);
    if let Some(val) = e.storage().persistent().get(&key) {
        e.storage()
            .persistent()
            .extend_ttl(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
        return val;
    }

    false
}

pub fn check_kyc_passed(e: &Env, addr: Address) {
    let passed = is_kyc_passed(e, addr);
    if !passed {
        panic!("address is not passed kyc");
    }
}

pub fn remove_blacklist(e: &Env, addr: Address) {
    let key = DataKey::Blacklisted(addr);
    e.storage().persistent().remove(&key);
}

pub fn write_blacklist(e: &Env, addr: Address) {
    let key = DataKey::Blacklisted(addr);
    e.storage().persistent().set(&key, &true);
    e.storage()
        .persistent()
        .extend_ttl(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
}

pub fn check_not_blacklisted(e: &Env, addr: Address) {
    let key = DataKey::Blacklisted(addr);
    if let Some(val) = e.storage().persistent().get::<DataKey, bool>(&key) {
        e.storage()
            .persistent()
            .extend_ttl(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
        assert!(!val, "address is blacklisted");
    }
}
pub fn remove_amm(e: &Env, addr: Address) {
    let key = DataKey::Amm(addr);
    e.storage().persistent().remove(&key);
}

pub fn add_amm(e: &Env, addr: Address) {
    let key = DataKey::Amm(addr);
    e.storage().persistent().set(&key, &true);
    e.storage()
        .persistent()
        .extend_ttl(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
}

pub fn is_amm(e: &Env, addr: Address) -> bool {
    let key = DataKey::Amm(addr);
    if let Some(val) = e.storage().persistent().get(&key) {
        e.storage()
            .persistent()
            .extend_ttl(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
        return val;
    }
    false
}

pub fn check_not_amm(e: &Env, addr: Address) {
    if is_amm(e, addr.clone()) {
        panic!("amm address not allowed")
    }
}
pub fn require_admin(e: &Env) -> Address {
    let admin = read_administrator(e);
    admin.require_auth();
    admin
}
