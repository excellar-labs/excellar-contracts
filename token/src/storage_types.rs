use soroban_sdk::{contracttype, Address};


pub const ONE_DAY_LEDGERS: u32 = 17280; // assumes 5s a ledger
pub const INSTANCE_LIFETIME_THRESHOLD: u32 = ONE_DAY_LEDGERS * 30; // ~ 30 days
pub const INSTANCE_BUMP_AMOUNT: u32 = INSTANCE_LIFETIME_THRESHOLD + ONE_DAY_LEDGERS; // ~ 31 days
pub const LEDGER_THRESHOLD_SHARED: u32 = ONE_DAY_LEDGERS * 45; // ~ 45 days
pub const LEDGER_BUMP_SHARED: u32 = LEDGER_THRESHOLD_SHARED + ONE_DAY_LEDGERS; // ~ 46 days
pub const LEDGER_THRESHOLD_USER: u32 = ONE_DAY_LEDGERS * 100; // ~ 100 days
pub const LEDGER_BUMP_USER: u32 = LEDGER_THRESHOLD_USER + 20 * ONE_DAY_LEDGERS; // ~ 120 days


#[contracttype]
pub struct AllowanceValue {
    pub amount: i128,
    pub expiration_ledger: u32,
}

#[contracttype]
pub struct AccumulatedReward {
    pub created_ledger_number: u32,
    pub last_ledger_number: u32,
    pub amount: i128,
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    RewardCheckpoint(Address),
    Kyc(Address),
    Blacklisted(Address),
    Amm(Address),
    AmmDepositor(Address),
    Admin,
    TokenAddress,
    TotalSupply,
    RewardRate,
    RewardTick,
}
