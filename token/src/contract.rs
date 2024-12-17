//! This contract demonstrates a sample implementation of the Soroban token
//! interface.

use crate::admin::{
    add_amm,
    check_authorized, // check_kyc_passed,
    check_not_amm,
    check_not_blacklisted,
    has_contract_admin, //read_contract_admin,
    read_token_address,
    remove_amm,
    remove_blacklist,
    remove_kyc,
    require_contract_admin,
    write_blacklist,
    write_contract_admin,
    write_kyc,
    write_token_address,
};
use crate::event::{
    add_amm_event, blacklist_event, fail_kyc_event, pass_kyc_event, remove_amm_event,
    whitelist_event,
};
use soroban_sdk::{
    contract, contractimpl,
    token::{Client as TokenClient, TokenInterface},
    token::{StellarAssetClient as TokenAdminClient, StellarAssetInterface},
    Address, BytesN, Env, String,
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
        if has_contract_admin(&e) {
            panic!("already initialized")
        }
        write_contract_admin(&e, &admin);
        write_token_address(&e, &token);

        // should be roughly 0.013% to result in 5% APY. Below is 0.01%
        set_reward_rate(&e, 10_000);
        // roughly the number of ledger advancements in day
        set_reward_tick(&e, 28_800);
    }

    pub fn set_contract_admin(e: Env, new_admin: Address) {
        let _ = require_contract_admin(&e);

        write_contract_admin(&e, &new_admin);
    }

    // pub fn claim_reward(e: Env, to: Address) {
    //     to.require_auth();
    //     // check_kyc_passed(&e, to.clone());
    //     check_authorized(&e, to.clone());
    //     // amm addresses cannot directly claim
    //     check_not_amm(&e, to.clone());
    //
    //     checkpoint_reward(&e, to.clone());
    //     let reward = read_reward(&e, to.clone());
    //     if reward < 1 {
    //         return;
    //     }
    //     reset_reward(&e, to.clone());
    //     checkpoint_reward(&e, to.clone());
    //
    //     let xusd_token = TokenAdminClient::new(&e, &read_token_address(&e));
    //     xusd_token.mint(&to, &reward)
    // }

    pub fn admin_claim_reward(e: Env, to: Address) {
        let _ = require_contract_admin(&e);
        check_authorized(&e, to.clone());
        // check_kyc_passed(&e, to.clone());

        // amm addresses cannot be awarded directly
        check_not_amm(&e, to.clone());

        checkpoint_reward(&e, to.clone());
        let reward = read_reward(&e, to.clone());
        if reward < 1 {
            return;
        }
        reset_reward(&e, to.clone());
        checkpoint_reward(&e, to.clone());

        let xusd_token = TokenAdminClient::new(&e, &read_token_address(&e));
        xusd_token.mint(&to, &reward)
    }

    pub fn fail_kyc(e: Env, addr: Address) {
        let _ = require_contract_admin(&e);

        remove_kyc(&e, addr.clone());
        fail_kyc_event(&e, addr.clone());
    }

    pub fn pass_kyc(e: Env, addr: Address) {
        let _ = require_contract_admin(&e);

        write_kyc(&e, addr.clone());
        pass_kyc_event(&e, addr.clone());
    }

    pub fn blacklist(e: Env, addr: Address) {
        let _ = require_contract_admin(&e);

        write_blacklist(&e, addr.clone());
        blacklist_event(&e, addr.clone());
    }

    pub fn whitelist(e: Env, addr: Address) {
        let _ = require_contract_admin(&e);

        remove_blacklist(&e, addr.clone());
        whitelist_event(&e, addr.clone());
    }

    pub fn upgrade(e: Env, new_wasm_hash: BytesN<32>) {
        require_contract_admin(&e);

        e.deployer().update_current_contract_wasm(new_wasm_hash);
    }

    pub fn set_reward_rate(e: Env, rate: u32) {
        require_contract_admin(&e);
        set_reward_rate(&e, rate);
    }

    pub fn set_reward_tick(e: Env, rate: u32) {
        require_contract_admin(&e);
        set_reward_tick(&e, rate);
    }

    pub fn add_amm_address(e: Env, addr: Address) {
        let _ = require_contract_admin(&e);

        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        add_amm(&e, addr.clone());
        add_amm_event(&e, addr.clone());
    }

    pub fn remove_amm_address(e: Env, addr: Address) {
        let _ = require_contract_admin(&e);

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
    fn allowance(env: Env, from: Address, spender: Address) -> i128 {
        let token = TokenAdminClient::new(&env, &read_token_address(&env));
        token.set_authorized(&from, &true);

        from.require_auth();
        let xusd_token = TokenClient::new(&env, &read_token_address(&env));
        let result = xusd_token.allowance(&from, &spender);

        token.set_authorized(&from, &false);
        result
    }

    fn approve(env: Env, from: Address, spender: Address, amount: i128, expiration_ledger: u32) {
        let token = TokenAdminClient::new(&env, &read_token_address(&env));
        token.set_authorized(&from, &true);

        from.require_auth();
        let xusd_token = TokenClient::new(&env, &read_token_address(&env));
        xusd_token.approve(&from, &spender, &amount, &expiration_ledger);

        token.set_authorized(&from, &false);
    }

    fn balance(env: Env, id: Address) -> i128 {
        let xusd_token = TokenClient::new(&env, &read_token_address(&env));
        xusd_token.balance(&id)
    }

    fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        let token = TokenAdminClient::new(&env, &read_token_address(&env));
        token.set_authorized(&from, &true);
        token.set_authorized(&to, &true);

        let xusd_token = TokenClient::new(&env, &read_token_address(&env));
        xusd_token.transfer(&from, &to, &amount);

        token.set_authorized(&from, &false);
        token.set_authorized(&to, &false);
    }

    fn transfer_from(env: Env, spender: Address, from: Address, to: Address, amount: i128) {
        let token = TokenAdminClient::new(&env, &read_token_address(&env));
        token.set_authorized(&spender, &true);
        token.set_authorized(&from, &true);
        token.set_authorized(&to, &true);

        spender.require_auth();
        let xusd_token = TokenClient::new(&env, &read_token_address(&env));
        xusd_token.transfer_from(&spender, &from, &to, &amount);

        token.set_authorized(&spender, &false);
        token.set_authorized(&from, &false);
        token.set_authorized(&to, &false);
    }

    fn burn(env: Env, from: Address, amount: i128) {
        let token = TokenAdminClient::new(&env, &read_token_address(&env));
        token.set_authorized(&from, &true);

        from.require_auth();
        let xusd_token = TokenClient::new(&env, &read_token_address(&env));
        xusd_token.burn(&from, &amount);

        token.set_authorized(&from, &false);
    }

    fn burn_from(env: Env, spender: Address, from: Address, amount: i128) {
        let token = TokenAdminClient::new(&env, &read_token_address(&env));
        token.set_authorized(&spender, &true);
        token.set_authorized(&from, &true);

        spender.require_auth();
        let xusd_token = TokenClient::new(&env, &read_token_address(&env));
        xusd_token.burn_from(&spender, &from, &amount);

        token.set_authorized(&spender, &false);
        token.set_authorized(&from, &false);
    }

    fn decimals(env: Env) -> u32 {
        let xusd_token = TokenClient::new(&env, &read_token_address(&env));
        xusd_token.decimals()
    }

    fn name(env: Env) -> String {
        let xusd_token = TokenClient::new(&env, &read_token_address(&env));
        xusd_token.name()
    }

    fn symbol(env: Env) -> String {
        let xusd_token = TokenClient::new(&env, &read_token_address(&env));
        xusd_token.symbol()
    }
}

#[contractimpl]
impl StellarAssetInterface for ExcellarToken {
    fn set_admin(env: Env, new_admin: Address) {
        let _ = require_contract_admin(&env);

        let token = TokenAdminClient::new(&env, &read_token_address(&env));
        token.set_admin(&new_admin);
    }

    fn admin(env: Env) -> Address {
        let token = TokenAdminClient::new(&env, &read_token_address(&env));
        token.admin()
    }

    fn set_authorized(env: Env, id: Address, authorize: bool) {
        let _ = require_contract_admin(&env);

        let token = TokenAdminClient::new(&env, &read_token_address(&env));
        token.set_authorized(&id, &authorize);
    }

    fn authorized(env: Env, id: Address) -> bool {
        // check_kyc_passed(&env, id.clone());
        let token = TokenAdminClient::new(&env, &read_token_address(&env));
        token.authorized(&id)
    }

    fn mint(env: Env, to: Address, amount: i128) {
        let _ = require_contract_admin(&env);

        let token = TokenAdminClient::new(&env, &read_token_address(&env));
        token.set_authorized(&to, &true);

        checkpoint_reward(&env, to.clone());
        token.mint(&to, &amount);

        token.set_authorized(&to, &false);
    }

    fn clawback(env: Env, from: Address, amount: i128) {
        let _ = require_contract_admin(&env);
        let token = TokenAdminClient::new(&env, &read_token_address(&env));

        token.clawback(&from, &amount);
    }
}
