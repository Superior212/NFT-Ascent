import { useState, useEffect } from "react";
import { ethers } from "ethers";
import { CONTRACTS, NFT_ABI } from "@/config/contracts";
// import { parseEther } from '@/lib/utils';

interface NFTContract {
  contract: ethers.Contract | null;
  readOnlyContract: ethers.Contract | null;
}

export const useNFT = (
  provider: ethers.BrowserProvider | null,
  signer: ethers.JsonRpcSigner | null
) => {
  const [contracts, setContracts] = useState<NFTContract>({
    contract: null,
    readOnlyContract: null,
  });
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (provider && signer) {
      const contract = new ethers.Contract(
        CONTRACTS.NFT_CONTRACT,
        NFT_ABI,
        signer
      );
      const readOnlyContract = new ethers.Contract(
        CONTRACTS.NFT_CONTRACT,
        NFT_ABI,
        provider
      );

      setContracts({ contract, readOnlyContract });
    }
  }, [provider, signer]);

  const mintNFT = async (tokenURI: string) => {
    if (!contracts.contract) {
      throw new Error("Contract not initialized");
    }

    setLoading(true);
    setError(null);

    try {
      const tx = await contracts.contract.mintNft(tokenURI);
      const receipt = await tx.wait();

      // Extract token ID from events
      const mintEvent = receipt.logs.find((log: any) => {
        try {
          const parsed = contracts.contract!.interface.parseLog(log);
          return parsed?.name === "NFTMinted";
        } catch {
          return false;
        }
      });

      if (mintEvent) {
        const parsed = contracts.contract!.interface.parseLog(mintEvent);
        const tokenId = parsed?.args.tokenId.toString();
        return { success: true, tokenId, txHash: receipt.hash };
      }

      return { success: true, txHash: receipt.hash };
    } catch (error: any) {
      setError(error.message);
      return { success: false, error: error.message };
    } finally {
      setLoading(false);
    }
  };

  const approveMarketplace = async (tokenId: string) => {
    if (!contracts.contract) {
      throw new Error("Contract not initialized");
    }

    setLoading(true);
    setError(null);

    try {
      const tx = await contracts.contract.approve(
        CONTRACTS.MARKETPLACE_CONTRACT,
        tokenId
      );
      await tx.wait();
      return { success: true, txHash: tx.hash };
    } catch (error: any) {
      setError(error.message);
      return { success: false, error: error.message };
    } finally {
      setLoading(false);
    }
  };

  const getTokenURI = async (tokenId: string) => {
    if (!contracts.readOnlyContract) return null;

    try {
      return await contracts.readOnlyContract.tokenURI(tokenId);
    } catch (error) {
      console.error("Get token URI error:", error);
      return null;
    }
  };

  const getOwnerNFTs = async (ownerAddress: string) => {
    if (!contracts.readOnlyContract) return [];

    try {
      await contracts.readOnlyContract.balanceOf(ownerAddress);
      const nextTokenId = await contracts.readOnlyContract.getNextTokenId();

      const nfts = [];
      for (let i = 1; i < Number(nextTokenId); i++) {
        try {
          const owner = await contracts.readOnlyContract!.ownerOf(i);
          if (owner.toLowerCase() === ownerAddress.toLowerCase()) {
            const tokenURI = await contracts.readOnlyContract!.tokenURI(i);
            nfts.push({ tokenId: i.toString(), tokenURI, owner });
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

  const getApprovalStatus = async (tokenId: string) => {
    if (!contracts.readOnlyContract) return false;

    try {
      const approved = await contracts.readOnlyContract.getApproved(tokenId);
      return (
        approved.toLowerCase() === CONTRACTS.MARKETPLACE_CONTRACT.toLowerCase()
      );
    } catch (error) {
      console.error("Get approval status error:", error);
      return false;
    }
  };

  return {
    mintNFT,
    approveMarketplace,
    getTokenURI,
    getOwnerNFTs,
    getApprovalStatus,
    loading,
    error,
  };
};
