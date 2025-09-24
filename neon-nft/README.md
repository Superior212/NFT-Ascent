# Neon Multi-Collection NFT

A multi-collection NFT smart contract built with Rust and Arbitrum Stylus. This contract allows users to create their own NFT collections and mint unique digital assets within those collections.

## Features

- **Multi-Collection Support**: Create unlimited NFT collections with custom names, symbols, and metadata
- **ERC721 Compatible**: Full compatibility with the ERC721 standard and existing NFT infrastructure
- **Gas Efficient**: Built on Arbitrum Stylus for ultra-low gas costs and high performance
- **Collection Management**: Comprehensive tools for managing collections and tracking ownership
- **Marketplace Ready**: Works seamlessly with NFT marketplaces and trading platforms

## Contract Architecture

### Core Components

1. **Collection Management**
   - Create new collections with custom metadata
   - Track collection creators and ownership
   - Manage collection-specific token numbering

2. **Token Operations**
   - Mint NFTs within specific collections
   - Transfer tokens with automatic balance tracking
   - Support for approvals and operator management

3. **Balance Tracking**
   - Global balance across all collections
   - Collection-specific balances
   - Efficient storage and retrieval

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

### 1. Create a Collection

```rust
// Create a new NFT collection
let collection_id = contract.create_collection(
    "My Art Collection".to_string(),
    "MAC".to_string(),
    "https://myart.com/metadata/".to_string()
)?;
```

### 2. Mint an NFT

```rust
// Mint an NFT in the collection
let token_id = contract.mint_nft(
    collection_id,
    "https://myart.com/metadata/1.json".to_string()
)?;
```

### 3. Query Collection Info

```rust
// Get collection details
let (name, symbol, creator, base_uri, next_token_id) = contract.get_collection(collection_id)?;

// Get user's balance in a specific collection
let collection_balance = contract.balance_of_collection(user_address, collection_id)?;
```

### 4. Transfer Tokens

```rust
// Transfer token between addresses
contract.transfer_from(from_address, to_address, token_id)?;
```

## API Reference

### Collection Management

- `create_collection(name, symbol, base_uri)` - Create a new collection
- `get_collection(collection_id)` - Get collection information
- `get_next_collection_id()` - Get the next available collection ID

### Token Operations

- `mint_nft(collection_id, token_uri)` - Mint a new NFT
- `owner_of(token_id)` - Get token owner
- `token_uri(token_id)` - Get token metadata URI
- `token_collection(token_id)` - Get the collection ID for a token

### Balance Queries

- `balance_of(owner)` - Get total token balance
- `balance_of_collection(owner, collection_id)` - Get collection-specific balance

### ERC721 Standard

- `approve(to, token_id)` - Approve token transfer
- `get_approved(token_id)` - Get approved address
- `set_approval_for_all(operator, approved)` - Set operator approval
- `is_approved_for_all(owner, operator)` - Check operator approval
- `transfer_from(from, to, token_id)` - Transfer token
- `safe_transfer_from(from, to, token_id)` - Safe transfer token

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

The contract emits the following events:

- `CollectionCreated(collection_id, creator, name, symbol)` - New collection created
- `NFTMinted(token_id, collection_id, creator, token_uri)` - New NFT minted
- `Transfer(from, to, token_id)` - Token transferred
- `Approval(owner, approved, token_id)` - Token approved
- `ApprovalForAll(owner, operator, approved)` - Operator approval set

## Integration with Marketplaces

This contract is designed to work seamlessly with NFT marketplaces. The included marketplace contract (`neon-marketplace`) provides:

- Auction functionality for any ERC721 token
- Collection information retrieval
- Platform fee management
- Automated settlement and fund distribution

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

- [Arbitrum Stylus Documentation](https://docs.arbitrum.io/stylus/stylus-gentle-introduction)
- [Stylus SDK](https://github.com/OffchainLabs/stylus-sdk-rs)
- [Arbitrum Testnet Information](https://docs.arbitrum.io/stylus/reference/testnet-information)

## Support

For issues and questions:
- Open an issue on GitHub
- Check the Arbitrum Stylus documentation
- Join the Arbitrum developer community