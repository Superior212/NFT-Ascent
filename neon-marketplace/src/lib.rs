// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]
#![cfg_attr(not(any(test, feature = "export-abi")), no_std)]

extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use stylus_sdk::{
    alloy_primitives::{Address, U256},
    call::Call,
    evm,
    prelude::*,
};
use alloy_sol_types::sol;

// ERC721 Interface for interacting with existing NFTs
sol_interface! {
    interface IERC721 {
        function ownerOf(uint256 tokenId) external view returns (address);
        function transferFrom(address from, address to, uint256 tokenId) external;
        function safeTransferFrom(address from, address to, uint256 tokenId) external;
        function approve(address to, uint256 tokenId) external;
        function getApproved(uint256 tokenId) external view returns (address);
        function setApprovalForAll(address operator, bool approved) external;
        function isApprovedForAll(address owner, address operator) external view returns (bool);
        function balanceOf(address owner) external view returns (uint256);
    }
}

// Multi-Collection NFT Interface (optional - for additional collection info)
sol_interface! {
    interface IMultiCollectionNFT {
        function tokenCollection(uint256 tokenId) external view returns (uint256);
        function getCollection(uint256 collectionId) external view returns (string memory, string memory, address, string memory, uint256);
        function balanceOfCollection(address owner, uint256 collectionId) external view returns (uint256);
    }
}

// Marketplace Events
sol! {
    event AuctionCreated(uint256 indexed auctionId, address indexed nftContract, uint256 indexed tokenId, uint256 reservePrice, uint256 endTime);
    event BidPlaced(uint256 indexed auctionId, address indexed bidder, uint256 amount);
    event AuctionSettled(uint256 indexed auctionId, address indexed winner, uint256 amount);
    event AuctionCanceled(uint256 indexed auctionId, address indexed seller);
    event PlatformFeeUpdated(uint256 newFeePercentage);
    event FundsWithdrawn(address indexed user, uint256 amount);
}

// Error definitions
sol! {
    error AlreadyInitialized();
    error AuctionNotFound();
    error AuctionNotActive();
    error BidTooLow();
    error AuctionNotEnded();
    error AuctionAlreadySettled();
    error NotTokenOwner();
    error NotAuctionSeller();
    error NotPlatformOwner();
    error InsufficientBalance();
    error TransferFailed();
    error AuctionHasBids();
    error InvalidDuration();
    error InvalidReservePrice();
    error InvalidFeePercentage();
    error ERC721InvalidTokenId();
    error NotApprovedForTransfer();
}

#[derive(SolidityError)]
pub enum MarketplaceError {
    AlreadyInitialized(AlreadyInitialized),
    AuctionNotFound(AuctionNotFound),
    AuctionNotActive(AuctionNotActive),
    BidTooLow(BidTooLow),
    AuctionNotEnded(AuctionNotEnded),
    AuctionAlreadySettled(AuctionAlreadySettled),
    NotTokenOwner(NotTokenOwner),
    NotAuctionSeller(NotAuctionSeller),
    NotPlatformOwner(NotPlatformOwner),
    InsufficientBalance(InsufficientBalance),
    TransferFailed(TransferFailed),
    AuctionHasBids(AuctionHasBids),
    InvalidDuration(InvalidDuration),
    InvalidReservePrice(InvalidReservePrice),
    InvalidFeePercentage(InvalidFeePercentage),
    ERC721InvalidTokenId(ERC721InvalidTokenId),
    NotApprovedForTransfer(NotApprovedForTransfer),
}

// Auction structure
sol_storage! {
    pub struct Auction {
        address nft_contract;     // NFT contract address
        uint256 token_id;        // NFT token ID
        address seller;          // NFT seller
        uint256 reserve_price;   // Minimum bid amount
        uint256 current_bid;     // Current highest bid
        address current_bidder;  // Current highest bidder
        uint256 end_time;        // Auction end timestamp
        bool settled;            // Whether auction is settled
    }
}

// Main marketplace contract
sol_storage! {
    #[entrypoint]
    pub struct NeonMarketplace {
        // Contract initialization
        bool initialized;

        // Marketplace auctions
        uint256 next_auction_id;
        mapping(uint256 => Auction) auctions;           // auctionId => Auction

        // Platform fees and balances
        uint256 platform_fee_percentage;                // 5% = 500 (basis points)
        address platform_owner;
        mapping(address => uint256) user_balances;      // withdrawable balances
    }
}


const ONE_DAY: u64 = 86400; // 24 hours in seconds

#[public]
impl NeonMarketplace {

    /// Initialize the marketplace contract
    pub fn initialize(&mut self, platform_fee_percentage: U256) -> Result<(), MarketplaceError> {
        if self.initialized.get() {
            return Err(MarketplaceError::AlreadyInitialized(AlreadyInitialized{}));
        }

        // Validate fee percentage (max 10% = 1000 basis points)
        if platform_fee_percentage > U256::from(1000) {
            return Err(MarketplaceError::InvalidFeePercentage(InvalidFeePercentage{}));
        }

        self.initialized.set(true);
        self.next_auction_id.set(U256::from(1));
        self.platform_fee_percentage.set(platform_fee_percentage);
        self.platform_owner.set(self.vm().msg_sender());

        Ok(())
    }

    /// Create auction for existing NFT (from any ERC721 contract)
    pub fn create_auction(
        &mut self,
        nft_contract: Address,
        token_id: U256,
        reserve_price: U256,
        duration: U256,
    ) -> Result<U256, MarketplaceError> {
        // Validate inputs
        if reserve_price == U256::ZERO {
            return Err(MarketplaceError::InvalidReservePrice(InvalidReservePrice{}));
        }

        if duration == U256::ZERO || duration > U256::from(30 * ONE_DAY) {
            return Err(MarketplaceError::InvalidDuration(InvalidDuration{}));
        }

        // Check if caller owns the NFT using static call
        let nft = IERC721::new(nft_contract);
        let owner = nft.owner_of(Call::new(), token_id).map_err(|_| MarketplaceError::ERC721InvalidTokenId(ERC721InvalidTokenId{}))?;

        if owner != self.vm().msg_sender() {
            return Err(MarketplaceError::NotTokenOwner(NotTokenOwner{}));
        }

        // Check if marketplace is approved to transfer this NFT
        let approved = nft.get_approved(Call::new(), token_id).map_err(|_| MarketplaceError::ERC721InvalidTokenId(ERC721InvalidTokenId{}))?;
        let is_approved_for_all = nft.is_approved_for_all(Call::new(), self.vm().msg_sender(), self.vm().contract_address()).map_err(|_| MarketplaceError::ERC721InvalidTokenId(ERC721InvalidTokenId{}))?;

        if approved != self.vm().contract_address() && !is_approved_for_all {
            return Err(MarketplaceError::NotApprovedForTransfer(NotApprovedForTransfer{}));
        }

        let auction_id = self.next_auction_id.get();
        let end_time = U256::from(self.vm().block_timestamp()) + duration;
        let sender = self.vm().msg_sender();

        // Create auction
        let mut auction = self.auctions.setter(auction_id);
        auction.nft_contract.set(nft_contract);
        auction.token_id.set(token_id);
        auction.seller.set(sender);
        auction.reserve_price.set(reserve_price);
        auction.current_bid.set(U256::ZERO);
        auction.current_bidder.set(Address::ZERO);
        auction.end_time.set(end_time);
        auction.settled.set(false);

        // Transfer NFT to contract
        nft.transfer_from(Call::new(), self.vm().msg_sender(), self.vm().contract_address(), token_id).map_err(|_| MarketplaceError::TransferFailed(TransferFailed{}))?;

        // Increment auction ID
        self.next_auction_id.set(auction_id + U256::from(1));

        // Emit event
        evm::log(AuctionCreated {
            auctionId: auction_id,
            nftContract: nft_contract,
            tokenId: token_id,
            reservePrice: reserve_price,
            endTime: end_time,
        });

        Ok(auction_id)
    }

    /// Cancel an auction (only if no bids placed)
    pub fn cancel_auction(&mut self, auction_id: U256) -> Result<(), MarketplaceError> {
        let auction = self.auctions.getter(auction_id);

        // Check if auction exists
        if auction.seller.get() == Address::ZERO {
            return Err(MarketplaceError::AuctionNotFound(AuctionNotFound{}));
        }

        // Check if caller is the seller
        if auction.seller.get() != self.vm().msg_sender() {
            return Err(MarketplaceError::NotAuctionSeller(NotAuctionSeller{}));
        }

        // Check if auction is already settled
        if auction.settled.get() {
            return Err(MarketplaceError::AuctionAlreadySettled(AuctionAlreadySettled{}));
        }

        // Check if auction has bids (can only cancel if no bids)
        if auction.current_bidder.get() != Address::ZERO {
            return Err(MarketplaceError::AuctionHasBids(AuctionHasBids{}));
        }

        let nft_contract = auction.nft_contract.get();
        let token_id = auction.token_id.get();
        let seller = auction.seller.get();
        let contract_addr = self.vm().contract_address();

        // Mark auction as settled (cancelled)
        self.auctions.setter(auction_id).settled.set(true);

        // Return NFT to seller
        let nft = IERC721::new(nft_contract);
        nft.transfer_from(Call::new(), contract_addr, seller, token_id).map_err(|_| MarketplaceError::TransferFailed(TransferFailed{}))?;

        // Emit cancellation event
        evm::log(AuctionCanceled {
            auctionId: auction_id,
            seller: seller,
        });

        Ok(())
    }

    /// Place a bid on an auction
    #[payable]
    pub fn place_bid(&mut self, auction_id: U256) -> Result<(), MarketplaceError> {
        let auction = self.auctions.getter(auction_id);

        // Check if auction exists
        if auction.seller.get() == Address::ZERO {
            return Err(MarketplaceError::AuctionNotFound(AuctionNotFound{}));
        }

        // Check if auction is active
        if U256::from(self.vm().block_timestamp()) >= auction.end_time.get() || auction.settled.get() {
            return Err(MarketplaceError::AuctionNotActive(AuctionNotActive{}));
        }

        let bid_amount = self.vm().msg_value();
        let current_bid = auction.current_bid.get();
        let reserve_price = auction.reserve_price.get();
        let sender = self.vm().msg_sender();

        // Check if bid meets minimum requirements
        let min_bid = if current_bid == U256::ZERO {
            reserve_price
        } else {
            current_bid + (current_bid / U256::from(20)) // 5% increment
        };

        if bid_amount < min_bid {
            return Err(MarketplaceError::BidTooLow(BidTooLow{}));
        }

        // Refund previous bidder
        let previous_bidder = auction.current_bidder.get();
        if previous_bidder != Address::ZERO {
            let previous_balance = self.user_balances.getter(previous_bidder).get();
            self.user_balances.setter(previous_bidder).set(previous_balance + current_bid);
        }

        // Update auction with new bid
        let mut auction_mut = self.auctions.setter(auction_id);
        auction_mut.current_bid.set(bid_amount);
        auction_mut.current_bidder.set(sender);

        // Emit event
        evm::log(BidPlaced {
            auctionId: auction_id,
            bidder: sender,
            amount: bid_amount,
        });

        Ok(())
    }

    /// Settle an auction after it ends (can be called by anyone)
    pub fn settle_auction(&mut self, auction_id: U256) -> Result<(), MarketplaceError> {
        // Get auction details in one read
        let auction = self.auctions.getter(auction_id);
        let nft_contract = auction.nft_contract.get();
        let token_id = auction.token_id.get();
        let seller = auction.seller.get();
        let reserve_price = auction.reserve_price.get();
        let current_bid = auction.current_bid.get();
        let current_bidder = auction.current_bidder.get();
        let end_time = auction.end_time.get();
        let settled = auction.settled.get();

        // Check if auction exists
        if seller == Address::ZERO {
            return Err(MarketplaceError::AuctionNotFound(AuctionNotFound{}));
        }

        // Check if auction has ended
        if U256::from(self.vm().block_timestamp()) < end_time {
            return Err(MarketplaceError::AuctionNotEnded(AuctionNotEnded{}));
        }

        // Check if already settled
        if settled {
            return Err(MarketplaceError::AuctionAlreadySettled(AuctionAlreadySettled{}));
        }

        // Mark as settled
        self.auctions.setter(auction_id).settled.set(true);

        if current_bidder != Address::ZERO && current_bid >= reserve_price {
            // Calculate platform fee using stored percentage
            let platform_fee = (current_bid * self.platform_fee_percentage.get()) / U256::from(10000);
            let seller_amount = current_bid - platform_fee;

            // Transfer NFT to winner
            let nft = IERC721::new(nft_contract);
            nft.transfer_from(Call::new(), self.vm().contract_address(), current_bidder, token_id).map_err(|_| MarketplaceError::TransferFailed(TransferFailed{}))?;

            // Add seller proceeds to withdrawable balance
            let seller_balance = self.user_balances.getter(seller).get();
            self.user_balances.setter(seller).set(seller_balance + seller_amount);

            // Add platform fee to platform owner balance
            let platform_owner = self.platform_owner.get();
            let platform_balance = self.user_balances.getter(platform_owner).get();
            self.user_balances.setter(platform_owner).set(platform_balance + platform_fee);

            // Emit settlement event
            evm::log(AuctionSettled {
                auctionId: auction_id,
                winner: current_bidder,
                amount: current_bid,
            });
        } else {
            // No valid bids - return NFT to seller
            let nft = IERC721::new(nft_contract);
            nft.transfer_from(Call::new(), self.vm().contract_address(), seller, token_id).map_err(|_| MarketplaceError::TransferFailed(TransferFailed{}))?;

            // Emit settlement event with no winner
            evm::log(AuctionSettled {
                auctionId: auction_id,
                winner: Address::ZERO,
                amount: U256::ZERO,
            });
        }

        Ok(())
    }

    /// Update platform fee percentage (only platform owner)
    pub fn update_platform_fee(&mut self, new_fee_percentage: U256) -> Result<(), MarketplaceError> {
        // Check if caller is platform owner
        if self.vm().msg_sender() != self.platform_owner.get() {
            return Err(MarketplaceError::NotPlatformOwner(NotPlatformOwner{}));
        }

        // Validate fee percentage (max 10% = 1000 basis points)
        if new_fee_percentage > U256::from(1000) {
            return Err(MarketplaceError::InvalidFeePercentage(InvalidFeePercentage{}));
        }

        self.platform_fee_percentage.set(new_fee_percentage);

        // Emit event
        evm::log(PlatformFeeUpdated {
            newFeePercentage: new_fee_percentage,
        });

        Ok(())
    }

    /// Withdraw accumulated funds
    pub fn withdraw(&mut self) -> Result<(), MarketplaceError> {
        let sender = self.vm().msg_sender();
        let balance = self.user_balances.getter(sender).get();

        if balance == U256::ZERO {
            return Err(MarketplaceError::InsufficientBalance(InsufficientBalance{}));
        }

        // Reset balance before transfer (reentrancy protection)
        self.user_balances.setter(sender).set(U256::ZERO);

        // Transfer funds using vm().transfer_eth
        match self.vm().transfer_eth(sender, balance) {
            Ok(_) => {
                evm::log(FundsWithdrawn {
                    user: sender,
                    amount: balance,
                });
                Ok(())
            }
            Err(_) => {
                // Restore balance on failed transfer
                self.user_balances.setter(sender).set(balance);
                Err(MarketplaceError::TransferFailed(TransferFailed{}))
            }
        }
    }

    /// Get auction details
    pub fn get_auction(&self, auction_id: U256) -> Result<(Address, U256, Address, U256, U256, Address, U256, bool), MarketplaceError> {
        let auction = self.auctions.getter(auction_id);

        if auction.seller.get() == Address::ZERO {
            return Err(MarketplaceError::AuctionNotFound(AuctionNotFound{}));
        }

        Ok((
            auction.nft_contract.get(),
            auction.token_id.get(),
            auction.seller.get(),
            auction.reserve_price.get(),
            auction.current_bid.get(),
            auction.current_bidder.get(),
            auction.end_time.get(),
            auction.settled.get(),
        ))
    }

    /// Check if auction is active
    pub fn is_auction_active(&self, auction_id: U256) -> Result<bool, MarketplaceError> {
        let auction = self.auctions.getter(auction_id);

        if auction.seller.get() == Address::ZERO {
            return Ok(false);
        }

        let is_active = U256::from(self.vm().block_timestamp()) < auction.end_time.get() && !auction.settled.get();
        Ok(is_active)
    }

    /// Get user's withdrawable balance
    pub fn get_balance(&self, user_address: Address) -> Result<U256, MarketplaceError> {
        Ok(self.user_balances.getter(user_address).get())
    }

    /// Get next auction ID
    pub fn get_next_auction_id(&self) -> Result<U256, MarketplaceError> {
        Ok(self.next_auction_id.get())
    }

    /// Get platform fee percentage
    pub fn get_platform_fee(&self) -> Result<U256, MarketplaceError> {
        Ok(self.platform_fee_percentage.get())
    }

    /// Get platform owner
    pub fn get_platform_owner(&self) -> Result<Address, MarketplaceError> {
        Ok(self.platform_owner.get())
    }

    /// Get collection information for a token (if using multi-collection NFT)
    pub fn get_token_collection_info(&self, nft_contract: Address, token_id: U256) -> Result<(U256, String, String, Address), MarketplaceError> {
        // Try to call the multi-collection NFT interface
        let multi_nft = IMultiCollectionNFT::new(nft_contract);

        match multi_nft.token_collection(Call::new(), token_id) {
            Ok(collection_id) => {
                match multi_nft.get_collection(Call::new(), collection_id) {
                    Ok((name, symbol, creator, _base_uri, _next_token_id)) => {
                        Ok((collection_id, name, symbol, creator))
                    }
                    Err(_) => Err(MarketplaceError::ERC721InvalidTokenId(ERC721InvalidTokenId{}))
                }
            }
            Err(_) => {
                // Not a multi-collection NFT, return default values
                Ok((U256::ZERO, "Unknown Collection".to_string(), "UNK".to_string(), Address::ZERO))
            }
        }
    }

    /// Get platform fee percentage
    pub fn get_platform_fee_percentage(&self) -> Result<U256, MarketplaceError> {
        Ok(self.platform_fee_percentage.get())
    }
}