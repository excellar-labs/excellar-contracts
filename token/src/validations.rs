use soroban_sdk::{Address, Env};

use crate::admin::{check_kyc_passed, check_not_blacklisted};

pub fn pre_mint_burn_checks(e: &Env, to: Address, amount: i128) {
    check_non_negative_amount(amount);
    check_kyc_passed(e, to);
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
