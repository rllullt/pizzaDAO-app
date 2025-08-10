# pizzaDAO-app

Super-app that tokenizes parties, rewards organizers, and creates the world's most exciting digital economy.


## Using the frontend

The frontend of the app is located at the `frontend/` directory.
It is built with Vite.
To test this part of the application you must move there and follow the instructions written in the README.
```bash
cd frontend/
```


## Using the smart contracts

To test, it is important to configure Test identities.
You can use the famous Alice and Bob identities.
Configure them globally via:

```
stellar keys generate --global alice --network testnet --fund
stellar keys generate --global bob --network testnet --fund
stellar keys generate --global carol --network testnet --fund
```

Create a project-local identity for simulating a pool:

```
stellar keys generate pool-wallet --network testnet --fund
```

## Deploying and using the Liquidity Pool + Faucet

### 1. Compile

```bash
# Compilar todos los contratos
cargo build --target wasm32v1-none --release

# O compilar individualmente
cd contracts/liquidity_pool/
cargo build --target wasm32v1-none --release
cd ../../contracts/faucet/
cargo build --target wasm32v1-none --release
cd ../../
```

This creates the wasm files at the location e.g. for liquidity_pool: `target/wasm32v1-none/release/liquidity_pool.wasm`

Now, 2 alternatives: run Faucet tests (that use the LiquidityPool), or deploy on testnet via the command line.

### 2. Run tests

In an editor like VSCode, install the Rust-analyzer extension.
Now, you can run the specific tests by clicking in the “Run Tests” button into the file `contracts/faucet/src/lib.rs`.

### 2. Deploy on Testnet

1. **Compile contracts**:
```bash
cargo build --target wasm32v1-none --release
```

2. **Create test tokens**:
```bash
# Token A (XLM-like)
stellar contract asset deploy \
    --asset "TOKENA:$(stellar keys address alice)" \
    --source alice \
    --network testnet \
    --alias token_a

TOKEN_A=$(stellar contract id asset --asset "TOKENA:$(stellar keys address alice)" --network testnet)

# Token B (USDC-like)  
stellar contract asset deploy \
    --asset "TOKENB:$(stellar keys address bob)" \
    --source bob \
    --network testnet \
    --alias token_b

TOKEN_B=$(stellar contract id asset --asset "TOKENB:$(stellar keys address bob)" --network testnet)
```

3. **Deploy LiquidityPool**:
```bash
stellar contract deploy \
    --wasm target/wasm32v1-none/release/liquidity_pool.wasm \
    --source pool-wallet \
    --network testnet \
    --alias liquidity_pool \
    -- \
    --token_a "$TOKEN_A" \
    --token_b "$TOKEN_B"
# Liquidity_pool contract address:
LIQUIDITY_POOL=[ID returned by previous command]
```

4. **Deploy Faucet**:
```bash
stellar contract deploy \
    --wasm target/wasm32v1-none/release/faucet.wasm \
    --source pool-wallet \
    --network testnet \
    --alias faucet \
    -- \
    --admin "$(stellar keys address pool-wallet)" \
    --liquidity_pool "$LIQUIDITY_POOL" \
    --faucet_amount 1000000 \
    --claim_interval 300 \
    --which_token true
# Faucet contract address:
FAUCET_CONTRACT=[ID returned by previous command]
```

### 3. Use the contracts

### LiquidityPool - Add liquidity (as admin):

Add tokens to pool-wallet:

First, grant a trustline for pool-wallet:

```bash
LEDGER_NOW=$(curl -s https://horizon-testnet.stellar.org | jq .history_latest_ledger)
# Add 1000 for reasonable time (~5 minutes more)
EXPIRATION_LEDGER=$((LEDGER_NOW + 1000))

# Approve trustline
stellar contract invoke \
  --id "$TOKEN_A" \
  --source pool-wallet \
  --network testnet \
  -- \
  approve \
  --from "$(stellar keys address pool-wallet)" \
  --spender "$(stellar keys address alice)" \
  --amount "0" \
  --expiration-ledger "$EXPIRATION_LEDGER"
```

Now, add tokens:

```bash
stellar contract invoke \
    --id "$TOKEN_A" \
    --source pool-wallet \
    --network testnet \
    -- \
    mint \
    --to "$(stellar keys address pool-wallet)" \
    --amount "10000000000"
```

```bash
stellar contract invoke \
    --id "$FAUCET_CONTRACT" \
    --source pool-wallet \
    --network testnet \
    -- \
    deposit_liquidity \
    --admin "$(stellar keys address pool-wallet)" \
    --desired_a 10000000 \
    --min_a 9000000 \
    --desired_b 10000000 \
    --min_b 9000000
```

#### Faucet - Claim tokens:

```bash
# Verify if can claim
stellar contract invoke \
    --id faucet \
    --source carol \
    --network testnet \
    -- \
    can_claim \
    --user "$(stellar keys address carol)"

# Claim tokens
stellar contract invoke \
    --id faucet \
    --source carol \
    --network testnet \
    -- \
    claim \
    --to "$(stellar keys address carol)"
```

## Troubleshooting

Sometimes it is not possible to add tokens to an account from the command line.
In this case, there is not troubleshooting because the liquidity pool must need funds to the faucet to take from it.
At least, all the relevant MVP tests do pass.

```bash
test test::test_faucet_claim_availability ... ok
test test::test_faucet_initialization ... ok
test test::test_admin_functions ... ok
test test::test_multiple_users_independent_claims ... ok
test test::test_invalid_faucet_amount_initialization - should panic ... ok
```

