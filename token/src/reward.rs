use crate::balance::{read_balance, receive_balance};
use crate::storage_types::{
    DataKey, BALANCE_BUMP_AMOUNT, BALANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT,
    INSTANCE_LIFETIME_THRESHOLD,
};
use soroban_sdk::{contracttype, Address, Env};

#[contracttype]
pub struct AccumulatedReward {
    created_ledger_number: u32,
    last_ledger_number: u32,
    amount: i128,
}

pub fn read_reward(e: &Env, addr: Address) -> i128 {
    let key = DataKey::RewardCheckpoint(addr);
    if let Some(reward) = e
        .storage()
        .persistent()
        .get::<DataKey, AccumulatedReward>(&key)
    {
        e.storage()
            .persistent()
            .extend_ttl(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
        reward.amount
    } else {
        0
    }
}

fn write_reward(e: &Env, addr: Address, amount: i128) {
    let key = DataKey::RewardCheckpoint(addr);
    let existing_reward: Option<AccumulatedReward> = e.storage().persistent().get(&key);
    match existing_reward {
        Some(reward) => {
            let acc_reward = AccumulatedReward {
                created_ledger_number: reward.created_ledger_number,
                last_ledger_number: e.ledger().sequence(),
                amount: amount + reward.amount,
            };
            e.storage().persistent().set(&key, &acc_reward);
            e.storage().persistent().extend_ttl(
                &key,
                BALANCE_LIFETIME_THRESHOLD,
                BALANCE_BUMP_AMOUNT,
            );
        }
        None => {
            let acc_reward = AccumulatedReward {
                created_ledger_number: e.ledger().sequence(),
                last_ledger_number: e.ledger().sequence(),
                amount,
            };
            e.storage().persistent().set(&key, &acc_reward)
        }
    }
    e.storage()
        .persistent()
        .extend_ttl(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
}

pub fn reset_reward(e: &Env, addr: Address) {
    let key = DataKey::RewardCheckpoint(addr);
    e.storage().persistent().remove(&key);
}

pub fn set_reward_rate(e: &Env, rate: u32) {
    let key = DataKey::RewardRate;
    let rate = rate.max(0);
    e.storage().persistent().set(&key, &rate);
    e.storage()
        .persistent()
        .extend_ttl(&key, INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
}

pub fn get_reward_rate(e: &Env) -> u32 {
    let key = DataKey::RewardRate;
    if let Some(rate) = e.storage().persistent().get::<DataKey, u32>(&key) {
        e.storage().persistent().extend_ttl(
            &key,
            INSTANCE_LIFETIME_THRESHOLD,
            INSTANCE_BUMP_AMOUNT,
        );
        rate
    } else {
        0
    }
}

pub fn set_reward_tick(e: &Env, tick: u32) {
    let tick = tick.max(0);
    let key = DataKey::RewardTick;
    e.storage().persistent().set(&key, &tick);
    e.storage()
        .persistent()
        .extend_ttl(&key, INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
}

pub fn get_reward_tick(e: &Env) -> u32 {
    let key = DataKey::RewardTick;
    if let Some(tick) = e.storage().persistent().get::<DataKey, u32>(&key) {
        e.storage().persistent().extend_ttl(
            &key,
            INSTANCE_LIFETIME_THRESHOLD,
            INSTANCE_BUMP_AMOUNT,
        );
        tick
    } else {
        // every block
        1
    }
}

pub fn calculate_reward(e: &Env, addr: Address) -> i128 {
    let key = DataKey::RewardCheckpoint(addr.clone());
    let reward_checkpoint: Option<AccumulatedReward> = e.storage().persistent().get(&key);
    let blocks_held = match reward_checkpoint {
        Some(checkpoint) => e.ledger().sequence() - checkpoint.created_ledger_number,
        None => 0,
    };
    let balance = read_balance(e, addr.clone());
    let reward_rate = get_reward_rate(&e);
    let reward_tick = get_reward_tick(&e);

    _calculate_reward(blocks_held, balance, reward_rate, reward_tick)
}

pub fn _calculate_reward(
    blocks_held: u32,
    balance: i128,
    reward_rate: u32,
    reward_tick: u32,
) -> i128 {
    let basis_points = 100_00;
    let reward_rate = reward_rate as f64 / basis_points as f64;
    let holding_period = blocks_held as f64 / reward_tick as f64;
    (balance as f64 * reward_rate * holding_period).round() as i128
}
pub fn checkpoint_reward(e: &Env, address: Address) {
    let reward = calculate_reward(&e, address.clone());
    write_reward(&e, address, reward);
}
