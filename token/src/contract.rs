//! This contract demonstrates a sample implementation of the Soroban token
//! interface.
use soroban_sdk::token::{self, Interface as _};
use soroban_sdk::{contract, contractimpl, Address, BytesN, Env, String};
use soroban_token_sdk::metadata::TokenMetadata;
use soroban_token_sdk::TokenUtils;

use crate::admin::{
    check_kyc_passed, has_administrator, read_administrator, remove_blacklist, remove_kyc,
    require_admin, write_administrator, write_blacklist, write_kyc,
};
use crate::allowance::{read_allowance, spend_allowance, write_allowance};
use crate::balance::{read_balance, receive_balance, spend_balance, total_supply};
use crate::event::{blacklist_event, fail_kyc_event, pass_kyc_event, whitelist_event};
use crate::metadata::{read_decimal, read_name, read_symbol, write_metadata};
use crate::reward::{
    checkpoint_reward, read_reward, reset_reward, set_reward_rate, set_reward_tick,
};
#[cfg(test)]
use crate::storage_types::{AllowanceDataKey, AllowanceValue, DataKey};
use crate::storage_types::{INSTANCE_BUMP_AMOUNT, INSTANCE_LIFETIME_THRESHOLD};
use crate::validations::{pre_mint_burn_checks, pre_transfer_checks};

#[contract]
pub struct ExcellarToken;

#[contractimpl]
impl ExcellarToken {
    pub fn initialize(e: Env, admin: Address, decimal: u32, name: String, symbol: String) {
        if has_administrator(&e) {
            panic!("already initialized")
        }
        write_administrator(&e, &admin);
        if decimal > u8::MAX.into() {
            panic!("Decimal must fit in a u8");
        }

        write_metadata(
            &e,
            TokenMetadata {
                decimal,
                name,
                symbol,
            },
        );

        // &5
        set_reward_rate(&e, 1_00);
        // roughly the number of ledger advancements
        set_reward_tick(&e, 28_800);
    }

    pub fn mint(e: Env, to: Address, amount: i128) {
        pre_mint_burn_checks(&e, to.clone(), amount);
        let admin = require_admin(&e);

        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        checkpoint_reward(&e, to.clone());
        receive_balance(&e, to.clone(), amount);
        TokenUtils::new(&e).events().mint(admin, to, amount);
    }

    pub fn claim_reward(e: Env, to: Address) {
        to.require_auth();
        check_kyc_passed(&e, to.clone());

        let reward = read_reward(&e, to.clone());
        if reward < 1 {
            return;
        }
        reset_reward(&e, to.clone());
        checkpoint_reward(&e, to.clone());
        receive_balance(&e, to, reward);
    }

    pub fn admin_claim_reward(e: Env, to: Address) {
        require_admin(&e);
        let reward = read_reward(&e, to.clone());
        if reward < 1 {
            return;
        }
        reset_reward(&e, to.clone());
        checkpoint_reward(&e, to.clone());
        receive_balance(&e, to, reward);
    }

    pub fn set_admin(e: Env, new_admin: Address) {
        let admin = require_admin(&e);

        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        write_administrator(&e, &new_admin);
        TokenUtils::new(&e).events().set_admin(admin, new_admin);
    }

    pub fn fail_kyc(e: Env, addr: Address) {
        let admin = read_administrator(&e);
        admin.require_auth();

        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        remove_kyc(&e, addr.clone());
        fail_kyc_event(&e, addr.clone());
    }

    pub fn pass_kyc(e: Env, addr: Address) {
        let admin = read_administrator(&e);
        admin.require_auth();

        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        write_kyc(&e, addr.clone());
        pass_kyc_event(&e, addr.clone());
    }

    pub fn blacklist(e: Env, addr: Address) {
        let admin = read_administrator(&e);
        admin.require_auth();

        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        write_blacklist(&e, addr.clone());
        blacklist_event(&e, addr.clone());
    }

    pub fn whitelist(e: Env, addr: Address) {
        let admin = read_administrator(&e);
        admin.require_auth();

        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        remove_blacklist(&e, addr.clone());
        whitelist_event(&e, addr.clone());
    }

    #[cfg(test)]
    pub fn get_allowance(e: Env, from: Address, spender: Address) -> Option<AllowanceValue> {
        let key = DataKey::Allowance(AllowanceDataKey { from, spender });
        let allowance = e.storage().temporary().get::<_, AllowanceValue>(&key);
        allowance
    }

    fn upgrade(e: Env, new_wasm_hash: BytesN<32>) {
        require_admin(&e);

        e.deployer().update_current_contract_wasm(new_wasm_hash);
    }

    pub fn total_supply(e: Env) -> i128 {
        total_supply(&e)
    }

    pub fn set_reward_rate(e: Env, rate: u32) {
        require_admin(&e);
        set_reward_rate(&e, rate);
    }
    pub fn set_reward_tick(e: Env, rate: u32) {
        require_admin(&e);
        set_reward_tick(&e, rate);
    }
}

#[contractimpl]
impl token::Interface for ExcellarToken {
    fn allowance(e: Env, from: Address, spender: Address) -> i128 {
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        read_allowance(&e, from, spender).amount
    }

    fn approve(e: Env, from: Address, spender: Address, amount: i128, expiration_ledger: u32) {
        pre_transfer_checks(&e, from.clone(), spender.clone(), amount);

        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        write_allowance(&e, from.clone(), spender.clone(), amount, expiration_ledger);
        TokenUtils::new(&e)
            .events()
            .approve(from, spender, amount, expiration_ledger);
    }

    fn balance(e: Env, id: Address) -> i128 {
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        read_balance(&e, id)
    }

    fn transfer(e: Env, from: Address, to: Address, amount: i128) {
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        pre_transfer_checks(&e, from.clone(), to.clone(), amount);

        checkpoint_reward(&e, from.clone());
        checkpoint_reward(&e, to.clone());
        spend_balance(&e, from.clone(), amount);
        receive_balance(&e, to.clone(), amount);
        TokenUtils::new(&e).events().transfer(from, to, amount);
    }

    fn transfer_from(e: Env, spender: Address, from: Address, to: Address, amount: i128) {
        pre_transfer_checks(&e, spender.clone(), to.clone(), amount);

        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        checkpoint_reward(&e, from.clone());
        checkpoint_reward(&e, to.clone());

        spend_allowance(&e, from.clone(), spender, amount);
        spend_balance(&e, from.clone(), amount);
        receive_balance(&e, to.clone(), amount);
        TokenUtils::new(&e).events().transfer(from, to, amount)
    }

    fn burn(e: Env, from: Address, amount: i128) {
        pre_mint_burn_checks(&e, from.clone(), amount);
        from.require_auth();

        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        checkpoint_reward(&e, from.clone());
        spend_balance(&e, from.clone(), amount);
        TokenUtils::new(&e).events().burn(from, amount);
    }

    fn burn_from(_env: Env, _spender: Address, _from: Address, _amount: i128) {
        panic!("not implemented")
    }

    fn decimals(e: Env) -> u32 {
        read_decimal(&e)
    }

    fn name(e: Env) -> String {
        read_name(&e)
    }

    fn symbol(e: Env) -> String {
        read_symbol(&e)
    }
}
