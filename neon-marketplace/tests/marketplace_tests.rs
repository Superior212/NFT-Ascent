use neon_marketplace::*;
use stylus_sdk::testing::*;
use alloy_primitives::{Address, U256};

fn setup() -> (TestVM, NeonMarketplace) {
    let vm = TestVM::default();
    let contract = NeonMarketplace::from(&vm);
    (vm, contract)
}

#[test]
fn test_initialization() {
    let (vm, mut contract) = setup();

    // Initialize contract with 5% platform fee
    assert!(contract.initialize(U256::from(500)).is_ok());

    // Check platform fee
    assert_eq!(contract.get_platform_fee_percentage().unwrap(), U256::from(500));

    // Cannot initialize twice
    assert!(contract.initialize(U256::from(500)).is_err());
}

#[test]
fn test_create_auction() {
    let (vm, mut contract) = setup();
    contract.initialize(U256::from(500)).unwrap();

    let nft_contract = Address::from([1u8; 20]);
    let token_id = U256::from(1);
    let reserve_price = U256::from(1000);
    let duration = U256::from(3600); // 1 hour

    // Create auction
    let auction_id = contract.create_auction(
        nft_contract,
        token_id,
        reserve_price,
        duration
    ).unwrap();

    assert_eq!(auction_id, U256::from(1));

    // Check auction details
    let auction = contract.get_auction(auction_id).unwrap();
    assert_eq!(auction.0, nft_contract); // nft_contract
    assert_eq!(auction.1, token_id); // token_id
    assert_eq!(auction.2, vm.msg_sender()); // seller
    assert_eq!(auction.3, reserve_price); // reserve_price
    assert_eq!(auction.6, false); // settled
}

#[test]
fn test_invalid_auction_creation() {
    let (vm, mut contract) = setup();
    contract.initialize(U256::from(500)).unwrap();

    let nft_contract = Address::from([1u8; 20]);
    let token_id = U256::from(1);

    // Cannot create auction with zero reserve price
    assert!(contract.create_auction(
        nft_contract,
        token_id,
        U256::ZERO,
        U256::from(3600)
    ).is_err());

    // Cannot create auction with zero duration
    assert!(contract.create_auction(
        nft_contract,
        token_id,
        U256::from(1000),
        U256::ZERO
    ).is_err());

    // Cannot create auction with zero address NFT contract
    assert!(contract.create_auction(
        Address::ZERO,
        token_id,
        U256::from(1000),
        U256::from(3600)
    ).is_err());
}

#[test]
fn test_place_bid() {
    let (vm, mut contract) = setup();
    contract.initialize(U256::from(500)).unwrap();

    let nft_contract = Address::from([1u8; 20]);
    let token_id = U256::from(1);
    let reserve_price = U256::from(1000);
    let duration = U256::from(3600);

    // Create auction
    let auction_id = contract.create_auction(
        nft_contract,
        token_id,
        reserve_price,
        duration
    ).unwrap();

    // Set different bidder
    let bidder = Address::from([2u8; 20]);
    vm.set_msg_sender(bidder);
    vm.set_msg_value(U256::from(1500));

    // Place bid
    assert!(contract.place_bid(auction_id).is_ok());

    // Check bid details
    let auction = contract.get_auction(auction_id).unwrap();
    assert_eq!(auction.4, bidder); // highest_bidder
    assert_eq!(auction.5, U256::from(1500)); // highest_bid
}

#[test]
fn test_invalid_bids() {
    let (vm, mut contract) = setup();
    contract.initialize(U256::from(500)).unwrap();

    let nft_contract = Address::from([1u8; 20]);
    let token_id = U256::from(1);
    let reserve_price = U256::from(1000);
    let duration = U256::from(3600);

    // Create auction
    let auction_id = contract.create_auction(
        nft_contract,
        token_id,
        reserve_price,
        duration
    ).unwrap();

    let bidder = Address::from([2u8; 20]);
    vm.set_msg_sender(bidder);

    // Bid below reserve price should fail
    vm.set_msg_value(U256::from(500));
    assert!(contract.place_bid(auction_id).is_err());

    // Seller cannot bid on own auction
    vm.set_msg_sender(vm.msg_sender()); // Reset to original sender (seller)
    vm.set_msg_value(U256::from(1500));
    assert!(contract.place_bid(auction_id).is_err());
}

#[test]
fn test_bid_progression() {
    let (vm, mut contract) = setup();
    contract.initialize(U256::from(500)).unwrap();

    let nft_contract = Address::from([1u8; 20]);
    let token_id = U256::from(1);
    let reserve_price = U256::from(1000);
    let duration = U256::from(3600);

    // Create auction
    let auction_id = contract.create_auction(
        nft_contract,
        token_id,
        reserve_price,
        duration
    ).unwrap();

    // First bidder
    let bidder1 = Address::from([2u8; 20]);
    vm.set_msg_sender(bidder1);
    vm.set_msg_value(U256::from(1500));
    assert!(contract.place_bid(auction_id).is_ok());

    // Second bidder with higher bid
    let bidder2 = Address::from([3u8; 20]);
    vm.set_msg_sender(bidder2);
    vm.set_msg_value(U256::from(2000));
    assert!(contract.place_bid(auction_id).is_ok());

    // Check highest bidder changed
    let auction = contract.get_auction(auction_id).unwrap();
    assert_eq!(auction.4, bidder2); // highest_bidder
    assert_eq!(auction.5, U256::from(2000)); // highest_bid

    // Cannot bid lower than current highest bid
    let bidder3 = Address::from([4u8; 20]);
    vm.set_msg_sender(bidder3);
    vm.set_msg_value(U256::from(1800));
    assert!(contract.place_bid(auction_id).is_err());
}

#[test]
fn test_settle_auction() {
    let (vm, mut contract) = setup();
    contract.initialize(U256::from(500)).unwrap();

    let nft_contract = Address::from([1u8; 20]);
    let token_id = U256::from(1);
    let reserve_price = U256::from(1000);
    let duration = U256::from(3600);

    // Create auction
    let auction_id = contract.create_auction(
        nft_contract,
        token_id,
        reserve_price,
        duration
    ).unwrap();

    // Place bid
    let bidder = Address::from([2u8; 20]);
    vm.set_msg_sender(bidder);
    vm.set_msg_value(U256::from(1500));
    contract.place_bid(auction_id).unwrap();

    // Advance time past auction end
    vm.set_block_timestamp(vm.block_timestamp() + duration + U256::from(1));

    // Settle auction
    assert!(contract.settle_auction(auction_id).is_ok());

    // Check auction is settled
    let auction = contract.get_auction(auction_id).unwrap();
    assert_eq!(auction.6, true); // settled
}

#[test]
fn test_cannot_settle_active_auction() {
    let (vm, mut contract) = setup();
    contract.initialize(U256::from(500)).unwrap();

    let nft_contract = Address::from([1u8; 20]);
    let token_id = U256::from(1);
    let reserve_price = U256::from(1000);
    let duration = U256::from(3600);

    // Create auction
    let auction_id = contract.create_auction(
        nft_contract,
        token_id,
        reserve_price,
        duration
    ).unwrap();

    // Cannot settle active auction
    assert!(contract.settle_auction(auction_id).is_err());
}

#[test]
fn test_nonexistent_auction() {
    let (vm, mut contract) = setup();
    contract.initialize(U256::from(500)).unwrap();

    let nonexistent_auction = U256::from(999);

    // Operations on nonexistent auction should fail
    assert!(contract.get_auction(nonexistent_auction).is_err());
    assert!(contract.place_bid(nonexistent_auction).is_err());
    assert!(contract.settle_auction(nonexistent_auction).is_err());
}

#[test]
fn test_update_platform_fee() {
    let (vm, mut contract) = setup();
    contract.initialize(U256::from(500)).unwrap();

    let original_owner = vm.msg_sender();
    let new_fee = U256::from(750); // 7.5%

    // Update platform fee
    assert!(contract.update_platform_fee(new_fee).is_ok());
    assert_eq!(contract.get_platform_fee_percentage().unwrap(), new_fee);

    // Non-owner cannot update platform fee
    let non_owner = Address::from([1u8; 20]);
    vm.set_msg_sender(non_owner);
    assert!(contract.update_platform_fee(U256::from(1000)).is_err());
}

#[test]
fn test_invalid_platform_fee() {
    let (vm, mut contract) = setup();

    // Cannot initialize with fee > 10000 basis points (100%)
    assert!(contract.initialize(U256::from(10001)).is_err());

    // Initialize with valid fee
    contract.initialize(U256::from(500)).unwrap();

    // Cannot update to invalid fee
    assert!(contract.update_platform_fee(U256::from(10001)).is_err());
}