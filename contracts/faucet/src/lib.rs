#![no_std]

use soroban_sdk::{contract, contractimpl, contractmeta, contracttype, Address, Env};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    LiquidityPool,
    FaucetAmount,
    LastClaim(Address),
    ClaimInterval,
    WhichToken, // true = token_a, false = token_b
}

// Interface del Liquidity Pool
mod liquidity_pool {
    use soroban_sdk::{contractclient, Address, Env};

    #[contractclient(name = "LiquidityPoolClient")]
    pub trait LiquidityPoolTrait {
        /// Get current reserves
        fn get_reserves(e: Env) -> (i128, i128);
        
        /// Get token addresses
        fn get_tokens(e: Env) -> (Address, Address);
        
        /// Withdraw liquidity from the pool
        fn withdraw(
            e: Env,
            to: Address,
            share_amount: i128,
            min_a: i128,
            min_b: i128,
        ) -> (i128, i128);
        
        /// Get user's share balance
        fn balance_shares(e: Env, user: Address) -> i128;
        
        /// Get total shares
        fn get_total_shares(e: Env) -> i128;

        /// Deposit liquidity into the pool
        fn deposit(
            e: Env,
            to: Address,
            desired_a: i128,
            min_a: i128,
            desired_b: i128,
            min_b: i128,
        );
    }
}

contractmeta!(
    key = "Description",
    val = "Faucet contract that distributes tokens from a liquidity pool with rate limiting"
);

#[contract]
pub struct Faucet;

#[contractimpl]
impl Faucet {
    /// Initialize the faucet contract
    pub fn __constructor(
        e: Env,
        admin: Address,
        liquidity_pool: Address,
        faucet_amount: i128,
        claim_interval: u64,
        which_token: bool, // true = token_a, false = token_b
    ) {
        if faucet_amount <= 0 {
            panic!("faucet amount must be positive");
        }

        e.storage().instance().set(&DataKey::Admin, &admin);
        e.storage().instance().set(&DataKey::LiquidityPool, &liquidity_pool);
        e.storage().instance().set(&DataKey::FaucetAmount, &faucet_amount);
        e.storage().instance().set(&DataKey::ClaimInterval, &claim_interval);
        e.storage().instance().set(&DataKey::WhichToken, &which_token);
    }

    /// Claim tokens from the liquidity pool
    pub fn claim(e: Env, to: Address) {
        to.require_auth();

        let current_time = e.ledger().timestamp();
        let claim_interval: u64 = e.storage().instance().get(&DataKey::ClaimInterval).unwrap();
        
        // Check if enough time has passed since last claim
        if let Some(last_claim_time) = e.storage().persistent().get::<DataKey, u64>(&DataKey::LastClaim(to.clone())) {
            if current_time < last_claim_time + claim_interval {
                panic!("claim interval not met");
            }
        }

        let pool_address: Address = e.storage().instance().get(&DataKey::LiquidityPool).unwrap();
        let faucet_amount: i128 = e.storage().instance().get(&DataKey::FaucetAmount).unwrap();
        let which_token: bool = e.storage().instance().get(&DataKey::WhichToken).unwrap();

        let pool_client = liquidity_pool::LiquidityPoolClient::new(&e, &pool_address);
        
        // Get current reserves to check availability
        let (reserve_a, reserve_b) = pool_client.get_reserves();
        let available_amount = if which_token { reserve_a } else { reserve_b };
        
        if available_amount < faucet_amount {
            panic!("insufficient liquidity in pool");
        }

        // Get faucet's share balance in the pool
        let faucet_shares = pool_client.balance_shares(&e.current_contract_address());
        let total_shares = pool_client.get_total_shares();
        
        if faucet_shares == 0 {
            panic!("faucet has no shares in the pool");
        }

        // Calculate minimum shares needed to get the desired amount
        // We need to withdraw enough shares to get at least faucet_amount of the desired token
        let total_reserve = reserve_a + reserve_b;
        let target_reserve = if which_token { reserve_a } else { reserve_b };
        
        // Calculate shares needed: (faucet_amount * total_shares) / target_reserve
        // Add a small buffer to ensure we get enough
        let shares_needed = (faucet_amount * total_shares * 101) / (target_reserve * 100);
        
        if shares_needed > faucet_shares {
            panic!("faucet doesn't have enough shares");
        }

        // Calculate minimum amounts (we want 0 of the token we don't need)
        let (min_a, min_b) = if which_token {
            (faucet_amount, 0)  // We want at least faucet_amount of token A
        } else {
            (0, faucet_amount)  // We want at least faucet_amount of token B
        };

        // Withdraw from pool to get the tokens
        let (received_a, received_b) = pool_client.withdraw(
            &e.current_contract_address(),
            &shares_needed,
            &min_a,
            &min_b
        );

        // Get token addresses from pool
        let (token_a, token_b) = pool_client.get_tokens();
        
        // Transfer the desired token to the user
        if which_token {
            // Transfer token A
            let token_client = soroban_sdk::token::Client::new(&e, &token_a);
            token_client.transfer(&e.current_contract_address(), &to, &faucet_amount);
            
            // If we got more than needed, keep the extra
            // If we also got token B, keep it for future operations
        } else {
            // Transfer token B
            let token_client = soroban_sdk::token::Client::new(&e, &token_b);
            token_client.transfer(&e.current_contract_address(), &to, &faucet_amount);
            
            // If we got more than needed, keep the extra
            // If we also got token A, keep it for future operations
        }

        // Update last claim time
        e.storage().persistent().set(&DataKey::LastClaim(to.clone()), &current_time);
    }

    /// Admin function to deposit shares into the faucet
    pub fn deposit_shares(e: Env, from: Address, share_amount: i128) {
        from.require_auth();
        
        let admin: Address = e.storage().instance().get(&DataKey::Admin).unwrap();
        if from != admin {
            panic!("only admin can deposit shares");
        }

        let pool_address: Address = e.storage().instance().get(&DataKey::LiquidityPool).unwrap();
        let pool_client = liquidity_pool::LiquidityPoolClient::new(&e, &pool_address);
        
        // This would need to be implemented if the pool supports share transfers
        // For now, admin needs to deposit liquidity directly to the pool with faucet as recipient
        panic!("use deposit_liquidity instead");
    }

    /// Admin function to deposit liquidity directly to the pool
    /// This requires the admin to transfer tokens to this contract first
    pub fn deposit_liquidity(
        e: Env, 
        admin: Address,
        desired_a: i128,
        min_a: i128, 
        desired_b: i128,
        min_b: i128
    ) {
        admin.require_auth();
        
        let stored_admin: Address = e.storage().instance().get(&DataKey::Admin).unwrap();
        if admin != stored_admin {
            panic!("unauthorized");
        }

        let pool_address: Address = e.storage().instance().get(&DataKey::LiquidityPool).unwrap();
        let pool_client = liquidity_pool::LiquidityPoolClient::new(&e, &pool_address);
        
        // Get token addresses
        let (token_a, token_b) = pool_client.get_tokens();
        
        // Transfer tokens from admin to faucet
        let token_a_client = soroban_sdk::token::Client::new(&e, &token_a);
        let token_b_client = soroban_sdk::token::Client::new(&e, &token_b);
        
        token_a_client.transfer(&admin, &e.current_contract_address(), &desired_a);
        token_b_client.transfer(&admin, &e.current_contract_address(), &desired_b);
        
        // Deposit to pool (this will give shares to the faucet contract)
        pool_client.deposit(
            &e.current_contract_address(),
            &desired_a,
            &min_a,
            &desired_b,
            &min_b
        );
    }

    /// Admin function to set faucet amount
    pub fn set_faucet_amount(e: Env, admin: Address, new_amount: i128) {
        admin.require_auth();
        
        let stored_admin: Address = e.storage().instance().get(&DataKey::Admin).unwrap();
        if admin != stored_admin {
            panic!("unauthorized");
        }

        if new_amount <= 0 {
            panic!("faucet amount must be positive");
        }

        e.storage().instance().set(&DataKey::FaucetAmount, &new_amount);
    }

    /// Admin function to set claim interval
    pub fn set_claim_interval(e: Env, admin: Address, new_interval: u64) {
        admin.require_auth();
        
        let stored_admin: Address = e.storage().instance().get(&DataKey::Admin).unwrap();
        if admin != stored_admin {
            panic!("unauthorized");
        }

        e.storage().instance().set(&DataKey::ClaimInterval, &new_interval);
    }

    /// Admin function to change which token to distribute
    pub fn set_token_type(e: Env, admin: Address, which_token: bool) {
        admin.require_auth();
        
        let stored_admin: Address = e.storage().instance().get(&DataKey::Admin).unwrap();
        if admin != stored_admin {
            panic!("unauthorized");
        }

        e.storage().instance().set(&DataKey::WhichToken, &which_token);
    }

    /// Get current faucet amount
    pub fn get_faucet_amount(e: Env) -> i128 {
        e.storage().instance().get(&DataKey::FaucetAmount).unwrap()
    }

    /// Get current claim interval
    pub fn get_claim_interval(e: Env) -> u64 {
        e.storage().instance().get(&DataKey::ClaimInterval).unwrap()
    }

    /// Get faucet's share balance in the pool
    pub fn get_share_balance(e: Env) -> i128 {
        let pool_address: Address = e.storage().instance().get(&DataKey::LiquidityPool).unwrap();
        let pool_client = liquidity_pool::LiquidityPoolClient::new(&e, &pool_address);
        pool_client.balance_shares(&e.current_contract_address())
    }

    /// Get pool reserves
    pub fn get_pool_reserves(e: Env) -> (i128, i128) {
        let pool_address: Address = e.storage().instance().get(&DataKey::LiquidityPool).unwrap();
        let pool_client = liquidity_pool::LiquidityPoolClient::new(&e, &pool_address);
        pool_client.get_reserves()
    }

    /// Get token addresses from pool
    pub fn get_tokens(e: Env) -> (Address, Address) {
        let pool_address: Address = e.storage().instance().get(&DataKey::LiquidityPool).unwrap();
        let pool_client = liquidity_pool::LiquidityPoolClient::new(&e, &pool_address);
        pool_client.get_tokens()
    }

    /// Get which token is being distributed
    pub fn get_token_type(e: Env) -> bool {
        e.storage().instance().get(&DataKey::WhichToken).unwrap()
    }

    /// Get liquidity pool address
    pub fn get_pool_address(e: Env) -> Address {
        e.storage().instance().get(&DataKey::LiquidityPool).unwrap()
    }

    /// Get admin address
    pub fn get_admin(e: Env) -> Address {
        e.storage().instance().get(&DataKey::Admin).unwrap()
    }

    /// Get time until next claim is available for an address
    pub fn time_until_next_claim(e: Env, user: Address) -> u64 {
        let current_time = e.ledger().timestamp();
        let claim_interval: u64 = e.storage().instance().get(&DataKey::ClaimInterval).unwrap();
        
        if let Some(last_claim_time) = e.storage().persistent().get::<DataKey, u64>(&DataKey::LastClaim(user)) {
            let next_claim_time = last_claim_time + claim_interval;
            if current_time >= next_claim_time {
                0
            } else {
                next_claim_time - current_time
            }
        } else {
            0
        }
    }

    /// Check if an address can claim now
    pub fn can_claim(e: Env, user: Address) -> bool {
        Self::time_until_next_claim(e, user) == 0
    }

    /// Get estimated available tokens for claims
    pub fn get_available_for_claims(e: Env) -> i128 {
        let pool_address: Address = e.storage().instance().get(&DataKey::LiquidityPool).unwrap();
        let which_token: bool = e.storage().instance().get(&DataKey::WhichToken).unwrap();
        let pool_client = liquidity_pool::LiquidityPoolClient::new(&e, &pool_address);
        
        let faucet_shares = pool_client.balance_shares(&e.current_contract_address());
        if faucet_shares == 0 {
            return 0;
        }
        
        let (reserve_a, reserve_b) = pool_client.get_reserves();
        let total_shares = pool_client.get_total_shares();
        
        if total_shares == 0 {
            return 0;
        }
        
        // Estimate how much of the desired token we could get
        let target_reserve = if which_token { reserve_a } else { reserve_b };
        
        // Conservative estimate: (faucet_shares * target_reserve) / total_shares
        (faucet_shares * target_reserve) / total_shares
    }
}

mod test;
