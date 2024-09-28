//! This contract demonstrates a sample implementation of the Soroban token
//! interface.

use crate::admin::{
    add_amm, check_kyc_passed, check_not_amm, check_not_blacklisted, has_administrator, read_administrator, read_token_address, remove_amm, remove_blacklist, remove_kyc, require_admin, write_administrator, write_blacklist, write_kyc, write_token_address,
};
use crate::event::{
    add_amm_event, blacklist_event, fail_kyc_event, pass_kyc_event, remove_amm_event,
    whitelist_event,
};
use soroban_sdk::{
    contract, contractimpl, token::{Client as TokenClient, TokenInterface}, token::{StellarAssetClient as TokenAdminClient, StellarAssetInterface}, Address, BytesN,
    Env,
    String,
};


use crate::reward::{
    checkpoint_reward, read_reward, reset_reward, set_reward_rate, set_reward_tick,
};

use crate::storage_types::{INSTANCE_BUMP_AMOUNT, INSTANCE_LIFETIME_THRESHOLD};

#[contract]
pub struct ExcellarToken;

#[contractimpl]
impl ExcellarToken {
    pub fn initialize(e: Env, admin: Address, token: Address) {
        if has_administrator(&e) {
            panic!("already initialized")
        }
        write_administrator(&e, &admin);
        write_token_address(&e, &token);

        // should be roughly 0.013% to result in 5% APY. Below is 0.01%
        set_reward_rate(&e, 10_000);
        // roughly the number of ledger advancements in day
        set_reward_tick(&e, 28_800);
    }

    pub fn claim_reward(e: Env, to: Address) {
        to.require_auth();
        check_kyc_passed(&e, to.clone());
        // amm addresses cannot directly claim
        check_not_amm(&e, to.clone());

        checkpoint_reward(&e, to.clone());
        let reward = read_reward(&e, to.clone());
        if reward < 1 {
            return;
        }
        reset_reward(&e, to.clone());

        // TODO: fix somehow
        // receive_balance(&e, to.clone(), reward);
    }

    pub fn admin_claim_reward(e: Env, to: Address) {
        let _ = require_admin(&e);
        check_kyc_passed(&e, to.clone());
        // amm addresses cannot be awarded directly
        check_not_amm(&e, to.clone());

        checkpoint_reward(&e, to.clone());
        let reward = read_reward(&e, to.clone());
        if reward < 1 {
            return;
        }
        reset_reward(&e, to.clone());
        checkpoint_reward(&e, to.clone());
        // TODO: fix somehow
        // receive_balance(&e, to.clone(), reward);
    }

    pub fn fail_kyc(e: Env, addr: Address) {
        let admin = read_administrator(&e);
        admin.require_auth();

        remove_kyc(&e, addr.clone());
        fail_kyc_event(&e, addr.clone());
    }

    pub fn pass_kyc(e: Env, addr: Address) {
        let admin = read_administrator(&e);
        admin.require_auth();

        write_kyc(&e, addr.clone());
        pass_kyc_event(&e, addr.clone());
    }

    pub fn blacklist(e: Env, addr: Address) {
        let admin = read_administrator(&e);
        admin.require_auth();

        write_blacklist(&e, addr.clone());
        blacklist_event(&e, addr.clone());
    }

    pub fn whitelist(e: Env, addr: Address) {
        let admin = read_administrator(&e);
        admin.require_auth();

        remove_blacklist(&e, addr.clone());
        whitelist_event(&e, addr.clone());
    }

    fn upgrade(e: Env, new_wasm_hash: BytesN<32>) {
        require_admin(&e);

        e.deployer().update_current_contract_wasm(new_wasm_hash);
    }

    pub fn set_reward_rate(e: Env, rate: u32) {
        require_admin(&e);
        set_reward_rate(&e, rate);
    }

    pub fn set_reward_tick(e: Env, rate: u32) {
        require_admin(&e);
        set_reward_tick(&e, rate);
    }

    pub fn add_amm_address(e: Env, addr: Address) {
        let admin = read_administrator(&e);
        admin.require_auth();

        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        add_amm(&e, addr.clone());
        add_amm_event(&e, addr.clone());
    }

    pub fn remove_amm_address(e: Env, addr: Address) {
        let admin = read_administrator(&e);
        admin.require_auth();

        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        remove_amm(&e, addr.clone());
        reset_reward(&e, addr.clone());
        remove_amm_event(&e, addr.clone());
    }
    pub fn get_reward(e: Env, to: Address) -> i128 {
        read_reward(&e, to.clone())
    }
}

#[contractimpl]
impl TokenInterface for ExcellarToken {
    fn allowance(e: Env, from: Address, spender: Address) -> i128 {
        let xusd_token = TokenClient::new(&e, &read_token_address(&e));
        xusd_token.allowance(&from, &spender)
    }

    fn approve(e: Env, from: Address, spender: Address, amount: i128, expiration_ledger: u32) {
        let xusd_token = TokenClient::new(&e, &read_token_address(&e));
        xusd_token.approve(&from, &spender, &amount, &expiration_ledger)
    }

    fn balance(e: Env, id: Address) -> i128 {
        let xusd_token = TokenClient::new(&e, &read_token_address(&e));
        xusd_token.balance(&id)
    }

    fn transfer(e: Env, from: Address, to: Address, amount: i128) {
        let xusd_token = TokenClient::new(&e, &read_token_address(&e));
        xusd_token.transfer(&from, &to, &amount)
    }

    fn transfer_from(e: Env, spender: Address, from: Address, to: Address, amount: i128) {
        let xusd_token = TokenClient::new(&e, &read_token_address(&e));
        xusd_token.transfer_from(&spender, &from, &to, &amount)
    }

    fn burn(e: Env, from: Address, amount: i128) {
        let xusd_token = TokenClient::new(&e, &read_token_address(&e));
        xusd_token.burn(&from, &amount)
    }

    fn burn_from(e: Env, spender: Address, from: Address, amount: i128) {
        let xusd_token = TokenClient::new(&e, &read_token_address(&e));
        xusd_token.burn_from(&spender, &from, &amount)
    }

    fn decimals(e: Env) -> u32 {
        let xusd_token = TokenClient::new(&e, &read_token_address(&e));
        xusd_token.decimals()
    }

    fn name(e: Env) -> String {
        let xusd_token = TokenClient::new(&e, &read_token_address(&e));
        xusd_token.name()
    }

    fn symbol(e: Env) -> String {
        let xusd_token = TokenClient::new(&e, &read_token_address(&e));
        xusd_token.symbol()
    }
}

#[contractimpl]
impl StellarAssetInterface for ExcellarToken {
    fn set_admin(e: Env, new_admin: Address) {
        let _ = require_admin(&e);
        let token = TokenAdminClient::new(&e, &read_token_address(&e));

        write_administrator(&e, &new_admin);
        token.set_admin(&new_admin);
    }

    fn admin(e: Env) -> Address {
        read_administrator(&e)
    }

    fn set_authorized(e: Env, id: Address, authorize: bool) {
        let _ = require_admin(&e);
        let token = TokenAdminClient::new(&e, &read_token_address(&e));

        write_kyc(&e, id.clone());
        token.set_authorized(&id, &authorize);
    }

    fn authorized(e: Env, id: Address) -> bool {
        check_kyc_passed(&e, id.clone());
        let token = TokenAdminClient::new(&e, &read_token_address(&e));
        token.authorized(&id)
    }

    fn mint(e: Env, to: Address, amount: i128) {
        pre_mint_burn_checks(&e, to.clone(), amount);

        let _ = require_admin(&e);
        let token = TokenAdminClient::new(&e, &read_token_address(&e));

        checkpoint_reward(&e, to.clone());
        token.mint(&to, &amount);
    }

    fn clawback(e: Env, from: Address, amount: i128) {
        pre_mint_burn_checks(&e, from.clone(), amount);
        let _ = require_admin(&e);
        let token = TokenAdminClient::new(&e, &read_token_address(&e));

        checkpoint_reward(&e, from.clone());
        token.clawback(&from, &amount);
    }
}

pub fn pre_mint_burn_checks(e: &Env, to: Address, amount: i128) {
    check_non_negative_amount(amount);
    check_kyc_passed(e, to.clone());
}

pub fn pre_transfer_checks(e: &Env, spender: Address, to: Address, amount: i128) {
    spender.require_auth();

    check_non_negative_amount(amount);
    check_not_blacklisted(e, to);
}

pub fn check_non_negative_amount(amount: i128) {
    if amount < 0 {
        panic!("negative amount is not allowed: {}", amount)
    }
}
