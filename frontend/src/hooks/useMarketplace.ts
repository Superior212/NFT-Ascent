import { useState, useEffect } from "react";
import { ethers } from "ethers";
import { CONTRACTS, MARKETPLACE_ABI } from "@/config/contracts";
import { parseEther, formatEther } from "@/lib/utils";

interface MarketplaceContract {
  contract: ethers.Contract | null;
  readOnlyContract: ethers.Contract | null;
}

export const useMarketplace = (
  provider: ethers.BrowserProvider | null,
  signer: ethers.JsonRpcSigner | null
) => {
  const [contracts, setContracts] = useState<MarketplaceContract>({
    contract: null,
    readOnlyContract: null,
  });
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (provider && signer) {
      const contract = new ethers.Contract(
        CONTRACTS.MARKETPLACE_CONTRACT,
        MARKETPLACE_ABI,
        signer
      );
      const readOnlyContract = new ethers.Contract(
        CONTRACTS.MARKETPLACE_CONTRACT,
        MARKETPLACE_ABI,
        provider
      );

      setContracts({ contract, readOnlyContract });
    }
  }, [provider, signer]);

  const createAuction = async (
    nftContract: string,
    tokenId: string,
    reservePrice: string,
    duration: number
  ) => {
    if (!contracts.contract) {
      throw new Error("Contract not initialized");
    }

    setLoading(true);
    setError(null);

    try {
      const tx = await contracts.contract.createAuction(
        nftContract,
        tokenId,
        parseEther(reservePrice),
        duration
      );
      const receipt = await tx.wait();

      // Extract auction ID from events
      const auctionEvent = receipt.logs.find((log: any) => {
        try {
          const parsed = contracts.contract!.interface.parseLog(log);
          return parsed?.name === "AuctionCreated";
        } catch {
          return false;
        }
      });

      if (auctionEvent) {
        const parsed = contracts.contract!.interface.parseLog(auctionEvent);
        const auctionId = parsed?.args.auctionId.toString();
        return { success: true, auctionId, txHash: receipt.hash };
      }

      return { success: true, txHash: receipt.hash };
    } catch (error: any) {
      setError(error.message);
      return { success: false, error: error.message };
    } finally {
      setLoading(false);
    }
  };

  const placeBid = async (auctionId: string, bidAmount: string) => {
    if (!contracts.contract) {
      throw new Error("Contract not initialized");
    }

    setLoading(true);
    setError(null);

    try {
      const tx = await contracts.contract.placeBid(auctionId, {
        value: parseEther(bidAmount),
      });
      await tx.wait();
      return { success: true, txHash: tx.hash };
    } catch (error: any) {
      setError(error.message);
      return { success: false, error: error.message };
    } finally {
      setLoading(false);
    }
  };

  const settleAuction = async (auctionId: string) => {
    if (!contracts.contract) {
      throw new Error("Contract not initialized");
    }

    setLoading(true);
    setError(null);

    try {
      const tx = await contracts.contract.settleAuction(auctionId);
      await tx.wait();
      return { success: true, txHash: tx.hash };
    } catch (error: any) {
      setError(error.message);
      return { success: false, error: error.message };
    } finally {
      setLoading(false);
    }
  };

  const cancelAuction = async (auctionId: string) => {
    if (!contracts.contract) {
      throw new Error("Contract not initialized");
    }

    setLoading(true);
    setError(null);

    try {
      const tx = await contracts.contract.cancelAuction(auctionId);
      await tx.wait();
      return { success: true, txHash: tx.hash };
    } catch (error: any) {
      setError(error.message);
      return { success: false, error: error.message };
    } finally {
      setLoading(false);
    }
  };

  const withdraw = async () => {
    if (!contracts.contract) {
      throw new Error("Contract not initialized");
    }

    setLoading(true);
    setError(null);

    try {
      const tx = await contracts.contract.withdraw();
      await tx.wait();
      return { success: true, txHash: tx.hash };
    } catch (error: any) {
      setError(error.message);
      return { success: false, error: error.message };
    } finally {
      setLoading(false);
    }
  };

  const getAuction = async (auctionId: string) => {
    if (!contracts.readOnlyContract) return null;

    try {
      const auction = await contracts.readOnlyContract.getAuction(auctionId);

      return {
        nftContract: auction.nftContract,
        tokenId: auction.tokenId.toString(),
        seller: auction.seller,
        reservePrice: formatEther(auction.reservePrice),
        currentBid: formatEther(auction.currentBid),
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
    if (!contracts.readOnlyContract) return [];

    try {
      const nextAuctionId = await contracts.readOnlyContract.getNextAuctionId();

      const auctions = [];
      for (let i = 1; i < Number(nextAuctionId); i++) {
        try {
          const auction = await contracts.readOnlyContract!.getAuction(i);
          const isActive = await contracts.readOnlyContract!.isAuctionActive(i);

          if (isActive) {
            auctions.push({
              auctionId: i.toString(),
              nftContract: auction.nftContract,
              tokenId: auction.tokenId.toString(),
              seller: auction.seller,
              reservePrice: formatEther(auction.reservePrice),
              currentBid: formatEther(auction.currentBid),
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

  const getBalance = async (userAddress: string) => {
    if (!contracts.readOnlyContract) return "0";

    try {
      const balance = await contracts.readOnlyContract.getBalance(userAddress);
      return formatEther(balance);
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
    error,
  };
};
