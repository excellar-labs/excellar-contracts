#![cfg(test)]
extern crate std;

use soroban_sdk::testutils::{Address as _, Ledger, LedgerInfo};
use soroban_sdk::{Address, Env, IntoVal};

use crate::reward::{_calculate_reward, set_reward_rate, set_reward_tick};
use crate::ExcellarTokenClient;

fn create_token<'a>(e: &Env, admin: &Address) -> ExcellarTokenClient<'a> {
    let token = ExcellarTokenClient::new(
        e,
        &e.register_contract(None, crate::contract::ExcellarToken {}),
    );
    token.initialize(admin, &7, &"name".into_val(e), &"symbol".into_val(e));
    token
}

fn advance_ledger(env: &Env, blocks: u32) {
    let ledger = env.ledger().get();
    env.ledger().set(LedgerInfo {
        timestamp: ledger.timestamp,
        protocol_version: ledger.protocol_version,
        sequence_number: ledger.sequence_number + blocks,
        network_id: ledger.network_id,
        base_reserve: ledger.base_reserve,
        min_temp_entry_ttl: ledger.min_temp_entry_ttl,
        min_persistent_entry_ttl: ledger.min_persistent_entry_ttl,
        max_entry_ttl: ledger.max_entry_ttl,
    });
}
fn setup_test_env() -> (Env, Address) {
    let env = Env::default();
    let investor = soroban_sdk::Address::generate(&env);
    let token_admin = soroban_sdk::Address::generate(&env);
    env.mock_all_auths();
    let token = create_token(&env, &token_admin);
    set_reward_tick(&env, 1);
    set_reward_rate(&env, 5_00);
    (env, investor)
}

#[test]
fn test_reward_calculation_per_block_tick() {
    let reward_rate = 5_00;
    let reward_tick = 1;

    let blocks_held = 10;
    let balance = 1000;

    let result = _calculate_reward(blocks_held, balance, reward_rate, reward_tick);

    assert_eq!(result, 500, "Rounding error in _calculate_reward function");

    let blocks_held = 1_000_000;
    let balance = 1_000_000_000;

    let result = _calculate_reward(blocks_held, balance, reward_rate, reward_tick);
    assert_eq!(
        result, 50_000_000_000_000,
        "Rounding error in _calculate_reward function"
    );
}

#[test]
fn test_reward_calculation_per_day_tick() {
    let reward_rate = 5_00;
    let reward_tick = 28_800;

    let blocks_held = 287;
    let balance = 1_000;

    let result = _calculate_reward(blocks_held, balance, reward_rate, reward_tick);
    assert_eq!(result, 0);

    let blocks_held = 288;
    let result = _calculate_reward(blocks_held, balance, reward_rate, reward_tick);
    assert_eq!(result, 1);

    let balance = 10_000;
    let result = _calculate_reward(blocks_held, balance, reward_rate, reward_tick);
    assert_eq!(result, 5);
}
