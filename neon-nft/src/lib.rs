// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]
#![cfg_attr(not(any(test, feature = "export-abi")), no_std)]

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;
use stylus_sdk::{
    alloy_primitives::{Address, U256},
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
    event NFTMinted(uint256 indexed tokenId, address indexed to, string tokenURI);
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

// Single-collection NFT contract
sol_storage! {
    #[entrypoint]
    pub struct SimpleNFT {
        // Contract initialization
        bool initialized;

        // NFT metadata
        string name;
        string symbol;

        // Token management
        uint256 next_token_id;
        mapping(uint256 => address) owners;
        mapping(address => uint256) balances;
        mapping(uint256 => address) token_approvals;
        mapping(address => mapping(address => bool)) operator_approvals;
        mapping(uint256 => string) token_uris;
    }
}

#[public]
impl SimpleNFT {

    /// Initialize the NFT contract
    pub fn initialize(&mut self, name: String, symbol: String) -> Result<(), NFTError> {
        if self.initialized.get() {
            return Err(NFTError::AlreadyInitialized(AlreadyInitialized{}));
        }

        self.initialized.set(true);
        self.name.set_str(name);
        self.symbol.set_str(symbol);
        self.next_token_id.set(U256::from(1));

        Ok(())
    }

    /// Returns the token collection name
    pub fn name(&self) -> Result<String, NFTError> {
        Ok(self.name.get_string())
    }

    /// Returns the token collection symbol
    pub fn symbol(&self) -> Result<String, NFTError> {
        Ok(self.symbol.get_string())
    }

    /// Returns the total number of tokens in owner's account
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
        log(self.vm(), Approval {
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
        log(self.vm(), ApprovalForAll {
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
        let owner = self.owner_of(token_id)?;
        let sender = self.vm().msg_sender();

        // Check if sender is owner, approved, or operator
        if sender != owner &&
           self.get_approved(token_id)? != sender &&
           !self.is_approved_for_all(owner, sender)? {
            return Err(NFTError::ERC721InsufficientApproval(ERC721InsufficientApproval{}));
        }

        self._transfer(from, to, token_id)?;
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

        // Verify ownership
        let actual_owner = self.owner_of(token_id)?;
        if from != actual_owner {
            return Err(NFTError::ERC721InvalidSender(ERC721InvalidSender{}));
        }

        // Clear approvals
        self.token_approvals.setter(token_id).set(Address::ZERO);

        // Update balances with underflow protection
        let from_balance = self.balances.getter(from).get();
        if from_balance == U256::ZERO {
            return Err(NFTError::ERC721InvalidSender(ERC721InvalidSender{}));
        }
        self.balances.setter(from).set(from_balance - U256::from(1));

        let to_balance = self.balances.getter(to).get();
        self.balances.setter(to).set(to_balance + U256::from(1));

        // Update ownership
        self.owners.setter(token_id).set(to);

        log(self.vm(), Transfer {
            from: from,
            to: to,
            tokenId: token_id,
        });

        Ok(())
    }

    /// Mint a new NFT
    pub fn mint(&mut self, to: Address, token_uri: String) -> Result<U256, NFTError> {
        if !self.initialized.get() {
            return Err(NFTError::AlreadyInitialized(AlreadyInitialized{}));
        }

        if token_uri.is_empty() {
            return Err(NFTError::InvalidTokenURI(InvalidTokenURI{}));
        }

        if to == Address::ZERO {
            return Err(NFTError::ERC721InvalidReceiver(ERC721InvalidReceiver{}));
        }

        let token_id = self.next_token_id.get();

        // Set token data
        self.token_uris.setter(token_id).set_str(token_uri.clone());
        self.owners.setter(token_id).set(to);

        // Update balance
        let total_balance = self.balances.getter(to).get();
        self.balances.setter(to).set(total_balance + U256::from(1));

        // Increment token ID
        self.next_token_id.set(token_id + U256::from(1));

        // Emit events
        log(self.vm(), Transfer {
            from: Address::ZERO,
            to: to,
            tokenId: token_id,
        });

        log(self.vm(), NFTMinted {
            tokenId: token_id,
            to: to,
            tokenURI: token_uri,
        });

        Ok(token_id)
    }

}