use neon_nft::*;
use stylus_sdk::testing::*;
use alloy_primitives::{Address, U256};

fn setup() -> (TestVM, NeonNFT) {
    let vm = TestVM::default();
    let contract = NeonNFT::from(&vm);
    (vm, contract)
}

#[test]
fn test_initialization() {
    let (vm, mut contract) = setup();

    // Initialize contract
    assert!(contract.initialize().is_ok());

    // Check name and symbol
    assert_eq!(contract.name().unwrap(), "Neon NFT Collection");
    assert_eq!(contract.symbol().unwrap(), "NEON");
    assert_eq!(contract.get_next_token_id().unwrap(), U256::from(1));

    // Cannot initialize twice
    assert!(contract.initialize().is_err());
}

#[test]
fn test_minting() {
    let (vm, mut contract) = setup();
    contract.initialize().unwrap();

    let owner = vm.msg_sender();
    let token_uri = "https://example.com/token/1".to_string();

    // Mint NFT
    let token_id = contract.mint_nft(token_uri.clone()).unwrap();
    assert_eq!(token_id, U256::from(1));

    // Check ownership
    assert_eq!(contract.owner_of(token_id).unwrap(), owner);
    assert_eq!(contract.balance_of(owner).unwrap(), U256::from(1));
    assert_eq!(contract.token_uri(token_id).unwrap(), token_uri);

    // Next token ID should increment
    assert_eq!(contract.get_next_token_id().unwrap(), U256::from(2));
}

#[test]
fn test_invalid_minting() {
    let (vm, mut contract) = setup();
    contract.initialize().unwrap();

    // Cannot mint with empty URI
    assert!(contract.mint_nft("".to_string()).is_err());
}

#[test]
fn test_approval() {
    let (vm, mut contract) = setup();
    contract.initialize().unwrap();

    let owner = vm.msg_sender();
    let approved = Address::from([1u8; 20]);
    let token_id = contract.mint_nft("test".to_string()).unwrap();

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
    let token_id = contract.mint_nft("test".to_string()).unwrap();

    // Transfer token
    assert!(contract.transfer_from(owner, recipient, token_id).is_ok());

    // Check new ownership
    assert_eq!(contract.owner_of(token_id).unwrap(), recipient);
    assert_eq!(contract.balance_of(owner).unwrap(), U256::ZERO);
    assert_eq!(contract.balance_of(recipient).unwrap(), U256::from(1));
}

#[test]
fn test_unauthorized_transfer() {
    let (vm, mut contract) = setup();
    contract.initialize().unwrap();

    let owner = vm.msg_sender();
    let unauthorized = Address::from([1u8; 20]);
    let recipient = Address::from([2u8; 20]);
    let token_id = contract.mint_nft("test".to_string()).unwrap();

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