# Neon NFT Ecosystem

A complete NFT ecosystem built with **Rust** and **Arbitrum Stylus** featuring both NFT contracts and a decentralized marketplace for ultra-fast, low-cost NFT trading.

## 🚀 Deployed Contracts

**Arbitrum Sepolia Testnet:**

- **NFT Contract**: `0xd3e20ae9c803da4c82dc4bae8a3e96ca0e4a4a84`
- **Marketplace Contract**: `0x135b3a004e7a746c43967226a8379f95fe9b4e23`
- **Network**: Arbitrum Sepolia (Chain ID: 421614)

## 📂 Project Structure

This repository contains two main components:

### 1. [**neon-nft**](./neon-nft) - Simple NFT Contract

- **Size**: 23.8 KiB (optimized for Stylus deployment)
- **Type**: Single collection NFT contract
- **Features**: ERC721 compliant, secure minting, optimized gas usage

### 2. [**neon-marketplace**](./neon-marketplace) - NFT Marketplace

- **Size**: 21.6 KiB (optimized for Stylus deployment)
- **Type**: English auction marketplace
- **Features**: Universal ERC721 support, platform fees, automated settlement

## 🎯 Key Features

### NFT Contract (`neon-nft`)

- **✅ ERC721 Compliant**: Full compatibility with existing NFT infrastructure
- **🔒 Secure**: Comprehensive security measures and error handling
- **⚡ Optimized**: Contract size under 24KB for efficient Stylus deployment
- **💰 Gas Efficient**: Built on Arbitrum Stylus for ultra-low costs

### Marketplace Contract (`neon-marketplace`)

- **🏆 English Auctions**: Time-based bidding with automatic settlement
- **🌐 Universal Support**: Works with any ERC721-compatible NFT
- **💸 Platform Fees**: Configurable fees (currently 5%)
- **🔐 Security**: Reentrancy protection and comprehensive validation

## 🚀 Quick Start

### Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Stylus CLI
cargo install --force cargo-stylus cargo-stylus-check
rustup target add wasm32-unknown-unknown
```

### Build Both Contracts

```bash
# Build NFT contract
cd neon-nft
cargo stylus check

# Build marketplace contract
cd ../neon-marketplace
cargo stylus check
```

### Deploy Contracts

```bash
# Deploy NFT contract
cd neon-nft
cargo stylus deploy --private-key-path=<KEY_FILE> --endpoint=https://sepolia-rollup.arbitrum.io/rpc

# Deploy marketplace contract
cd ../neon-marketplace
cargo stylus deploy --private-key-path=<KEY_FILE> --endpoint=https://sepolia-rollup.arbitrum.io/rpc
```

## 📖 Usage Examples

### 1. Initialize NFT Contract

```bash
cast send 0xd3e20ae9c803da4c82dc4bae8a3e96ca0e4a4a84 \
  "initialize(string,string)" "My NFT Collection" "MNC" \
  --private-key <KEY> --rpc-url https://sepolia-rollup.arbitrum.io/rpc
```

### 2. Initialize Marketplace Contract

```bash
cast send 0x135b3a004e7a746c43967226a8379f95fe9b4e23 \
  "initialize(uint256)" 500 \
  --private-key <KEY> --rpc-url https://sepolia-rollup.arbitrum.io/rpc
```

### 3. Mint NFT

```bash
cast send 0xd3e20ae9c803da4c82dc4bae8a3e96ca0e4a4a84 \
  "mint(address,string)" <TO_ADDRESS> "ipfs://QmYourMetadataHash" \
  --private-key <KEY> --rpc-url https://sepolia-rollup.arbitrum.io/rpc
```

### 4. Create Auction on Marketplace

```bash
# First approve marketplace to transfer NFT
cast send 0xd3e20ae9c803da4c82dc4bae8a3e96ca0e4a4a84 \
  "approve(address,uint256)" 0x135b3a004e7a746c43967226a8379f95fe9b4e23 <TOKEN_ID> \
  --private-key <KEY> --rpc-url https://sepolia-rollup.arbitrum.io/rpc

# Create auction
cast send 0x135b3a004e7a746c43967226a8379f95fe9b4e23 \
  "createAuction(address,uint256,uint256,uint256)" \
  0xd3e20ae9c803da4c82dc4bae8a3e96ca0e4a4a84 <TOKEN_ID> 100000000000000000 86400 \
  --private-key <KEY> --rpc-url https://sepolia-rollup.arbitrum.io/rpc
```

## 🔍 Complete User Flow

### NFT Creator Journey

1. **Deploy** or use existing NFT contract
2. **Initialize** contract with collection name and symbol
3. **Mint NFTs** with IPFS metadata URIs
4. **Approve marketplace** to transfer NFTs
5. **Create auctions** with reserve prices and durations
6. **Settle auctions** after they end
7. **Withdraw earnings** (minus platform fees)

### NFT Buyer Journey

1. **Browse** active auctions on marketplace
2. **Place bids** (must exceed reserve price and current bid)
3. **Monitor** auction until it ends
4. **Receive NFT** if winning bid
5. **Get refunds** for outbid amounts

## 🔐 Security Features

### NFT Contract

- ✅ Integer underflow protection
- ✅ Ownership verification on transfers
- ✅ Proper initialization checks
- ✅ Comprehensive input validation

### Marketplace Contract

- ✅ Reentrancy protection
- ✅ Automatic bid refunds
- ✅ Secure fund management
- ✅ Platform fee protection

## 🌐 Integration

Both contracts are fully compatible with:

- **Wallets**: MetaMask, WalletConnect, etc.
- **Explorers**: Arbiscan, Etherscan-compatible
- **NFT Platforms**: OpenSea, LooksRare (via ERC721 compliance)
- **DeFi**: Any protocol supporting ERC721 tokens

## 📄 Contract Interfaces

### NFT Contract (ISimpleNFT)

```solidity
function initialize(string memory name, string memory symbol) external;
function mint(address to, string memory tokenURI) external returns (uint256);
function ownerOf(uint256 tokenId) external view returns (address);
function tokenURI(uint256 tokenId) external view returns (string memory);
function balanceOf(address owner) external view returns (uint256);
```

### Marketplace Contract (INeonMarketplace)

```solidity
function initialize(uint256 platform_fee_percentage) external;
function createAuction(address nft_contract, uint256 token_id, uint256 reserve_price, uint256 duration) external returns (uint256);
function placeBid(uint256 auction_id) external payable;
function settleAuction(uint256 auction_id) external;
function withdraw() external;
```

## 🛠️ Development

### Testing

```bash
# Test both contracts
cd neon-nft && cargo test
cd ../neon-marketplace && cargo test
```

### Generate ABIs

```bash
# Generate NFT ABI
cd neon-nft && cargo stylus export-abi > nft_abi.sol

# Generate Marketplace ABI
cd neon-marketplace && cargo stylus export-abi > marketplace_abi.sol
```

## 📈 Roadmap

### Phase 1 ✅ (Current)

- [x] Basic NFT contract with ERC721 compliance
- [x] English auction marketplace
- [x] Security audits and optimizations
- [x] Testnet deployment

### Phase 2 🚧 (In Progress)

- [ ] Multi-collection NFT support
- [ ] Creator royalties system
- [ ] Advanced auction types (Dutch, sealed-bid)
- [ ] Batch operations

### Phase 3 📋 (Planned)

- [ ] NFT staking and rewards
- [ ] Cross-chain compatibility
- [ ] Advanced marketplace features
- [ ] DAO governance integration

## 🤝 Contributing

1. Fork the repository
2. Choose a component (`neon-nft` or `neon-marketplace`)
3. Create a feature branch
4. Implement changes with tests
5. Ensure `cargo stylus check` passes
6. Submit a pull request

## 👥 Team

This project was developed by:

### Core Contributors

- **Samson**
- **Olumide**
- **James**

## 📞 Support & Links

- **NFT Explorer**: https://sepolia.arbiscan.io/address/0xd3e20ae9c803da4c82dc4bae8a3e96ca0e4a4a84
- **Marketplace Explorer**: https://sepolia.arbiscan.io/address/0x135b3a004e7a746c43967226a8379f95fe9b4e23

## 📄 License

This project is licensed under MIT OR Apache-2.0 license. See individual `licenses/` directories for full texts.
