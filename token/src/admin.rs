use crate::storage_types::{
    DataKey, INSTANCE_BUMP_AMOUNT, INSTANCE_LIFETIME_THRESHOLD, LEDGER_BUMP_USER,
    LEDGER_THRESHOLD_USER,
};
use soroban_sdk::{token::StellarAssetClient as TokenAdminClient, Address, Env};

pub fn has_contract_admin(e: &Env) -> bool {
    let key = DataKey::ContractAdmin;
    e.storage()
        .instance()
        .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
    e.storage().instance().has(&key)
}

fn read_contract_admin(e: &Env) -> Address {
    let key = DataKey::ContractAdmin;
    e.storage()
        .instance()
        .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
    e.storage().instance().get(&key).unwrap()
}

pub fn write_contract_admin(e: &Env, id: &Address) {
    let key = DataKey::ContractAdmin;
    e.storage()
        .instance()
        .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
    e.storage().instance().set(&key, id);
}

pub fn write_token_address(e: &Env, token: &Address) {
    let key = DataKey::TokenAddress;
    e.storage()
        .instance()
        .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
    e.storage().instance().set(&key, token);
}

pub fn read_token_address(e: &Env) -> Address {
    let key = DataKey::TokenAddress;
    e.storage()
        .instance()
        .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
    e.storage().instance().get(&key).unwrap()
}

pub fn write_kyc(e: &Env, addr: Address) {
    let key = DataKey::Kyc(addr);
    e.storage().persistent().set(&key, &true);
    e.storage()
        .persistent()
        .extend_ttl(&key, LEDGER_THRESHOLD_USER, LEDGER_BUMP_USER);
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
            .extend_ttl(&key, LEDGER_THRESHOLD_USER, LEDGER_BUMP_USER);
        return val;
    }

    false
}

pub fn check_authorized(e: &Env, addr: Address) {
    let token = TokenAdminClient::new(e, &read_token_address(e));
    let authorized = token.authorized(&addr);
    if !authorized {
        panic!("address is not authorized");
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
        .extend_ttl(&key, LEDGER_THRESHOLD_USER, LEDGER_BUMP_USER);
}

pub fn check_not_blacklisted(e: &Env, addr: Address) {
    let key = DataKey::Blacklisted(addr);
    if let Some(val) = e.storage().persistent().get::<DataKey, bool>(&key) {
        e.storage()
            .persistent()
            .extend_ttl(&key, LEDGER_THRESHOLD_USER, LEDGER_BUMP_USER);
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
        .extend_ttl(&key, LEDGER_THRESHOLD_USER, LEDGER_BUMP_USER);
}

pub fn is_amm(e: &Env, addr: Address) -> bool {
    let key = DataKey::Amm(addr);
    if let Some(val) = e.storage().persistent().get(&key) {
        e.storage()
            .persistent()
            .extend_ttl(&key, LEDGER_THRESHOLD_USER, LEDGER_BUMP_USER);
        return val;
    }
    false
}

pub fn check_not_amm(e: &Env, addr: Address) {
    if is_amm(e, addr.clone()) {
        panic!("amm address not allowed")
    }
}

pub fn require_contract_admin(e: &Env) -> Address {
    let admin = read_contract_admin(e);
    admin.require_auth();
    admin
}

pub fn require_token_admin(e: &Env) -> Address {
    let token = read_token_address(e);
    read_token_address(e).require_auth();
    token
}
