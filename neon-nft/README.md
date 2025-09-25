# Neon Simple NFT

A simple NFT smart contract built with Rust and Arbitrum Stylus. This contract provides essential NFT functionality with optimal gas efficiency and security.

## 🚀 Deployed Contract

**Arbitrum Sepolia Testnet:**
- **Contract Address:** `0xd3e20ae9c803da4c82dc4bae8a3e96ca0e4a4a84`
- **Network:** Arbitrum Sepolia
- **Chain ID:** 421614

## Features

- **ERC721 Compatible**: Full compatibility with the ERC721 standard and existing NFT infrastructure
- **Gas Efficient**: Built on Arbitrum Stylus for ultra-low gas costs and high performance
- **Secure**: Comprehensive security measures and error handling
- **Optimized**: Contract size under 24KB for efficient deployment
- **Marketplace Ready**: Works seamlessly with NFT marketplaces and trading platforms

## Contract Architecture

### Core Components

1. **NFT Management**
   - Single collection with custom name and symbol
   - Mint NFTs with unique token URIs
   - Transfer tokens with automatic balance tracking

2. **Token Operations**
   - Secure minting with ownership tracking
   - Transfer with comprehensive approval system
   - Support for operator management

3. **Balance Tracking**
   - Efficient balance tracking per address
   - Token ownership verification
   - Optimized storage patterns

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
cargo test --test nft_tests
```

## Usage Examples

### 1. Initialize Contract (One-time setup)

```bash
# Using cast (Foundry)
cast send 0xd3e20ae9c803da4c82dc4bae8a3e96ca0e4a4a84 \
  "initialize(string,string)" "My NFT Collection" "MNC" \
  --private-key <YOUR_PRIVATE_KEY> \
  --rpc-url https://sepolia-rollup.arbitrum.io/rpc
```

### 2. Mint an NFT

```solidity
// Mint an NFT to an address with metadata URI
uint256 tokenId = nft.mint(toAddress, "ipfs://QmYourMetadataHash");
```

### 3. Query Token Info

```solidity
// Get token owner
address owner = nft.ownerOf(tokenId);

// Get token metadata URI
string memory uri = nft.tokenURI(tokenId);

// Get user's balance
uint256 balance = nft.balanceOf(userAddress);
```

### 4. Transfer Tokens

```solidity
// Transfer token between addresses
nft.transferFrom(fromAddress, toAddress, tokenId);
```

## Contract Interface

### Core Functions

```solidity
interface ISimpleNFT {
    // Initialization - Call once after deployment
    function initialize(string memory name, string memory symbol) external;

    // Token Operations
    function mint(address to, string memory tokenURI) external returns (uint256);
    function ownerOf(uint256 tokenId) external view returns (address);
    function tokenURI(uint256 tokenId) external view returns (string memory);
    function balanceOf(address owner) external view returns (uint256);

    // Collection Info
    function name() external view returns (string memory);
    function symbol() external view returns (string memory);
}
```

### Token Operations

- `mint(to, token_uri)` - Mint a new NFT to an address
- `ownerOf(token_id)` - Get token owner
- `tokenURI(token_id)` - Get token metadata URI
- `balanceOf(owner)` - Get token balance

### Collection Info

- `name()` - Get collection name
- `symbol()` - Get collection symbol

### ERC721 Standard Functions

- `approve(to, token_id)` - Approve token transfer
- `getApproved(token_id)` - Get approved address
- `setApprovalForAll(operator, approved)` - Set operator approval
- `isApprovedForAll(owner, operator)` - Check operator approval
- `transferFrom(from, to, token_id)` - Transfer token

## Deployment

### Testnet Deployment

```bash
# Deploy to Arbitrum Sepolia testnet
cargo stylus deploy \
  --private-key-path=<PRIVKEY_FILE_PATH> \
  --endpoint=https://sepolia-rollup.arbitrum.io/rpc
```

### Mainnet Deployment

```bash
# Deploy to Arbitrum One mainnet
cargo stylus deploy \
  --private-key-path=<PRIVKEY_FILE_PATH> \
  --endpoint=https://arb1.arbitrum.io/rpc
```

## Contract Events

```solidity
event Transfer(address indexed from, address indexed to, uint256 indexed tokenId);
event Approval(address indexed owner, address indexed approved, uint256 indexed tokenId);
event ApprovalForAll(address indexed owner, address indexed operator, bool approved);
event NFTMinted(uint256 indexed tokenId, address indexed to, string tokenURI);
```

The contract emits the following events:

- `NFTMinted(token_id, to, token_uri)` - New NFT minted
- `Transfer(from, to, token_id)` - Token transferred
- `Approval(owner, approved, token_id)` - Token approved
- `ApprovalForAll(owner, operator, approved)` - Operator approval set

## Integration with Marketplaces

This contract is designed to work seamlessly with NFT marketplaces and DeFi protocols:

- **Neon Marketplace**: Our companion marketplace contract with English auctions
- **OpenSea**: Full ERC721 compliance for listing and trading
- **DeFi Protocols**: Use NFTs as collateral or in yield farming
- **Cross-chain bridges**: Standard ERC721 for multi-chain deployment

### Using with Neon Marketplace

The NFT contract works perfectly with our deployed marketplace at `0x135b3a004e7a746c43967226a8379f95fe9b4e23`:

```bash
# Approve marketplace to transfer your NFT
cast send 0xd3e20ae9c803da4c82dc4bae8a3e96ca0e4a4a84 \
  "approve(address,uint256)" 0x135b3a004e7a746c43967226a8379f95fe9b4e23 <TOKEN_ID> \
  --private-key <KEY> --rpc-url https://sepolia-rollup.arbitrum.io/rpc

# Create auction on marketplace
cast send 0x135b3a004e7a746c43967226a8379f95fe9b4e23 \
  "createAuction(address,uint256,uint256,uint256)" \
  0xd3e20ae9c803da4c82dc4bae8a3e96ca0e4a4a84 <TOKEN_ID> 100000000000000000 86400 \
  --private-key <KEY> --rpc-url https://sepolia-rollup.arbitrum.io/rpc
```

## Security Features

- **Reentrancy Protection**: Safe external calls and state management
- **Access Control**: Proper ownership and approval checks
- **Input Validation**: Comprehensive parameter validation
- **Error Handling**: Detailed error messages for debugging

## Gas Optimization

Built on Arbitrum Stylus for maximum efficiency:

- **WASM Performance**: Rust compiled to WebAssembly for optimal execution
- **Storage Efficiency**: Optimized data structures and packing
- **Low Transaction Costs**: Leverages Arbitrum's L2 scaling benefits

## Token URI and Metadata

The contract stores individual token URIs for each NFT. These typically point to JSON metadata following the ERC721 standard:

### Metadata JSON Format
```json
{
  "name": "My NFT #1",
  "description": "Description of the NFT",
  "image": "ipfs://QmImageHashHere",
  "attributes": [
    {
      "trait_type": "Color",
      "value": "Blue"
    },
    {
      "trait_type": "Rarity",
      "value": "Common"
    }
  ]
}
```

### Storage Options
- **IPFS**: `"ipfs://QmYourMetadataHash"` (recommended for decentralization)
- **Arweave**: `"ar://transaction_id"` (permanent storage)
- **HTTP**: `"https://yourdomain.com/metadata/1.json"` (centralized but flexible)

## Development

### Project Structure

```
src/
├── lib.rs          # Main contract implementation
tests/
├── nft_tests.rs    # Comprehensive test suite
Cargo.toml          # Dependencies and metadata
README.md           # This file
```

### Contributing

1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Ensure all tests pass
5. Submit a pull request

## License

This project is licensed under MIT OR Apache-2.0 license. See the `licenses/` directory for full license texts.

## Links

- **Contract Address**: `0xd3e20ae9c803da4c82dc4bae8a3e96ca0e4a4a84`
- **Arbitrum Sepolia Explorer**: https://sepolia.arbiscan.io/address/0xd3e20ae9c803da4c82dc4bae8a3e96ca0e4a4a84
- [Arbitrum Stylus Documentation](https://docs.arbitrum.io/stylus/stylus-gentle-introduction)
- [Stylus SDK](https://github.com/OffchainLabs/stylus-sdk-rs)
- [Neon NFT Marketplace](../neon-marketplace)

## Support

For issues and questions:
- Open an issue on GitHub
- Check the Arbitrum Stylus documentation
- Join the Arbitrum developer community