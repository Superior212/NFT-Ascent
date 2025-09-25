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
  signer: ethers.JsonRpcSigner | null,
  userAddress: string | null
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
      if (!userAddress) {
        throw new Error("User address not available");
      }

      const tx = await contracts.contract.mint(userAddress, tokenURI);
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
    if (!contracts.readOnlyContract) {
      console.log("No read-only contract available for getTokenURI");
      return "URI not available";
    }

    try {
      console.log(`Getting token URI for token ${tokenId}...`);
      const uri = await contracts.readOnlyContract.tokenURI(tokenId);
      console.log(`Token ${tokenId} URI:`, uri);
      return uri;
    } catch (error) {
      console.error(`Get token URI error for token ${tokenId}:`, error);
      return "URI not available";
    }
  };

  const getOwnerNFTs = async (ownerAddress: string) => {
    if (!contracts.readOnlyContract) {
      console.log("No read-only contract available");
      return [];
    }

    try {
      console.log("Getting NFTs for address:", ownerAddress);
      const balance = await contracts.readOnlyContract.balanceOf(ownerAddress);
      const balanceNum = Number(balance);
      console.log("User balance:", balanceNum);

      if (balanceNum === 0) {
        console.log("User has no NFTs");
        return [];
      }

      // Since we don't have next_token_id function, we'll search through a reasonable range
      // Start with a smaller range and expand if needed
      let maxSearch = 20; // Start with smaller range
      console.log("Searching for NFTs in range 1 to", maxSearch);

      // Search through the range
      const nfts = [];
      let foundCount = 0;

      for (let i = 1; i <= maxSearch; i++) {
        try {
          const owner = await contracts.readOnlyContract!.ownerOf(i);
          console.log(`Token ${i} owner:`, owner);

          if (owner.toLowerCase() === ownerAddress.toLowerCase()) {
            try {
              const tokenURI = await contracts.readOnlyContract!.tokenURI(i);
              console.log(`Found NFT ${i} with URI:`, tokenURI);
              nfts.push({ tokenId: i.toString(), tokenURI, owner });
            } catch (uriError) {
              console.log(`Found NFT ${i} but tokenURI failed:`, uriError);
              // Still add the NFT even if URI fails
              nfts.push({
                tokenId: i.toString(),
                tokenURI: "URI not available",
                owner,
              });
            }
            foundCount++;

            // Stop if we found all the user's NFTs
            if (foundCount >= balanceNum) {
              console.log("Found all user's NFTs");
              break;
            }
          }
        } catch (e) {
          // Token doesn't exist or other error
          console.log(`Token ${i} doesn't exist or error:`, e);
          continue;
        }
      }

      // If we didn't find all NFTs, expand the search range
      if (foundCount < balanceNum && maxSearch < 100) {
        console.log(
          `Found ${foundCount} of ${balanceNum} NFTs, expanding search...`
        );
        maxSearch = 100;

        for (let i = 21; i <= maxSearch; i++) {
          try {
            const owner = await contracts.readOnlyContract!.ownerOf(i);
            if (owner.toLowerCase() === ownerAddress.toLowerCase()) {
              try {
                const tokenURI = await contracts.readOnlyContract!.tokenURI(i);
                console.log(`Found additional NFT ${i} with URI:`, tokenURI);
                nfts.push({ tokenId: i.toString(), tokenURI, owner });
              } catch (uriError) {
                console.log(
                  `Found additional NFT ${i} but tokenURI failed:`,
                  uriError
                );
                // Still add the NFT even if URI fails
                nfts.push({
                  tokenId: i.toString(),
                  tokenURI: "URI not available",
                  owner,
                });
              }
              foundCount++;

              if (foundCount >= balanceNum) {
                break;
              }
            }
          } catch (e) {
            continue;
          }
        }
      }

      // Final check
      if (foundCount < balanceNum) {
        console.warn(
          `Expected ${balanceNum} NFTs but only found ${foundCount}`
        );
      }

      console.log("Final NFTs found:", nfts);
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

  const initializeNFT = async (name: string, symbol: string) => {
    if (!contracts.contract) {
      throw new Error("Contract not initialized");
    }

    setLoading(true);
    setError(null);

    try {
      const tx = await contracts.contract.initialize(name, symbol);
      await tx.wait();
      return { success: true, txHash: tx.hash };
    } catch (error: any) {
      setError(error.message);
      return { success: false, error: error.message };
    } finally {
      setLoading(false);
    }
  };

  return {
    initializeNFT,
    mintNFT,
    approveMarketplace,
    getTokenURI,
    getOwnerNFTs,
    getApprovalStatus,
    loading,
    error,
  };
};
