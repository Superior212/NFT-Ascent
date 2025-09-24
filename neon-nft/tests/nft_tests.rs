use neon_nft::*;
use stylus_sdk::testing::*;
use alloy_primitives::{Address, U256};

fn setup() -> (TestVM, MultiCollectionNFT) {
    let vm = TestVM::default();
    let contract = MultiCollectionNFT::from(&vm);
    (vm, contract)
}

#[test]
fn test_initialization() {
    let (vm, mut contract) = setup();

    // Initialize contract
    assert!(contract.initialize().is_ok());

    // Check platform name and symbol
    assert_eq!(contract.name().unwrap(), "Neon Multi-Collection NFT");
    assert_eq!(contract.symbol().unwrap(), "NEON-MULTI");
    assert_eq!(contract.get_next_token_id().unwrap(), U256::from(1));
    assert_eq!(contract.get_next_collection_id().unwrap(), U256::from(1));

    // Cannot initialize twice
    assert!(contract.initialize().is_err());
}

#[test]
fn test_collection_creation() {
    let (vm, mut contract) = setup();
    contract.initialize().unwrap();

    let creator = vm.msg_sender();
    let collection_name = "My Art Collection".to_string();
    let collection_symbol = "MAC".to_string();
    let base_uri = "https://myart.com/metadata/".to_string();

    // Create collection
    let collection_id = contract.create_collection(collection_name.clone(), collection_symbol.clone(), base_uri.clone()).unwrap();
    assert_eq!(collection_id, U256::from(1));

    // Check collection details
    let (name, symbol, collection_creator, uri, next_token) = contract.get_collection(collection_id).unwrap();
    assert_eq!(name, collection_name);
    assert_eq!(symbol, collection_symbol);
    assert_eq!(collection_creator, creator);
    assert_eq!(uri, base_uri);
    assert_eq!(next_token, U256::from(1));

    // Next collection ID should increment
    assert_eq!(contract.get_next_collection_id().unwrap(), U256::from(2));
}

#[test]
fn test_minting() {
    let (vm, mut contract) = setup();
    contract.initialize().unwrap();

    let owner = vm.msg_sender();

    // Create a collection first
    let collection_id = contract.create_collection(
        "Test Collection".to_string(),
        "TEST".to_string(),
        "https://test.com/".to_string()
    ).unwrap();

    let token_uri = "https://example.com/token/1".to_string();

    // Mint NFT in the collection
    let token_id = contract.mint_nft(collection_id, token_uri.clone()).unwrap();
    assert_eq!(token_id, U256::from(1));

    // Check ownership
    assert_eq!(contract.owner_of(token_id).unwrap(), owner);
    assert_eq!(contract.balance_of(owner).unwrap(), U256::from(1));
    assert_eq!(contract.balance_of_collection(owner, collection_id).unwrap(), U256::from(1));
    assert_eq!(contract.token_uri(token_id).unwrap(), token_uri);
    assert_eq!(contract.token_collection(token_id).unwrap(), collection_id);

    // Next token ID should increment
    assert_eq!(contract.get_next_token_id().unwrap(), U256::from(2));
}

#[test]
fn test_invalid_collection_creation() {
    let (vm, mut contract) = setup();
    contract.initialize().unwrap();

    // Cannot create collection with empty name
    assert!(contract.create_collection("".to_string(), "TEST".to_string(), "https://test.com/".to_string()).is_err());

    // Cannot create collection with empty symbol
    assert!(contract.create_collection("Test".to_string(), "".to_string(), "https://test.com/".to_string()).is_err());
}

#[test]
fn test_invalid_minting() {
    let (vm, mut contract) = setup();
    contract.initialize().unwrap();

    // Create a collection first
    let collection_id = contract.create_collection(
        "Test Collection".to_string(),
        "TEST".to_string(),
        "https://test.com/".to_string()
    ).unwrap();

    // Cannot mint with empty URI
    assert!(contract.mint_nft(collection_id, "".to_string()).is_err());

    // Cannot mint to nonexistent collection
    assert!(contract.mint_nft(U256::from(999), "test".to_string()).is_err());
}

#[test]
fn test_approval() {
    let (vm, mut contract) = setup();
    contract.initialize().unwrap();

    let owner = vm.msg_sender();
    let approved = Address::from([1u8; 20]);

    // Create collection and mint token
    let collection_id = contract.create_collection(
        "Test Collection".to_string(),
        "TEST".to_string(),
        "https://test.com/".to_string()
    ).unwrap();
    let token_id = contract.mint_nft(collection_id, "test".to_string()).unwrap();

    // Approve token
    assert!(contract.approve(approved, token_id).is_ok());
    assert_eq!(contract.get_approved(token_id).unwrap(), approved);

    // Cannot approve to self
    assert!(contract.approve(owner, token_id).is_err());
}

#[test]
fn test_approval_for_all() {
    let (vm, mut contract) = setup();
    contract.initialize().unwrap();

    let owner = vm.msg_sender();
    let operator = Address::from([1u8; 20]);

    // Approve operator for all
    assert!(contract.set_approval_for_all(operator, true).is_ok());
    assert!(contract.is_approved_for_all(owner, operator).unwrap());

    // Remove approval
    assert!(contract.set_approval_for_all(operator, false).is_ok());
    assert!(!contract.is_approved_for_all(owner, operator).unwrap());

    // Cannot approve self as operator
    assert!(contract.set_approval_for_all(owner, true).is_err());
}

#[test]
fn test_transfer() {
    let (vm, mut contract) = setup();
    contract.initialize().unwrap();

    let owner = vm.msg_sender();
    let recipient = Address::from([1u8; 20]);

    // Create collection and mint token
    let collection_id = contract.create_collection(
        "Test Collection".to_string(),
        "TEST".to_string(),
        "https://test.com/".to_string()
    ).unwrap();
    let token_id = contract.mint_nft(collection_id, "test".to_string()).unwrap();

    // Transfer token
    assert!(contract.transfer_from(owner, recipient, token_id).is_ok());

    // Check new ownership
    assert_eq!(contract.owner_of(token_id).unwrap(), recipient);
    assert_eq!(contract.balance_of(owner).unwrap(), U256::ZERO);
    assert_eq!(contract.balance_of(recipient).unwrap(), U256::from(1));
    assert_eq!(contract.balance_of_collection(owner, collection_id).unwrap(), U256::ZERO);
    assert_eq!(contract.balance_of_collection(recipient, collection_id).unwrap(), U256::from(1));
}

#[test]
fn test_unauthorized_transfer() {
    let (vm, mut contract) = setup();
    contract.initialize().unwrap();

    let owner = vm.msg_sender();
    let unauthorized = Address::from([1u8; 20]);
    let recipient = Address::from([2u8; 20]);

    // Create collection and mint token
    let collection_id = contract.create_collection(
        "Test Collection".to_string(),
        "TEST".to_string(),
        "https://test.com/".to_string()
    ).unwrap();
    let token_id = contract.mint_nft(collection_id, "test".to_string()).unwrap();

    // Set sender to unauthorized address
    vm.set_msg_sender(unauthorized);

    // Should fail - unauthorized transfer
    assert!(contract.transfer_from(owner, recipient, token_id).is_err());
}

#[test]
fn test_nonexistent_token() {
    let (vm, mut contract) = setup();
    contract.initialize().unwrap();

    let nonexistent_token = U256::from(999);

    // Operations on nonexistent token should fail
    assert!(contract.owner_of(nonexistent_token).is_err());
    assert!(contract.token_uri(nonexistent_token).is_err());
    assert!(contract.get_approved(nonexistent_token).is_err());
}

#[test]
fn test_zero_address_operations() {
    let (vm, mut contract) = setup();
    contract.initialize().unwrap();

    // Cannot get balance of zero address
    assert!(contract.balance_of(Address::ZERO).is_err());
}