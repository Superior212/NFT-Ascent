# Neon NFT Marketplace

A complete NFT marketplace built with **Rust** and **Arbitrum Stylus** for ultra-fast, low-cost NFT trading. Features NFT minting, English auctions, automatic bidding, and platform fee management.

## 🚀 Features

- **🎨 NFT Minting** - Create NFTs with IPFS metadata URIs
- **🏆 English Auctions** - List NFTs for auction with reserve prices
- **💰 Automatic Bidding** - Place bids with automatic refunds for outbid users
- **💸 Platform Fees** - 5% platform fee on successful sales
- **🔒 Security** - Reentrancy protection and input validation
- **📊 Full Tracking** - Complete auction and ownership management

## ⚡ Why Stylus?

- **10x Cheaper Gas** - Significantly lower transaction costs than EVM
- **Faster Execution** - WebAssembly performance optimization
- **Full EVM Compatibility** - Works with existing Ethereum tools
- **Rust Safety** - Memory-safe smart contracts with compile-time guarantees

## 🛠️ Prerequisites

Before you begin, make sure you have:

- [Rust](https://rustup.rs/) (v1.87 or newer)
- [Docker](https://www.docker.com/products/docker-desktop/) (for compilation)
- [Stylus CLI](https://github.com/OffchainLabs/cargo-stylus)

## 📦 Installation

### 1. Install Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

### 2. Install Stylus CLI
```bash
cargo install --force cargo-stylus
```

### 3. Clone and Setup
```bash
git clone https://github.com/alfredadenigba/neon-marketplace
cd neon-marketplace
```

## 🚀 Quick Start

### Build and Check
```bash
# Verify the contract compiles
cargo stylus check

# Export Solidity ABI for frontend integration
cargo stylus export-abi
```

### Deploy to Stylus Testnet
```bash
cargo stylus deploy \
  --endpoint https://stylus-testnet.arbitrum.io/rpc \
  --private-key YOUR_PRIVATE_KEY
```

### Deploy to Local Testnet
```bash
# Start local test node (in another terminal)
git clone https://github.com/OffchainLabs/nitro-testnode.git
cd nitro-testnode && ./test-node.bash --init --detach

# Deploy locally
cargo stylus deploy \
  --endpoint http://localhost:8547 \
  --private-key 0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659
```

## 📖 Usage Guide

### Initialize Marketplace
```rust
// Call once after deployment
initialize()
```

### Mint an NFT
```rust
// Mint with metadata URI
mint_nft("ipfs://QmYourMetadataHash")
// Returns: token_id
```

### Create Auction
```rust
// List NFT for auction
create_auction(
    token_id: 1,
    reserve_price: 1000000000000000000, // 1 ETH in wei
    duration: 86400                     // 24 hours in seconds
)
// Returns: auction_id
```

### Place Bid
```rust
// Bid on auction (send ETH with transaction)
place_bid(auction_id: 1)
```

### Settle Auction
```rust
// Anyone can settle after auction ends
settle_auction(auction_id: 1)
```

### Withdraw Funds
```rust
// Withdraw your earnings or refunded bids
withdraw()
```

## 🔍 View Functions

### Get Auction Details
```rust
get_auction(auction_id: 1)
// Returns: (seller, token_id, reserve_price, current_bid, current_bidder, end_time, settled)
```

### Check if Auction is Active
```rust
is_auction_active(auction_id: 1)
// Returns: bool
```

### Get User Balance
```rust
get_balance(user_address)
// Returns: withdrawable_amount
```

### Get Contract State
```rust
get_next_ids()
// Returns: (next_token_id, next_auction_id)
```

### NFT Information
```rust
owner_of(token_id)
// Returns: owner_address

token_uri(token_id)
// Returns: metadata_uri

balance_of(owner_address)
// Returns: nft_count
```

## �� Fee Structure

- **Platform Fee**: 5% on successful auction sales
- **Gas Costs**: ~90% cheaper than equivalent Solidity contracts
- **No Minting Fees**: Free NFT creation
- **No Listing Fees**: Free auction creation

## 🔐 Security Features

- **Automatic Refunds**: Previous bidders get refunded automatically
- **Reentrancy Protection**: Safe withdrawal patterns
- **Input Validation**: Comprehensive parameter checking
- **Ownership Verification**: Proper token ownership checks
- **Platform Fee Protection**: Secure fee collection and distribution

## 🌐 Frontend Integration

### Generate ABI
```bash
cargo stylus export-abi > marketplace-abi.json
```

### Contract Address
After deployment, your contract will have a unique address on the Stylus network.

## 📊 Contract Events

The contract emits the following events for easy indexing:

- `NFTMinted(tokenId, creator, tokenURI)`
- `AuctionCreated(auctionId, tokenId, reservePrice, endTime)`  
- `BidPlaced(auctionId, bidder, amount)`
- `AuctionSettled(auctionId, winner, amount)`
- `FundsWithdrawn(user, amount)`

## 🔄 Complete User Flow

### NFT Creator Journey
1. **Initialize** marketplace (one-time setup)
2. **Mint NFT** with IPFS metadata
3. **Create auction** with reserve price and duration
4. **Monitor** auction progress
5. **Settle** auction after it ends
6. **Withdraw** earnings (minus 5% platform fee)

### NFT Buyer Journey
1. **Browse** active auctions
2. **Place bid** (must meet reserve price)
3. **Get outbid** → withdraw refund
4. **Win auction** → receive NFT
5. **Withdraw** any refunded bids

### Platform Owner Journey
1. **Deploy** and initialize marketplace
2. **Collect** 5% platform fees automatically
3. **Withdraw** accumulated platform earnings

## 🚨 Current Limitations

This is a **simplified** marketplace focused on core functionality:

- No collection system
- No royalty payments
- No admin controls or pausing
- No auction cancellation
- Fixed 5% platform fee
- No batch operations

## 🔮 Future Enhancements

Potential features for v2:
- Collection support and management
- Creator royalties and secondary sales
- Dutch auctions and fixed-price sales
- Admin governance and emergency controls
- Batch minting and auction creation
- Advanced bidding strategies
- Integration with external NFT standards

## 🏗️ Development

### Project Structure
```
neon-marketplace/
├── Cargo.toml              # Rust project configuration
├── src/
│   └── lib.rs              # Main marketplace implementation
├── rust-toolchain.toml     # Rust version specification
└── README.md               # This file
```

### Testing
```bash
# Run Rust tests
cargo test

# Test contract compilation
cargo stylus check

# Check contract size
cargo stylus check --release
```

### Local Development
```bash
# Watch for changes and recompile
cargo watch -x "stylus check"

# Build for release
cargo stylus build --release
```

## 📈 Performance Metrics

- **Contract Size**: 21.6 KiB (optimized for Stylus)
- **WASM Size**: 81.6 KiB
- **Gas Efficiency**: ~90% cheaper than Solidity equivalent
- **Compilation Time**: ~2-3 seconds
- **Deployment**: Single transaction

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Test thoroughly with `cargo stylus check`
5. Commit your changes (`git commit -m 'Add amazing feature'`)
6. Push to the branch (`git push origin feature/amazing-feature`)
7. Open a Pull Request

## 📄 License

MIT License - see LICENSE file for details

## 🏃‍♂️ Quick Command Reference

```bash
# Development
cargo stylus check                    # Verify contract
cargo stylus export-abi              # Generate ABI
cargo stylus build --release         # Build optimized version

# Deployment  
cargo stylus deploy --endpoint <RPC> --private-key <KEY>

# Utilities
cargo stylus activate <CONTRACT>     # Activate deployed contract
cargo stylus cache <CONTRACT>        # Cache for cheaper calls
```

## 🆘 Support

- **Documentation**: Check this README and inline code comments
- **Issues**: Open an issue on GitHub for bugs or feature requests
- **Stylus Docs**: [Official Stylus Documentation](https://docs.arbitrum.io/stylus)
- **Community**: Join the Arbitrum Discord for Stylus discussions

---

**Built with ❤️ using Rust and Arbitrum Stylus**
