// #![cfg(test)]
// extern crate std;

// use soroban_sdk::{Address, Env};

// use crate::{FaucetClient, LiquidityPoolClient};
// use crate::test::{create_liqpool_contract, create_token_contract};

// #[test]
// fn test_faucet_withdraws_and_pays() {
//     let e = Env::default();
//     e.mock_all_auths();

//     let admin1 = Address::generate(&e);
//     let admin2 = Address::generate(&e);
//     let (token1, token1_admin) = create_token_contract(&e, &admin1);
//     let (token2, token2_admin) = create_token_contract(&e, &admin2);

//     let depositor = Address::generate(&e);
//     token1_admin.mint(&depositor, &100);
//     token2_admin.mint(&depositor, &100);

//     let liqpool = create_liqpool_contract(&e, &token1.address, &token2.address);
//     let lp_client = LiquidityPoolClient::new(&e, &liqpool.address);
//     lp_client.deposit(&depositor, &100, &100, &100, &100);

//     // Faucet setup
//     let faucet_id = e.register(crate::Faucet {}, ());
//     let faucet = FaucetClient::new(&e, &faucet_id);

//     // Transfer shares del pool al faucet
//     let shares = lp_client.balance_shares(&depositor);
//     assert_eq!(shares, 100);
//     // Transferir manualmente las shares (usando acceso directo al storage)
//     lp_client
//         .env()
//         .storage()
//         .persistent()
//         .set(&crate::DataKey::Shares(faucet_id.clone()), &shares);
//     lp_client
//         .env()
//         .storage()
//         .persistent()
//         .set(&crate::DataKey::Shares(depositor.clone()), &0);
//     lp_client
//         .env()
//         .storage()
//         .instance()
//         .set(&crate::DataKey::TotalShares, &shares);

//     faucet.init(&liqpool.address, &token1.address, &1);

//     let user1 = Address::generate(&e);
//     faucet.pay_initial(&user1);

//     assert_eq!(token1.balance(&user1), 1);
// }

#![cfg(test)]
extern crate std;

use crate::{Faucet, FaucetClient};
use soroban_sdk::{
    symbol_short,
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation, MockAuth, MockAuthInvoke},
    token, Address, Env, IntoVal,
};

// Import the LiquidityPool for testing
mod liqpool {
    soroban_sdk::contractimport!(
        file = "../../target/wasm32v1-none/release/liquidity_pool.wasm"
    );
}

fn create_token_contract<'a>(
    e: &Env,
    admin: &Address,
) -> (token::Client<'a>, token::StellarAssetClient<'a>) {
    let sac = e.register_stellar_asset_contract_v2(admin.clone());
    (
        token::Client::new(e, &sac.address()),
        token::StellarAssetClient::new(e, &sac.address()),
    )
}

fn create_liqpool_contract<'a>(
    e: &Env,
    token_a: &Address,
    token_b: &Address,
) -> liqpool::Client<'a> {
    liqpool::Client::new(e, &e.register(liqpool::WASM, (token_a, token_b)))
}

fn create_faucet_contract<'a>(
    e: &Env,
    admin: &Address,
    liquidity_pool: &Address,
    faucet_amount: i128,
    claim_interval: u64,
    which_token: bool,
) -> FaucetClient<'a> {
    FaucetClient::new(
        e,
        &e.register(
            Faucet {},
            (admin, liquidity_pool, faucet_amount, claim_interval, which_token),
        ),
    )
}

#[test]
fn test_faucet_initialization() {
    let e = Env::default();
    e.mock_all_auths();

    let admin = Address::generate(&e);
    let liquidity_pool = Address::generate(&e);
    let faucet_amount = 100i128;
    let claim_interval = 3600u64; // 1 hour
    let which_token = true; // token_a

    let faucet = create_faucet_contract(&e, &admin, &liquidity_pool, faucet_amount, claim_interval, which_token);

    assert_eq!(faucet.get_faucet_amount(), faucet_amount);
    assert_eq!(faucet.get_claim_interval(), claim_interval);
    assert_eq!(faucet.get_admin(), admin);
    assert_eq!(faucet.get_pool_address(), liquidity_pool);
    assert_eq!(faucet.get_token_type(), which_token);
}

#[test]
fn test_faucet_claim_availability() {
    let e = Env::default();
    e.mock_all_auths();

    let admin = Address::generate(&e);
    let liquidity_pool = Address::generate(&e);
    let user = Address::generate(&e);
    let faucet_amount = 100i128;
    let claim_interval = 3600u64;
    let which_token = true;

    let faucet = create_faucet_contract(&e, &admin, &liquidity_pool, faucet_amount, claim_interval, which_token);

    // User should be able to claim initially
    assert_eq!(faucet.can_claim(&user), true);
    assert_eq!(faucet.time_until_next_claim(&user), 0);
}

#[test]
fn test_admin_functions() {
    let e = Env::default();
    e.mock_all_auths();

    let admin = Address::generate(&e);
    let liquidity_pool = Address::generate(&e);
    let faucet_amount = 100i128;
    let claim_interval = 3600u64;
    let which_token = true;

    let faucet = create_faucet_contract(&e, &admin, &liquidity_pool, faucet_amount, claim_interval, which_token);

    // Test setting new faucet amount
    let new_amount = 200i128;
    faucet.set_faucet_amount(&admin, &new_amount);
    assert_eq!(faucet.get_faucet_amount(), new_amount);

    // Test setting new claim interval
    let new_interval = 7200u64;
    faucet.set_claim_interval(&admin, &new_interval);
    assert_eq!(faucet.get_claim_interval(), new_interval);

    // Test changing token type
    let new_token_type = false;
    faucet.set_token_type(&admin, &new_token_type);
    assert_eq!(faucet.get_token_type(), new_token_type);
}

#[test]
#[should_panic(expected = "faucet amount must be positive")]
fn test_invalid_faucet_amount_initialization() {
    let e = Env::default();
    e.mock_all_auths();

    let admin = Address::generate(&e);
    let liquidity_pool = Address::generate(&e);
    let faucet_amount = -100i128; // Invalid negative amount
    let claim_interval = 3600u64;
    let which_token = true;

    create_faucet_contract(&e, &admin, &liquidity_pool, faucet_amount, claim_interval, which_token);
}

#[test]
#[should_panic(expected = "faucet amount must be positive")]
fn test_invalid_faucet_amount_update() {
    let e = Env::default();
    e.mock_all_auths();

    let admin = Address::generate(&e);
    let liquidity_pool = Address::generate(&e);
    let faucet_amount = 100i128;
    let claim_interval = 3600u64;
    let which_token = true;

    let faucet = create_faucet_contract(&e, &admin, &liquidity_pool, faucet_amount, claim_interval, which_token);

    // Try to set invalid amount
    faucet.set_faucet_amount(&admin, &(-50i128));
}

#[test]
fn test_faucet_pool_integration_basic() {
    let e = Env::default();
    e.mock_all_auths();

    // Setup tokens
    let admin1 = Address::generate(&e);
    let admin2 = Address::generate(&e);
    let (token1, token1_admin) = create_token_contract(&e, &admin1);
    let (token2, token2_admin) = create_token_contract(&e, &admin2);
    
    // Create liquidity pool
    let liqpool = create_liqpool_contract(&e, &token1.address, &token2.address);
    
    // Setup users
    let lp_provider = Address::generate(&e);
    let faucet_admin = Address::generate(&e);
    
    // Mint tokens to liquidity provider
    token1_admin.mint(&lp_provider, &10000);
    token2_admin.mint(&lp_provider, &10000);
    
    // Add liquidity to pool
    liqpool.deposit(&lp_provider, &1000, &1000, &1000, &1000);
    
    // Verify pool state
    let (reserve_a, reserve_b) = liqpool.get_rsrvs();
    assert_eq!(reserve_a, 1000);
    assert_eq!(reserve_b, 1000);
    
    // Create faucet
    let faucet_amount = 50i128;
    let claim_interval = 3600u64;
    let which_token = true; // distribute token_a
    
    let faucet = create_faucet_contract(
        &e,
        &faucet_admin,
        &liqpool.address,
        faucet_amount,
        claim_interval,
        which_token,
    );

    // Verify faucet can read pool data
    let (pool_reserve_a, pool_reserve_b) = faucet.get_pool_reserves();
    assert_eq!(pool_reserve_a, reserve_a);
    assert_eq!(pool_reserve_b, reserve_b);
    
    let (token_a_addr, token_b_addr) = faucet.get_tokens();
    assert_eq!(token_a_addr, token1.address);
    assert_eq!(token_b_addr, token2.address);

    // Initially faucet has no shares
    assert_eq!(faucet.get_share_balance(), 0);
    assert_eq!(faucet.get_available_for_claims(), 0);
}

#[test]
fn test_faucet_with_liquidity_deposit() {
    let e = Env::default();
    e.mock_all_auths();

    // Setup tokens
    let admin1 = Address::generate(&e);
    let admin2 = Address::generate(&e);
    let (token1, token1_admin) = create_token_contract(&e, &admin1);
    let (token2, token2_admin) = create_token_contract(&e, &admin2);
    
    // Create liquidity pool
    let liqpool = create_liqpool_contract(&e, &token1.address, &token2.address);
    
    // Setup users
    let lp_provider = Address::generate(&e);
    let faucet_admin = Address::generate(&e);
    
    // Mint tokens to both liquidity provider and faucet admin
    token1_admin.mint(&lp_provider, &10000);
    token2_admin.mint(&lp_provider, &10000);
    token1_admin.mint(&faucet_admin, &5000);
    token2_admin.mint(&faucet_admin, &5000);
    
    // Add initial liquidity to pool
    liqpool.deposit(&lp_provider, &1000, &1000, &1000, &1000);
    
    // Create faucet
    let faucet_amount = 50i128;
    let claim_interval = 3600u64;
    let which_token = true; // distribute token_a
    
    let faucet = create_faucet_contract(
        &e,
        &faucet_admin,
        &liqpool.address,
        faucet_amount,
        claim_interval,
        which_token,
    );

    // Faucet admin deposits liquidity to give faucet shares
    faucet.deposit_liquidity(&faucet_admin, &500, &400, &500, &400);
    
    // Now faucet should have shares and available tokens
    let faucet_shares = faucet.get_share_balance();
    assert!(faucet_shares > 0);
    
    let available_tokens = faucet.get_available_for_claims();
    assert!(available_tokens > 0);
    
    // Verify pool reserves increased
    let (reserve_a, reserve_b) = faucet.get_pool_reserves();
    assert_eq!(reserve_a, 1500); // 1000 + 500
    assert_eq!(reserve_b, 1500); // 1000 + 500
}

#[test]
#[should_panic(expected = "faucet has no shares in the pool")]
fn test_claim_without_shares() {
    let e = Env::default();
    e.mock_all_auths();

    // Setup tokens
    let admin1 = Address::generate(&e);
    let admin2 = Address::generate(&e);
    let (token1, token1_admin) = create_token_contract(&e, &admin1);
    let (token2, token2_admin) = create_token_contract(&e, &admin2);
    
    // Create liquidity pool with some liquidity
    let liqpool = create_liqpool_contract(&e, &token1.address, &token2.address);
    let lp_provider = Address::generate(&e);
    
    token1_admin.mint(&lp_provider, &10000);
    token2_admin.mint(&lp_provider, &10000);
    liqpool.deposit(&lp_provider, &1000, &1000, &1000, &1000);
    
    // Create faucet without giving it any shares
    let faucet_admin = Address::generate(&e);
    let faucet = create_faucet_contract(
        &e,
        &faucet_admin,
        &liqpool.address,
        50,
        3600,
        true,
    );

    // Try to claim - should fail because faucet has no shares
    let user = Address::generate(&e);
    faucet.claim(&user);
}

#[test]
fn test_multiple_users_independent_claims() {
    let e = Env::default();
    e.mock_all_auths();

    let admin = Address::generate(&e);
    let liquidity_pool = Address::generate(&e);
    let user1 = Address::generate(&e);
    let user2 = Address::generate(&e);
    let faucet_amount = 100i128;
    let claim_interval = 3600u64;
    let which_token = true;

    let faucet = create_faucet_contract(&e, &admin, &liquidity_pool, faucet_amount, claim_interval, which_token);

    // Both users should be able to claim initially
    assert_eq!(faucet.can_claim(&user1), true);
    assert_eq!(faucet.can_claim(&user2), true);
    
    // Time restrictions should be independent for each user
    assert_eq!(faucet.time_until_next_claim(&user1), 0);
    assert_eq!(faucet.time_until_next_claim(&user2), 0);
}

#[test]
#[should_panic(expected = "unauthorized")]
fn test_non_admin_cannot_change_settings() {
    let e = Env::default();
    e.mock_all_auths();

    let admin = Address::generate(&e);
    let non_admin = Address::generate(&e);
    let liquidity_pool = Address::generate(&e);
    let faucet_amount = 100i128;
    let claim_interval = 3600u64;
    let which_token = true;

    let faucet = create_faucet_contract(&e, &admin, &liquidity_pool, faucet_amount, claim_interval, which_token);

    // Non-admin tries to change faucet amount - should fail
    faucet.set_faucet_amount(&non_admin, &200);
}

#[test]
fn test_faucet_query_functions() {
    let e = Env::default();
    e.mock_all_auths();

    let admin = Address::generate(&e);
    let liquidity_pool = Address::generate(&e);
    let faucet_amount = 100i128;
    let claim_interval = 3600u64;
    let which_token = false; // token_b

    let faucet = create_faucet_contract(&e, &admin, &liquidity_pool, faucet_amount, claim_interval, which_token);

    // Test all getter functions
    assert_eq!(faucet.get_faucet_amount(), faucet_amount);
    assert_eq!(faucet.get_claim_interval(), claim_interval);
    assert_eq!(faucet.get_admin(), admin);
    assert_eq!(faucet.get_pool_address(), liquidity_pool);
    assert_eq!(faucet.get_token_type(), which_token);
    
    // These should return 0 initially since no real pool is connected
    assert_eq!(faucet.get_share_balance(), 0);
    assert_eq!(faucet.get_available_for_claims(), 0);
}