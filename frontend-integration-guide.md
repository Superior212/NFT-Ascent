# Frontend Integration Guide for Neon NFT Marketplace

This guide will help you integrate a frontend application with the Neon NFT Marketplace built on Arbitrum Stylus.

## 📋 Prerequisites

- Node.js (v16 or higher)
- npm or yarn
- Web3 wallet (MetaMask, WalletConnect, etc.)
- Basic knowledge of React/Next.js and Web3

## 🛠️ Setup

### 1. Install Dependencies

```bash
npm install ethers @ethersproject/providers @ethersproject/contracts
# or
yarn add ethers @ethersproject/providers @ethersproject/contracts
```

### 2. Contract Addresses and ABIs

After deploying your contracts, you'll need:

- NFT Contract Address
- Marketplace Contract Address
- Contract ABIs (provided in the project)

## 🔧 Configuration

Create a configuration file for your contracts:

```javascript
// config/contracts.js
export const CONTRACTS = {
  // Replace with your deployed contract addresses
  NFT_CONTRACT: "0x...", // Your deployed NFT contract address
  MARKETPLACE_CONTRACT: "0x...", // Your deployed marketplace contract address

  // Network configuration
  NETWORK: {
    chainId: 421614, // Arbitrum Sepolia testnet
    name: "Arbitrum Sepolia",
    rpcUrl: "https://sepolia-rollup.arbitrum.io/rpc",
    blockExplorer: "https://sepolia.arbiscan.io",
  },
};

export const NFT_ABI = [
  // ... (use the ABI from neon-nft-abi.json)
];

export const MARKETPLACE_ABI = [
  // ... (use the ABI from neon-marketplace-abi.json)
];
```

## 🎯 Core Integration Examples

### 1. Contract Initialization

```javascript
// utils/contracts.js
import { ethers } from "ethers";
import { CONTRACTS, NFT_ABI, MARKETPLACE_ABI } from "../config/contracts";

export class ContractManager {
  constructor(provider, signer) {
    this.provider = provider;
    this.signer = signer;

    // Initialize contracts
    this.nftContract = new ethers.Contract(
      CONTRACTS.NFT_CONTRACT,
      NFT_ABI,
      signer
    );

    this.marketplaceContract = new ethers.Contract(
      CONTRACTS.MARKETPLACE_CONTRACT,
      MARKETPLACE_ABI,
      signer
    );
  }

  // Get read-only contracts for view functions
  getReadOnlyContracts() {
    return {
      nft: new ethers.Contract(CONTRACTS.NFT_CONTRACT, NFT_ABI, this.provider),
      marketplace: new ethers.Contract(
        CONTRACTS.MARKETPLACE_CONTRACT,
        MARKETPLACE_ABI,
        this.provider
      ),
    };
  }
}
```

### 2. NFT Operations

```javascript
// hooks/useNFT.js
import { useState, useEffect } from "react";
import { ContractManager } from "../utils/contracts";

export const useNFT = (provider, signer) => {
  const [contractManager, setContractManager] = useState(null);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    if (provider && signer) {
      setContractManager(new ContractManager(provider, signer));
    }
  }, [provider, signer]);

  const mintNFT = async (tokenURI) => {
    if (!contractManager) throw new Error("Contract not initialized");

    setLoading(true);
    try {
      const tx = await contractManager.nftContract.mintNft(tokenURI);
      const receipt = await tx.wait();

      // Extract token ID from events
      const mintEvent = receipt.events.find((e) => e.event === "NFTMinted");
      const tokenId = mintEvent.args.tokenId;

      return { success: true, tokenId, txHash: receipt.transactionHash };
    } catch (error) {
      console.error("Mint NFT error:", error);
      return { success: false, error: error.message };
    } finally {
      setLoading(false);
    }
  };

  const approveMarketplace = async (tokenId) => {
    if (!contractManager) throw new Error("Contract not initialized");

    setLoading(true);
    try {
      const tx = await contractManager.nftContract.approve(
        CONTRACTS.MARKETPLACE_CONTRACT,
        tokenId
      );
      await tx.wait();
      return { success: true, txHash: tx.hash };
    } catch (error) {
      console.error("Approve error:", error);
      return { success: false, error: error.message };
    } finally {
      setLoading(false);
    }
  };

  const getTokenURI = async (tokenId) => {
    if (!contractManager) return null;

    try {
      const readOnly = contractManager.getReadOnlyContracts();
      return await readOnly.nft.tokenURI(tokenId);
    } catch (error) {
      console.error("Get token URI error:", error);
      return null;
    }
  };

  const getOwnerNFTs = async (ownerAddress) => {
    if (!contractManager) return [];

    try {
      const readOnly = contractManager.getReadOnlyContracts();
      const balance = await readOnly.nft.balanceOf(ownerAddress);

      // Note: This is a simplified approach. In production, you'd want to
      // track minted tokens or use events to build a complete list
      const nfts = [];
      for (let i = 1; i <= balance.toNumber(); i++) {
        try {
          const owner = await readOnly.nft.ownerOf(i);
          if (owner.toLowerCase() === ownerAddress.toLowerCase()) {
            const tokenURI = await readOnly.nft.tokenURI(i);
            nfts.push({ tokenId: i, tokenURI, owner });
          }
        } catch (e) {
          // Token doesn't exist or other error
          continue;
        }
      }

      return nfts;
    } catch (error) {
      console.error("Get owner NFTs error:", error);
      return [];
    }
  };

  return {
    mintNFT,
    approveMarketplace,
    getTokenURI,
    getOwnerNFTs,
    loading,
  };
};
```

### 3. Marketplace Operations

```javascript
// hooks/useMarketplace.js
import { useState, useEffect } from "react";
import { ethers } from "ethers";
import { ContractManager } from "../utils/contracts";

export const useMarketplace = (provider, signer) => {
  const [contractManager, setContractManager] = useState(null);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    if (provider && signer) {
      setContractManager(new ContractManager(provider, signer));
    }
  }, [provider, signer]);

  const createAuction = async (
    nftContract,
    tokenId,
    reservePrice,
    duration
  ) => {
    if (!contractManager) throw new Error("Contract not initialized");

    setLoading(true);
    try {
      const tx = await contractManager.marketplaceContract.createAuction(
        nftContract,
        tokenId,
        ethers.utils.parseEther(reservePrice.toString()),
        duration
      );
      const receipt = await tx.wait();

      // Extract auction ID from events
      const auctionEvent = receipt.events.find(
        (e) => e.event === "AuctionCreated"
      );
      const auctionId = auctionEvent.args.auctionId;

      return { success: true, auctionId, txHash: receipt.transactionHash };
    } catch (error) {
      console.error("Create auction error:", error);
      return { success: false, error: error.message };
    } finally {
      setLoading(false);
    }
  };

  const placeBid = async (auctionId, bidAmount) => {
    if (!contractManager) throw new Error("Contract not initialized");

    setLoading(true);
    try {
      const tx = await contractManager.marketplaceContract.placeBid(auctionId, {
        value: ethers.utils.parseEther(bidAmount.toString()),
      });
      await tx.wait();
      return { success: true, txHash: tx.hash };
    } catch (error) {
      console.error("Place bid error:", error);
      return { success: false, error: error.message };
    } finally {
      setLoading(false);
    }
  };

  const settleAuction = async (auctionId) => {
    if (!contractManager) throw new Error("Contract not initialized");

    setLoading(true);
    try {
      const tx = await contractManager.marketplaceContract.settleAuction(
        auctionId
      );
      await tx.wait();
      return { success: true, txHash: tx.hash };
    } catch (error) {
      console.error("Settle auction error:", error);
      return { success: false, error: error.message };
    } finally {
      setLoading(false);
    }
  };

  const cancelAuction = async (auctionId) => {
    if (!contractManager) throw new Error("Contract not initialized");

    setLoading(true);
    try {
      const tx = await contractManager.marketplaceContract.cancelAuction(
        auctionId
      );
      await tx.wait();
      return { success: true, txHash: tx.hash };
    } catch (error) {
      console.error("Cancel auction error:", error);
      return { success: false, error: error.message };
    } finally {
      setLoading(false);
    }
  };

  const withdraw = async () => {
    if (!contractManager) throw new Error("Contract not initialized");

    setLoading(true);
    try {
      const tx = await contractManager.marketplaceContract.withdraw();
      await tx.wait();
      return { success: true, txHash: tx.hash };
    } catch (error) {
      console.error("Withdraw error:", error);
      return { success: false, error: error.message };
    } finally {
      setLoading(false);
    }
  };

  const getAuction = async (auctionId) => {
    if (!contractManager) return null;

    try {
      const readOnly = contractManager.getReadOnlyContracts();
      const auction = await readOnly.marketplace.getAuction(auctionId);

      return {
        nftContract: auction.nftContract,
        tokenId: auction.tokenId.toString(),
        seller: auction.seller,
        reservePrice: ethers.utils.formatEther(auction.reservePrice),
        currentBid: ethers.utils.formatEther(auction.currentBid),
        currentBidder: auction.currentBidder,
        endTime: auction.endTime.toString(),
        settled: auction.settled,
      };
    } catch (error) {
      console.error("Get auction error:", error);
      return null;
    }
  };

  const getActiveAuctions = async () => {
    if (!contractManager) return [];

    try {
      const readOnly = contractManager.getReadOnlyContracts();
      const nextAuctionId = await readOnly.marketplace.getNextAuctionId();

      const auctions = [];
      for (let i = 1; i < nextAuctionId.toNumber(); i++) {
        try {
          const auction = await readOnly.marketplace.getAuction(i);
          const isActive = await readOnly.marketplace.isAuctionActive(i);

          if (isActive) {
            auctions.push({
              auctionId: i,
              nftContract: auction.nftContract,
              tokenId: auction.tokenId.toString(),
              seller: auction.seller,
              reservePrice: ethers.utils.formatEther(auction.reservePrice),
              currentBid: ethers.utils.formatEther(auction.currentBid),
              currentBidder: auction.currentBidder,
              endTime: auction.endTime.toString(),
              settled: auction.settled,
            });
          }
        } catch (e) {
          // Auction doesn't exist or other error
          continue;
        }
      }

      return auctions;
    } catch (error) {
      console.error("Get active auctions error:", error);
      return [];
    }
  };

  const getBalance = async (userAddress) => {
    if (!contractManager) return "0";

    try {
      const readOnly = contractManager.getReadOnlyContracts();
      const balance = await readOnly.marketplace.getBalance(userAddress);
      return ethers.utils.formatEther(balance);
    } catch (error) {
      console.error("Get balance error:", error);
      return "0";
    }
  };

  return {
    createAuction,
    placeBid,
    settleAuction,
    cancelAuction,
    withdraw,
    getAuction,
    getActiveAuctions,
    getBalance,
    loading,
  };
};
```

### 4. React Components Example

```jsx
// components/Marketplace.jsx
import React, { useState, useEffect } from "react";
import { useMarketplace } from "../hooks/useMarketplace";
import { useNFT } from "../hooks/useNFT";

const Marketplace = ({ provider, signer, userAddress }) => {
  const {
    getActiveAuctions,
    placeBid,
    loading: marketplaceLoading,
  } = useMarketplace(provider, signer);
  const {
    getOwnerNFTs,
    approveMarketplace,
    loading: nftLoading,
  } = useNFT(provider, signer);

  const [auctions, setAuctions] = useState([]);
  const [userNFTs, setUserNFTs] = useState([]);
  const [bidAmounts, setBidAmounts] = useState({});

  useEffect(() => {
    loadData();
  }, [provider, signer]);

  const loadData = async () => {
    const [auctionsData, nftsData] = await Promise.all([
      getActiveAuctions(),
      getOwnerNFTs(userAddress),
    ]);
    setAuctions(auctionsData);
    setUserNFTs(nftsData);
  };

  const handleBid = async (auctionId, amount) => {
    const result = await placeBid(auctionId, amount);
    if (result.success) {
      alert("Bid placed successfully!");
      loadData(); // Refresh data
    } else {
      alert(`Bid failed: ${result.error}`);
    }
  };

  const handleApprove = async (tokenId) => {
    const result = await approveMarketplace(tokenId);
    if (result.success) {
      alert("NFT approved for marketplace!");
    } else {
      alert(`Approval failed: ${result.error}`);
    }
  };

  return (
    <div className="marketplace">
      <h2>Active Auctions</h2>
      <div className="auctions-grid">
        {auctions.map((auction) => (
          <div key={auction.auctionId} className="auction-card">
            <h3>Token #{auction.tokenId}</h3>
            <p>Reserve Price: {auction.reservePrice} ETH</p>
            <p>Current Bid: {auction.currentBid} ETH</p>
            <p>Ends: {new Date(auction.endTime * 1000).toLocaleString()}</p>

            <div className="bid-section">
              <input
                type="number"
                placeholder="Bid amount (ETH)"
                value={bidAmounts[auction.auctionId] || ""}
                onChange={(e) =>
                  setBidAmounts({
                    ...bidAmounts,
                    [auction.auctionId]: e.target.value,
                  })
                }
              />
              <button
                onClick={() =>
                  handleBid(auction.auctionId, bidAmounts[auction.auctionId])
                }
                disabled={marketplaceLoading}>
                Place Bid
              </button>
            </div>
          </div>
        ))}
      </div>

      <h2>Your NFTs</h2>
      <div className="nfts-grid">
        {userNFTs.map((nft) => (
          <div key={nft.tokenId} className="nft-card">
            <h3>Token #{nft.tokenId}</h3>
            <p>URI: {nft.tokenURI}</p>
            <button
              onClick={() => handleApprove(nft.tokenId)}
              disabled={nftLoading}>
              Approve for Marketplace
            </button>
          </div>
        ))}
      </div>
    </div>
  );
};

export default Marketplace;
```

## 🚀 Deployment Steps

### 1. Deploy Contracts

```bash
# Deploy NFT contract
cd neon-nft
cargo stylus deploy --endpoint https://stylus-testnet.arbitrum.io/rpc --private-key YOUR_PRIVATE_KEY

# Deploy Marketplace contract
cd ../neon-marketplace
cargo stylus deploy --endpoint https://stylus-testnet.arbitrum.io/rpc --private-key YOUR_PRIVATE_KEY
```

### 2. Initialize Contracts

```javascript
// After deployment, initialize both contracts
const nftContract = new ethers.Contract(nftAddress, NFT_ABI, signer);
const marketplaceContract = new ethers.Contract(
  marketplaceAddress,
  MARKETPLACE_ABI,
  signer
);

// Initialize NFT contract
await nftContract.initialize();

// Initialize Marketplace contract
await marketplaceContract.initialize();
```

### 3. Update Frontend Configuration

Update your `config/contracts.js` with the deployed addresses:

```javascript
export const CONTRACTS = {
  NFT_CONTRACT: "0xYourDeployedNFTAddress",
  MARKETPLACE_CONTRACT: "0xYourDeployedMarketplaceAddress",
  // ... rest of config
};
```

## 🔍 Event Listening

```javascript
// Listen to marketplace events
const marketplaceContract = new ethers.Contract(address, ABI, provider);

marketplaceContract.on(
  "AuctionCreated",
  (auctionId, nftContract, tokenId, reservePrice, endTime) => {
    console.log("New auction created:", { auctionId, tokenId, reservePrice });
    // Update your UI
  }
);

marketplaceContract.on("BidPlaced", (auctionId, bidder, amount) => {
  console.log("New bid placed:", { auctionId, bidder, amount });
  // Update auction display
});

marketplaceContract.on("AuctionSettled", (auctionId, winner, amount) => {
  console.log("Auction settled:", { auctionId, winner, amount });
  // Remove from active auctions
});
```

## 🛡️ Error Handling

```javascript
// Common error handling patterns
const handleContractError = (error) => {
  if (error.code === "INSUFFICIENT_FUNDS") {
    return "Insufficient funds for transaction";
  } else if (error.code === "USER_REJECTED") {
    return "Transaction rejected by user";
  } else if (error.message.includes("execution reverted")) {
    return "Transaction failed: " + error.message;
  } else {
    return "Unknown error: " + error.message;
  }
};
```

## 📱 Mobile Integration

For mobile apps, consider using:

- **WalletConnect** for wallet connections
- **React Native** with ethers.js
- **Expo** for cross-platform development

## 🔧 Testing

```javascript
// Test contract interactions
const testContractIntegration = async () => {
  try {
    // Test minting
    const mintResult = await mintNFT("https://example.com/metadata.json");
    console.log("Mint result:", mintResult);

    // Test auction creation
    const auctionResult = await createAuction(
      CONTRACTS.NFT_CONTRACT,
      mintResult.tokenId,
      "1.0", // 1 ETH reserve
      86400 // 24 hours
    );
    console.log("Auction result:", auctionResult);
  } catch (error) {
    console.error("Test failed:", error);
  }
};
```

## 📚 Additional Resources

- [Arbitrum Stylus Documentation](https://docs.arbitrum.io/stylus)
- [Ethers.js Documentation](https://docs.ethers.io/)
- [Web3 React Hooks](https://github.com/NoahZinsmeister/web3-react)

This guide provides a solid foundation for integrating your frontend with the Neon NFT Marketplace. Remember to handle errors gracefully, provide good user feedback, and test thoroughly before deploying to production.
