# Neon NFT Marketplace

A decentralized NFT marketplace built with Rust and Arbitrum Stylus. This marketplace supports auction-based trading for any ERC721-compatible NFT, including multi-collection NFTs.

## 🚀 Deployed Contract

**Arbitrum Sepolia Testnet:**
- **Contract Address:** `0x135b3a004e7a746c43967226a8379f95fe9b4e23`
- **Network:** Arbitrum Sepolia
- **Chain ID:** 421614

## Features

- **Universal ERC721 Support**: Works with any ERC721-compliant NFT contract
- **English Auctions**: Time-based bidding with automatic settlement
- **Platform Fees**: Configurable platform fees (max 10%)
- **Gas Efficient**: Built on Arbitrum Stylus for ultra-low gas costs
- **Secure**: Comprehensive security measures and error handling
- **Multi-Collection Aware**: Enhanced support for multi-collection NFT contracts

## Quick Start

### Prerequisites

Install [Rust](https://www.rust-lang.org/tools/install) and the Stylus CLI tools:

```bash
cargo install --force cargo-stylus cargo-stylus-check
rustup target add wasm32-unknown-unknown
```

### Building

```bash
# Check compilation and deployment readiness
cargo stylus check

# Build the project
cargo build --release

# Export ABI for integration
cargo stylus export-abi
```

### Testing

```bash
# Run all tests
cargo test

# Run specific test file
cargo test --test marketplace_tests
```

## Contract Interface

### Core Functions

```solidity
interface INeonMarketplace {
    // Initialization - Call once after deployment
    function initialize(uint256 platform_fee_percentage) external;

    // Auction Management
    function createAuction(address nft_contract, uint256 token_id, uint256 reserve_price, uint256 duration) external returns (uint256);
    function cancelAuction(uint256 auction_id) external;
    function placeBid(uint256 auction_id) external payable;
    function settleAuction(uint256 auction_id) external;

    // Platform Management
    function updatePlatformFee(uint256 new_fee_percentage) external;
    function withdraw() external;

    // View Functions
    function getAuction(uint256 auction_id) external view returns (address, uint256, address, uint256, uint256, address, uint256, bool);
    function isAuctionActive(uint256 auction_id) external view returns (bool);
    function getBalance(address user_address) external view returns (uint256);
    function getNextAuctionId() external view returns (uint256);
    function getPlatformFeePercentage() external view returns (uint256);
    function getPlatformOwner() external view returns (address);
    function getTokenCollectionInfo(address nft_contract, uint256 token_id) external view returns (uint256, string memory, string memory, address);
}
```

## Usage Examples

### 1. Initialize Contract (One-time setup)

```bash
# Using cast (Foundry)
cast send 0x135b3a004e7a746c43967226a8379f95fe9b4e23 \
  "initialize(uint256)" 500 \
  --private-key <YOUR_PRIVATE_KEY> \
  --rpc-url https://sepolia-rollup.arbitrum.io/rpc
```

### 2. Create an Auction

```solidity
// Approve marketplace to transfer your NFT first
nft.approve(marketplaceAddress, tokenId);

// Create auction: 24 hours duration, 0.1 ETH reserve price
uint256 auctionId = marketplace.createAuction(
    nftContractAddress,
    tokenId,
    100000000000000000, // 0.1 ETH in wei
    86400 // 24 hours in seconds
);
```

### 3. Place a Bid

```solidity
// Bid must be higher than reserve price and current highest bid
marketplace.placeBid{value: 0.2 ether}(auctionId);
```

### 4. Settle Auction

```solidity
// After auction ends, anyone can settle
marketplace.settleAuction(auctionId);
```

### 5. Withdraw Funds

```solidity
// Withdraw accumulated funds (failed bids, auction proceeds)
marketplace.withdraw();
```

## Integration with Multi-Collection NFTs

This marketplace has enhanced support for multi-collection NFT contracts:

```solidity
// Get collection information for a token
(uint256 collectionId, string memory name, string memory symbol, address creator) =
    marketplace.getTokenCollectionInfo(nftContract, tokenId);
```

## Platform Configuration

### Fee Structure

- **Platform Fee**: Configurable percentage (max 10% = 1000 basis points)
- **Current Fee**: 5% (500 basis points)
- **Fee Distribution**: Deducted from seller's proceeds

### Auction Parameters

- **Minimum Duration**: 1 second
- **Maximum Duration**: 30 days
- **Minimum Reserve**: > 0 ETH
- **Bid Increment**: Must exceed current highest bid

## Security Features

- **Reentrancy Protection**: Safe external calls and state management
- **Access Control**: Proper ownership and authorization checks
- **Input Validation**: Comprehensive parameter validation
- **Fund Safety**: Secure fund management with withdrawal patterns
- **Auction Integrity**: Protection against manipulation and front-running

## Contract Events

```solidity
event AuctionCreated(uint256 indexed auctionId, address indexed nftContract, uint256 indexed tokenId, uint256 reservePrice, uint256 endTime);
event BidPlaced(uint256 indexed auctionId, address indexed bidder, uint256 amount);
event AuctionSettled(uint256 indexed auctionId, address indexed winner, uint256 amount);
event AuctionCanceled(uint256 indexed auctionId, address indexed seller);
event PlatformFeeUpdated(uint256 newFeePercentage);
event FundsWithdrawn(address indexed user, uint256 amount);
```

## Error Handling

The contract includes comprehensive error handling:

- `AlreadyInitialized()` - Contract already initialized
- `AuctionNotFound()` - Invalid auction ID
- `BidTooLow()` - Bid below minimum required
- `AuctionNotEnded()` - Auction still active
- `NotTokenOwner()` - Caller doesn't own the NFT
- `TransferFailed()` - NFT transfer failed
- `InsufficientBalance()` - Insufficient funds to withdraw

## Development

### Project Structure

```
src/
├── lib.rs          # Main marketplace contract
├── main.rs         # ABI export helper
tests/
├── marketplace_tests.rs # Comprehensive test suite
Cargo.toml          # Dependencies and metadata
README.md           # This file
```

### Running Tests

```bash
# Run all marketplace tests
cargo test --test marketplace_tests

# Run specific test
cargo test test_create_auction
```

### Deployment

```bash
# Deploy to Arbitrum Sepolia
cargo stylus deploy \
  --private-key-path=<PRIVKEY_FILE_PATH> \
  --endpoint=https://sepolia-rollup.arbitrum.io/rpc

# Deploy to Arbitrum One Mainnet
cargo stylus deploy \
  --private-key-path=<PRIVKEY_FILE_PATH> \
  --endpoint=https://arb1.arbitrum.io/rpc
```

## Integration Examples

### JavaScript/TypeScript

```javascript
import { ethers } from 'ethers';

const marketplaceAddress = "0x135b3a004e7a746c43967226a8379f95fe9b4e23";
const marketplace = new ethers.Contract(marketplaceAddress, abi, signer);

// Create auction
const tx = await marketplace.createAuction(
  nftContract,
  tokenId,
  ethers.parseEther("0.1"), // Reserve price
  86400 // Duration in seconds
);
```

### Python (web3.py)

```python
from web3 import Web3

w3 = Web3(Web3.HTTPProvider('https://sepolia-rollup.arbitrum.io/rpc'))
marketplace = w3.eth.contract(
    address="0x135b3a004e7a746c43967226a8379f95fe9b4e23",
    abi=marketplace_abi
)

# Place bid
tx = marketplace.functions.placeBid(auction_id).build_transaction({
    'from': account.address,
    'value': Web3.to_wei(0.2, 'ether'),
    'gas': 200000
})
```

## Gas Optimization

Built on Arbitrum Stylus for maximum efficiency:

- **WASM Performance**: Rust compiled to WebAssembly
- **Low Transaction Costs**: Leverages Arbitrum's L2 scaling
- **Optimized Storage**: Efficient data structures and mappings
- **Minimal Gas Usage**: Optimized auction and bidding logic

## Contributing

1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Ensure all tests pass
5. Submit a pull request

## License

This project is licensed under MIT OR Apache-2.0 license. See the `licenses/` directory for full license texts.

## Links

- **Contract Address**: `0x135b3a004e7a746c43967226a8379f95fe9b4e23`
- **Arbitrum Sepolia Explorer**: https://sepolia.arbiscan.io/address/0x135b3a004e7a746c43967226a8379f95fe9b4e23
- [Arbitrum Stylus Documentation](https://docs.arbitrum.io/stylus/stylus-gentle-introduction)
- [Stylus SDK](https://github.com/OffchainLabs/stylus-sdk-rs)
- [Multi-Collection NFT Contract](../neon-nft)

## Support

For issues and questions:
- Open an issue on GitHub
- Check the Arbitrum Stylus documentation
- Join the Arbitrum developer community