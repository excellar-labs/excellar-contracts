# XUSD Deployment Guide

This guide explains how to deploy the XUSD token contract to both Sepolia testnet and Ethereum mainnet.

## Prerequisites

1. Install Foundry if you haven't already:
```bash
curl -L https://foundry.paradigm.xyz | bash
foundryup
```

2. Create a `.env` file in the `evm` directory with the following variables:
```
PRIVATE_KEY=your_private_key_here
INITIAL_OWNER=address_that_will_own_the_contract
ETHERSCAN_API_KEY=your_etherscan_api_key
```

## Deployment Steps

### 1. Sepolia Testnet Deployment

```bash
# Load environment variables
source .env

# Deploy to Sepolia
forge script script/XUSD.s.sol:XUSDScript \
    --rpc-url https://sepolia.infura.io/v3/YOUR-PROJECT-ID \
    --broadcast \
    --verify \
    -vvvv

# Note down the proxy address from the deployment output
```

### 2. Mainnet Deployment

```bash
# Load environment variables
source .env

# Deploy to Mainnet
forge script script/XUSD.s.sol:XUSDScript \
    --rpc-url https://mainnet.infura.io/v3/YOUR-PROJECT-ID \
    --broadcast \
    --verify \
    -vvvv

# Note down the proxy address from the deployment output
```

## Post-Deployment Verification

After deployment, verify that:

1. The proxy contract is properly initialized
2. The initial owner has been set correctly
3. The contract is verified on Etherscan

You can interact with the deployed contract using the proxy address.

## Contract Addresses

Keep track of your deployed addresses here:

- Sepolia:
  - Implementation: `<implementation_address>`
  - Proxy: `<proxy_address>`

- Mainnet:
  - Implementation: `<implementation_address>`
  - Proxy: `<proxy_address>`

## Security Notes

1. Ensure you're using a secure private key
2. Double-check all addresses before deployment
3. Test all functionality on Sepolia before deploying to mainnet
4. Keep your deployment addresses and transaction hashes documented

## Important Reminders

1. The contract uses UUPS proxy pattern for upgradeability
2. Only the owner can mint new tokens
3. The contract has 6 decimals
4. The token symbol is "XUSD"
5. The token name is "XUSD Token"

## Verification Steps

After deployment, you can verify the contract on Etherscan:

1. Verify the implementation contract
2. Verify the proxy contract
3. Test basic functions like:
   - Check if owner is set correctly
   - Try minting tokens (only owner should be able to)
   - Verify token metadata (name, symbol, decimals)
