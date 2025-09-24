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

// ERC721 Events
sol! {
    event Transfer(address indexed from, address indexed to, uint256 indexed tokenId);
    event Approval(address indexed owner, address indexed approved, uint256 indexed tokenId);
    event ApprovalForAll(address indexed owner, address indexed operator, bool approved);
}

// NFT Events
sol! {
    event NFTMinted(uint256 indexed tokenId, uint256 indexed collectionId, address indexed creator, string tokenURI);
    event CollectionCreated(uint256 indexed collectionId, address indexed creator, string name, string symbol);
}

// Error definitions
sol! {
    error AlreadyInitialized();
    error InvalidTokenURI();
    error InvalidCollectionName();
    error InvalidCollectionId();
    error NotCollectionCreator();
    error ERC721InvalidTokenId();
    error ERC721InvalidSender();
    error ERC721InvalidReceiver();
    error ERC721InsufficientApproval();
    error ERC721InvalidApprover();
    error ERC721InvalidOperator();
}

#[derive(SolidityError)]
pub enum NFTError {
    AlreadyInitialized(AlreadyInitialized),
    InvalidTokenURI(InvalidTokenURI),
    InvalidCollectionName(InvalidCollectionName),
    InvalidCollectionId(InvalidCollectionId),
    NotCollectionCreator(NotCollectionCreator),
    ERC721InvalidTokenId(ERC721InvalidTokenId),
    ERC721InvalidSender(ERC721InvalidSender),
    ERC721InvalidReceiver(ERC721InvalidReceiver),
    ERC721InsufficientApproval(ERC721InsufficientApproval),
    ERC721InvalidApprover(ERC721InvalidApprover),
    ERC721InvalidOperator(ERC721InvalidOperator),
}

// Main multi-collection NFT contract
sol_storage! {
    #[entrypoint]
    pub struct MultiCollectionNFT {
        // Contract initialization
        bool initialized;

        // Platform info (for contract-level queries)
        string platform_name;                          // "Neon Multi-Collection NFT"
        string platform_symbol;                        // "NEON-MULTI"

        // Collection management
        uint256 next_collection_id;                    // Next collection ID
        mapping(uint256 => string) collection_names;   // collectionId => name
        mapping(uint256 => string) collection_symbols; // collectionId => symbol
        mapping(uint256 => address) collection_creators; // collectionId => creator
        mapping(uint256 => string) collection_base_uris; // collectionId => base_uri
        mapping(uint256 => uint256) collection_next_token_ids; // collectionId => next_token_id
        mapping(uint256 => bool) collection_exists;    // collectionId => exists

        // Global token management
        uint256 next_global_token_id;                  // Global token ID counter
        mapping(uint256 => uint256) token_collections; // tokenId => collectionId
        mapping(uint256 => address) owners;            // tokenId => owner
        mapping(address => uint256) balances;          // owner => total balance across all collections
        mapping(uint256 => address) token_approvals;   // tokenId => approved address
        mapping(address => mapping(address => bool)) operator_approvals; // owner => (operator => approved)
        mapping(uint256 => string) token_uris;         // tokenId => metadata URI

        // Collection-specific balances
        mapping(address => mapping(uint256 => uint256)) collection_balances; // owner => collectionId => balance
    }
}

#[public]
impl MultiCollectionNFT {

    /// Initialize the multi-collection NFT contract
    pub fn initialize(&mut self) -> Result<(), NFTError> {
        if self.initialized.get() {
            return Err(NFTError::AlreadyInitialized(AlreadyInitialized{}));
        }

        self.initialized.set(true);
        self.platform_name.set_str("Neon Multi-Collection NFT".to_string());
        self.platform_symbol.set_str("NEON-MULTI".to_string());
        self.next_collection_id.set(U256::from(1));
        self.next_global_token_id.set(U256::from(1));

        Ok(())
    }

    /// Returns the platform name
    pub fn name(&self) -> Result<String, NFTError> {
        Ok(self.platform_name.get_string())
    }

    /// Returns the platform symbol
    pub fn symbol(&self) -> Result<String, NFTError> {
        Ok(self.platform_symbol.get_string())
    }

    /// Create a new NFT collection
    pub fn create_collection(
        &mut self,
        name: String,
        symbol: String,
        base_uri: String
    ) -> Result<U256, NFTError> {
        if !self.initialized.get() {
            return Err(NFTError::AlreadyInitialized(AlreadyInitialized{})); // Reusing error for not initialized
        }

        if name.is_empty() || symbol.is_empty() {
            return Err(NFTError::InvalidCollectionName(InvalidCollectionName{}));
        }

        let collection_id = self.next_collection_id.get();
        let creator = self.vm().msg_sender();

        // Store collection info
        self.collection_names.setter(collection_id).set_str(name.clone());
        self.collection_symbols.setter(collection_id).set_str(symbol.clone());
        self.collection_creators.setter(collection_id).set(creator);
        self.collection_base_uris.setter(collection_id).set_str(base_uri.clone());
        self.collection_next_token_ids.setter(collection_id).set(U256::from(1));
        self.collection_exists.setter(collection_id).set(true);
        self.next_collection_id.set(collection_id + U256::from(1));

        evm::log(CollectionCreated {
            collectionId: collection_id,
            creator,
            name,
            symbol,
        });

        Ok(collection_id)
    }

    /// Get collection information
    pub fn get_collection(&self, collection_id: U256) -> Result<(String, String, Address, String, U256), NFTError> {
        if !self.collection_exists.getter(collection_id).get() {
            return Err(NFTError::InvalidCollectionId(InvalidCollectionId{}));
        }
        Ok((
            self.collection_names.getter(collection_id).get_string(),
            self.collection_symbols.getter(collection_id).get_string(),
            self.collection_creators.getter(collection_id).get(),
            self.collection_base_uris.getter(collection_id).get_string(),
            self.collection_next_token_ids.getter(collection_id).get(),
        ))
    }

    /// Returns the total number of tokens in owner's account across all collections
    pub fn balance_of(&self, owner: Address) -> Result<U256, NFTError> {
        if owner == Address::ZERO {
            return Err(NFTError::ERC721InvalidSender(ERC721InvalidSender{}));
        }
        Ok(self.balances.getter(owner).get())
    }

    /// Returns the number of tokens in owner's account for a specific collection
    pub fn balance_of_collection(&self, owner: Address, collection_id: U256) -> Result<U256, NFTError> {
        if owner == Address::ZERO {
            return Err(NFTError::ERC721InvalidSender(ERC721InvalidSender{}));
        }
        if !self.collection_exists.getter(collection_id).get() {
            return Err(NFTError::InvalidCollectionId(InvalidCollectionId{}));
        }
        Ok(self.collection_balances.getter(owner).getter(collection_id).get())
    }

    /// Returns the owner of the tokenId token
    pub fn owner_of(&self, token_id: U256) -> Result<Address, NFTError> {
        let owner = self.owners.getter(token_id).get();
        if owner == Address::ZERO {
            return Err(NFTError::ERC721InvalidTokenId(ERC721InvalidTokenId{}));
        }
        Ok(owner)
    }

    /// Returns the token URI
    pub fn token_uri(&self, token_id: U256) -> Result<String, NFTError> {
        if !self._exists(token_id) {
            return Err(NFTError::ERC721InvalidTokenId(ERC721InvalidTokenId{}));
        }
        Ok(self.token_uris.getter(token_id).get_string())
    }

    /// Returns the collection ID for a token
    pub fn token_collection(&self, token_id: U256) -> Result<U256, NFTError> {
        if !self._exists(token_id) {
            return Err(NFTError::ERC721InvalidTokenId(ERC721InvalidTokenId{}));
        }
        Ok(self.token_collections.getter(token_id).get())
    }

    /// Returns if the token exists
    fn _exists(&self, token_id: U256) -> bool {
        self.owners.getter(token_id).get() != Address::ZERO
    }

    /// Approve or remove operator for a token
    pub fn approve(&mut self, to: Address, token_id: U256) -> Result<(), NFTError> {
        let owner = self.owner_of(token_id)?;

        if to == owner {
            return Err(NFTError::ERC721InvalidApprover(ERC721InvalidApprover{}));
        }

        if self.vm().msg_sender() != owner && !self.is_approved_for_all(owner, self.vm().msg_sender())? {
            return Err(NFTError::ERC721InsufficientApproval(ERC721InsufficientApproval{}));
        }

        self.token_approvals.setter(token_id).set(to);
        evm::log(Approval {
            owner: owner,
            approved: to,
            tokenId: token_id,
        });

        Ok(())
    }

    /// Returns the account approved for tokenId token
    pub fn get_approved(&self, token_id: U256) -> Result<Address, NFTError> {
        if !self._exists(token_id) {
            return Err(NFTError::ERC721InvalidTokenId(ERC721InvalidTokenId{}));
        }
        Ok(self.token_approvals.getter(token_id).get())
    }

    /// Approve or remove operator for all tokens
    pub fn set_approval_for_all(&mut self, operator: Address, approved: bool) -> Result<(), NFTError> {
        if self.vm().msg_sender() == operator {
            return Err(NFTError::ERC721InvalidOperator(ERC721InvalidOperator{}));
        }

        self.operator_approvals.setter(self.vm().msg_sender()).setter(operator).set(approved);
        evm::log(ApprovalForAll {
            owner: self.vm().msg_sender(),
            operator: operator,
            approved: approved,
        });

        Ok(())
    }

    /// Returns if the operator is approved for all tokens
    pub fn is_approved_for_all(&self, owner: Address, operator: Address) -> Result<bool, NFTError> {
        Ok(self.operator_approvals.getter(owner).getter(operator).get())
    }

    /// Transfer token from one address to another
    pub fn transfer_from(&mut self, from: Address, to: Address, token_id: U256) -> Result<(), NFTError> {
        if !self._is_approved_or_owner(self.vm().msg_sender(), token_id)? {
            return Err(NFTError::ERC721InsufficientApproval(ERC721InsufficientApproval{}));
        }

        self._transfer(from, to, token_id)?;
        Ok(())
    }

    /// Safely transfer token from one address to another
    pub fn safe_transfer_from(&mut self, from: Address, to: Address, token_id: U256) -> Result<(), NFTError> {
        self.transfer_from(from, to, token_id)?;
        Ok(())
    }

    /// Safely transfer token from one address to another with data
    pub fn safe_transfer_from_with_data(&mut self, from: Address, to: Address, token_id: U256, _data: String) -> Result<(), NFTError> {
        self.transfer_from(from, to, token_id)?;
        Ok(())
    }

    /// Internal transfer function
    fn _transfer(&mut self, from: Address, to: Address, token_id: U256) -> Result<(), NFTError> {
        if from == Address::ZERO {
            return Err(NFTError::ERC721InvalidSender(ERC721InvalidSender{}));
        }
        if to == Address::ZERO {
            return Err(NFTError::ERC721InvalidReceiver(ERC721InvalidReceiver{}));
        }

        // Verify ownership - CRITICAL FIX
        let actual_owner = self.owner_of(token_id)?;
        if from != actual_owner {
            return Err(NFTError::ERC721InvalidSender(ERC721InvalidSender{}));
        }

        // Get collection ID for this token
        let collection_id = self.token_collections.getter(token_id).get();

        // Clear approvals
        self._clear_approval(token_id);

        // Update total balances with underflow protection - CRITICAL FIX
        let from_balance = self.balances.getter(from).get();
        if from_balance == U256::ZERO {
            return Err(NFTError::ERC721InvalidSender(ERC721InvalidSender{}));
        }
        self.balances.setter(from).set(from_balance - U256::from(1));

        let to_balance = self.balances.getter(to).get();
        self.balances.setter(to).set(to_balance + U256::from(1));

        // Update collection-specific balances with underflow protection - CRITICAL FIX
        let from_collection_balance = self.collection_balances.getter(from).getter(collection_id).get();
        if from_collection_balance == U256::ZERO {
            return Err(NFTError::ERC721InvalidSender(ERC721InvalidSender{}));
        }
        self.collection_balances.setter(from).setter(collection_id).set(from_collection_balance - U256::from(1));

        let to_collection_balance = self.collection_balances.getter(to).getter(collection_id).get();
        self.collection_balances.setter(to).setter(collection_id).set(to_collection_balance + U256::from(1));

        // Update ownership
        self.owners.setter(token_id).set(to);

        evm::log(Transfer {
            from: from,
            to: to,
            tokenId: token_id,
        });

        Ok(())
    }

    /// Internal clear approval function
    fn _clear_approval(&mut self, token_id: U256) {
        self.token_approvals.setter(token_id).set(Address::ZERO);
    }

    /// Check if address is approved or owner
    fn _is_approved_or_owner(&self, spender: Address, token_id: U256) -> Result<bool, NFTError> {
        if !self._exists(token_id) {
            return Err(NFTError::ERC721InvalidTokenId(ERC721InvalidTokenId{}));
        }

        let owner = self.owner_of(token_id)?;
        Ok(spender == owner ||
           self.get_approved(token_id)? == spender ||
           self.is_approved_for_all(owner, spender)?)
    }

    /// Mint a new NFT in a specific collection (only collection creator can mint)
    pub fn mint_nft(&mut self, collection_id: U256, token_uri: String) -> Result<U256, NFTError> {
        if !self.initialized.get() {
            return Err(NFTError::AlreadyInitialized(AlreadyInitialized{})); // Reusing error for not initialized
        }

        if token_uri.is_empty() {
            return Err(NFTError::InvalidTokenURI(InvalidTokenURI{}));
        }

        if !self.collection_exists.getter(collection_id).get() {
            return Err(NFTError::InvalidCollectionId(InvalidCollectionId{}));
        }

        // CRITICAL FIX: Only collection creator can mint
        let collection_creator = self.collection_creators.getter(collection_id).get();
        if self.vm().msg_sender() != collection_creator {
            return Err(NFTError::NotCollectionCreator(NotCollectionCreator{}));
        }

        let token_id = self.next_global_token_id.get();
        let to = self.vm().msg_sender();

        if to == Address::ZERO {
            return Err(NFTError::ERC721InvalidReceiver(ERC721InvalidReceiver{}));
        }

        // Set token data
        self.token_uris.setter(token_id).set_str(token_uri.clone());
        self.owners.setter(token_id).set(to);
        self.token_collections.setter(token_id).set(collection_id);

        // Update balances
        let total_balance = self.balances.getter(to).get();
        self.balances.setter(to).set(total_balance + U256::from(1));

        let collection_balance = self.collection_balances.getter(to).getter(collection_id).get();
        self.collection_balances.setter(to).setter(collection_id).set(collection_balance + U256::from(1));

        // Update collection's next token ID
        let current_next_token = self.collection_next_token_ids.getter(collection_id).get();
        self.collection_next_token_ids.setter(collection_id).set(current_next_token + U256::from(1));

        // Increment global token ID
        self.next_global_token_id.set(token_id + U256::from(1));

        // Emit events
        evm::log(Transfer {
            from: Address::ZERO,
            to: to,
            tokenId: token_id,
        });

        evm::log(NFTMinted {
            tokenId: token_id,
            collectionId: collection_id,
            creator: to,
            tokenURI: token_uri,
        });

        Ok(token_id)
    }

    /// Get next global token ID
    pub fn get_next_token_id(&self) -> Result<U256, NFTError> {
        Ok(self.next_global_token_id.get())
    }

    /// Get next collection ID
    pub fn get_next_collection_id(&self) -> Result<U256, NFTError> {
        Ok(self.next_collection_id.get())
    }

    /// Public mint function - allows anyone to mint in a collection (alternative to creator-only mint)
    pub fn public_mint_nft(&mut self, collection_id: U256, token_uri: String) -> Result<U256, NFTError> {
        if !self.initialized.get() {
            return Err(NFTError::AlreadyInitialized(AlreadyInitialized{})); // Reusing error for not initialized
        }

        if token_uri.is_empty() {
            return Err(NFTError::InvalidTokenURI(InvalidTokenURI{}));
        }

        if !self.collection_exists.getter(collection_id).get() {
            return Err(NFTError::InvalidCollectionId(InvalidCollectionId{}));
        }

        let token_id = self.next_global_token_id.get();
        let to = self.vm().msg_sender();

        if to == Address::ZERO {
            return Err(NFTError::ERC721InvalidReceiver(ERC721InvalidReceiver{}));
        }

        // Set token data
        self.token_uris.setter(token_id).set_str(token_uri.clone());
        self.owners.setter(token_id).set(to);
        self.token_collections.setter(token_id).set(collection_id);

        // Update balances
        let total_balance = self.balances.getter(to).get();
        self.balances.setter(to).set(total_balance + U256::from(1));

        let collection_balance = self.collection_balances.getter(to).getter(collection_id).get();
        self.collection_balances.setter(to).setter(collection_id).set(collection_balance + U256::from(1));

        // Update collection's next token ID
        let current_next_token = self.collection_next_token_ids.getter(collection_id).get();
        self.collection_next_token_ids.setter(collection_id).set(current_next_token + U256::from(1));

        // Increment global token ID
        self.next_global_token_id.set(token_id + U256::from(1));

        // Emit events
        evm::log(Transfer {
            from: Address::ZERO,
            to: to,
            tokenId: token_id,
        });

        evm::log(NFTMinted {
            tokenId: token_id,
            collectionId: collection_id,
            creator: to,
            tokenURI: token_uri,
        });

        Ok(token_id)
    }
}

