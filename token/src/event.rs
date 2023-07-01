use soroban_sdk::{Address, Env, Symbol};

pub(crate) fn whitelist(e: &Env, user: Address) {
    let topics = (Symbol::new(e, "whitelist"), user);
    e.events().publish(topics, true);
}

pub(crate) fn blacklist(e: &Env, user: Address) {
    let topics = (Symbol::new(e, "blacklist"), user);
    e.events().publish(topics, true);
}
