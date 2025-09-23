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
    event NFTMinted(uint256 indexed tokenId, address indexed creator, string tokenURI);
}

// Error definitions
sol! {
    error AlreadyInitialized();
    error InvalidTokenURI();
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
    ERC721InvalidTokenId(ERC721InvalidTokenId),
    ERC721InvalidSender(ERC721InvalidSender),
    ERC721InvalidReceiver(ERC721InvalidReceiver),
    ERC721InsufficientApproval(ERC721InsufficientApproval),
    ERC721InvalidApprover(ERC721InvalidApprover),
    ERC721InvalidOperator(ERC721InvalidOperator),
}

// Main NFT contract
sol_storage! {
    #[entrypoint]
    pub struct NeonNFT {
        // Contract initialization
        bool initialized;

        // ERC721 Implementation
        string name;                                    // "Neon NFT Collection"
        string symbol;                                  // "NEON"
        uint256 next_token_id;                          // Next token ID to mint
        mapping(uint256 => address) owners;             // tokenId => owner
        mapping(address => uint256) balances;           // owner => balance
        mapping(uint256 => address) token_approvals;    // tokenId => approved address
        mapping(address => mapping(address => bool)) operator_approvals; // owner => (operator => approved)
        mapping(uint256 => string) token_uris;          // tokenId => metadata URI
    }
}

#[public]
impl NeonNFT {

    /// Initialize the NFT contract
    pub fn initialize(&mut self) -> Result<(), NFTError> {
        if self.initialized.get() {
            return Err(NFTError::AlreadyInitialized(AlreadyInitialized{}));
        }

        self.initialized.set(true);
        self.name.set_str("Neon NFT Collection".to_string());
        self.symbol.set_str("NEON".to_string());
        self.next_token_id.set(U256::from(1));

        Ok(())
    }

    /// Returns the name of the token collection
    pub fn name(&self) -> Result<String, NFTError> {
        Ok(self.name.get_string())
    }

    /// Returns the symbol of the token collection
    pub fn symbol(&self) -> Result<String, NFTError> {
        Ok(self.symbol.get_string())
    }

    /// Returns the number of tokens in owner's account
    pub fn balance_of(&self, owner: Address) -> Result<U256, NFTError> {
        if owner == Address::ZERO {
            return Err(NFTError::ERC721InvalidSender(ERC721InvalidSender{}));
        }
        Ok(self.balances.getter(owner).get())
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

        // Clear approvals
        self._clear_approval(token_id);

        // Update balances
        let from_balance = self.balances.getter(from).get();
        self.balances.setter(from).set(from_balance - U256::from(1));

        let to_balance = self.balances.getter(to).get();
        self.balances.setter(to).set(to_balance + U256::from(1));

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

    /// Mint a new NFT with metadata URI
    pub fn mint_nft(&mut self, token_uri: String) -> Result<U256, NFTError> {
        if token_uri.is_empty() {
            return Err(NFTError::InvalidTokenURI(InvalidTokenURI{}));
        }

        let token_id = self.next_token_id.get();
        let to = self.vm().msg_sender();

        if to == Address::ZERO {
            return Err(NFTError::ERC721InvalidReceiver(ERC721InvalidReceiver{}));
        }

        // Set token data
        self.token_uris.setter(token_id).set_str(token_uri.clone());
        self.owners.setter(token_id).set(to);

        // Update balance
        let balance = self.balances.getter(to).get();
        self.balances.setter(to).set(balance + U256::from(1));

        // Increment next token ID
        self.next_token_id.set(token_id + U256::from(1));

        // Emit events
        evm::log(Transfer {
            from: Address::ZERO,
            to: to,
            tokenId: token_id,
        });

        evm::log(NFTMinted {
            tokenId: token_id,
            creator: to,
            tokenURI: token_uri,
        });

        Ok(token_id)
    }

    /// Get next token ID
    pub fn get_next_token_id(&self) -> Result<U256, NFTError> {
        Ok(self.next_token_id.get())
    }
}

